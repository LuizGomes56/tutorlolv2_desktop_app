//! Native-only sharing control. Live data has no queue or disk spool.
use crate::delivery::Delivery;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tutorlolv2_protocol::{
    Id16, RawSnapshot, RecordingState, RecordingStatus, ShareMode, ShareState,
};

pub const fn private_share() -> ShareState {
    ShareState {
        mode: ShareMode::Private,
        code: None,
        url: None,
    }
}

pub fn authorize_share_window(label: &str) -> Result<(), String> {
    if label == "main" {
        Ok(())
    } else {
        Err("Sharing can only be changed from the main window".into())
    }
}

impl Delivery {
    pub fn recording_state(&self) -> RecordingState {
        let mut state = self.recorder.lock().unwrap().recording_state();

        state.upload.pending_chunks = self.spool.pending().len().min(u32::MAX as usize) as u32;
        state.upload.last_error = self
            .sharing_refresh_error
            .lock()
            .unwrap()
            .clone()
            .or_else(|| self.sharing_error.lock().unwrap().clone())
            .or_else(|| self.live_error.lock().unwrap().clone())
            .or_else(|| self.control_error.lock().unwrap().clone())
            .or_else(|| self.delivery_error.lock().unwrap().clone());

        state
    }

    pub async fn set_share_mode(&self, mode: ShareMode) -> Result<ShareState, String> {
        let _guard = self.sharing_lock.lock().await;
        let state = self.recording_state();
        let id = state
            .recording_id
            .ok_or("The recording is still starting or no game is active")?;

        if mode == ShareMode::Public && (state.is_custom != Some(false) || state.is_practice) {
            return Err(
                "Public sharing is unavailable for custom, practice or unverified games".into(),
            );
        }

        let api = self.api.read().await.clone();
        let result = api.set_share_mode(&id, mode).await;

        match result {
            Ok(share) => {
                if !self.recorder.lock().unwrap().set_share(id, share.clone()) {
                    return Err("The active recording changed".into());
                }

                *self.sharing_error.lock().unwrap() = None;
                *self.sharing_refresh_error.lock().unwrap() = None;

                if share.mode == ShareMode::Private {
                    *self.live_error.lock().unwrap() = None;
                }

                self.sharing_now.notify_one();

                Ok(share)
            }
            Err(error) => {
                *self.sharing_error.lock().unwrap() = Some(error.clone());
                self.sharing_now.notify_one();

                Err(error)
            }
        }
    }

    /// Wake observers after recorder changes. A new snapshot does not invalidate an
    /// upload already in flight; the next pass picks up the newest snapshot.
    pub fn recording_changed(&self) {
        self.sharing_now.notify_one();
    }

    fn live_share_is_current(&self, id: &Id16, share: &ShareState) -> bool {
        if self.sharing_refresh_error.lock().unwrap().is_some() {
            return false;
        }

        let recorder = self.recorder.lock().unwrap();
        let state = recorder.recording_state();

        state.recording_id.as_ref() == Some(id)
            && state.share == *share
            && share.mode != ShareMode::Private
            && share.code.is_some()
            && recorder.live_snapshot().is_some()
    }

    /// Restore authoritative sharing without claiming revocation on a failed lookup.
    pub async fn refresh_share(&self, id: &Id16) -> Result<(), String> {
        let _guard = self.sharing_lock.lock().await;
        let api = self.api.read().await.clone();

        match api.share(id).await {
            Ok(share) => {
                if share.mode == ShareMode::Private {
                    *self.live_error.lock().unwrap() = None;
                }

                self.recorder.lock().unwrap().set_share(*id, share);
                *self.sharing_refresh_error.lock().unwrap() = None;

                Ok(())
            }
            Err(error) => {
                *self.sharing_refresh_error.lock().unwrap() = Some(format!(
                    "Sharing state unverified; the existing link may still be accessible: {error}"
                ));
                Err(error)
            }
        }
    }

    /// Publish one snapshot, retaining the request through snapshot-only ticks.
    /// Returns false when the active game or capability changed while waiting.
    pub async fn publish_live(
        &self,
        id: &Id16,
        share: &ShareState,
        snapshot: &RawSnapshot,
    ) -> Result<bool, String> {
        if !self.live_share_is_current(id, share) {
            return Ok(false);
        }

        let api = self.api.read().await.clone();
        let upload = api.upload_live(id, snapshot);
        tokio::pin!(upload);

        loop {
            tokio::select! {
                result = &mut upload => {
                    if let Err(error) = &result
                        && ["403", "404", "409", "410"]
                            .iter()
                            .any(|status| error.starts_with(status))
                    {
                        // Server lifecycle is authoritative. A GET can clear an expired
                        // Live code, but never creates a replacement capability.
                        let _ = self.refresh_share(id).await;
                        self.recording_changed();
                    }

                    *self.live_error.lock().unwrap() = result.as_ref().err().cloned();
                    return result.map(|()| true);
                }
                _ = self.sharing_now.notified() => {
                    if !self.live_share_is_current(id, share) {
                        return Ok(false);
                    }
                }
            }
        }
    }

    pub(crate) async fn sharing_loop(self: Arc<Self>, app: AppHandle) {
        let mut restored_id = None;
        let mut sent = None;
        let mut previous = None::<RecordingState>;

        loop {
            let mut state = self.recording_state();

            if let Some(id) = state.recording_id {
                let resumed = previous
                    .as_ref()
                    .is_some_and(|old| old.status == RecordingStatus::Suspended)
                    && state.status != RecordingStatus::Suspended;

                if restored_id != Some(id) || resumed {
                    match self.refresh_share(&id).await {
                        Ok(()) => restored_id = Some(id),
                        Err(_) => restored_id = None,
                    }

                    state = self.recording_state();
                }
            } else {
                restored_id = None;
                sent = None;

                *self.sharing_refresh_error.lock().unwrap() = None;
                *self.live_error.lock().unwrap() = None;
            }

            if previous.as_ref() != Some(&state) {
                let _ = app.emit("recording-state-changed", &state);

                if previous.as_ref().map(|s| &s.share) != Some(&state.share) {
                    let _ = app.emit("share-state-changed", &state.share);
                }

                previous = Some(state.clone());
            }

            let (live_state, snapshot) = {
                let recorder = self.recorder.lock().unwrap();
                (recorder.recording_state(), recorder.live_snapshot())
            };

            if let (Some(id), Some(snapshot)) = (live_state.recording_id, snapshot) {
                let key = (id, snapshot.captured_at_ms, live_state.share.code.clone());

                if restored_id == Some(id)
                    && live_state.share.mode != ShareMode::Private
                    && live_state.share.code.is_some()
                    && sent.as_ref() != Some(&key)
                {
                    match self.publish_live(&id, &live_state.share, &snapshot).await {
                        Ok(true) => {
                            sent = Some(key);
                            continue;
                        }
                        Ok(false) => continue,
                        Err(_) => restored_id = None,
                    }
                }
            }

            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(1),
                self.sharing_now.notified(),
            )
            .await;
        }
    }
}
