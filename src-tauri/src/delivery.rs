//! Persistent delivery of recording control calls and chunks. No Tauri dependency.
use crate::{
    recorder::{Effect, Recorder},
    spool::{Spool, Target},
    store::Store,
    uploader::{ApiClient, ChunkOutcome, backoff_ms, body_sha, renumber},
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Notify, RwLock};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const OUTBOX_EVERY: Duration = Duration::from_secs(30);
const UPLOAD_EVERY: Duration = Duration::from_secs(60);

pub struct Delivery {
    pub(crate) recorder: Mutex<Recorder>,
    pub(crate) api: RwLock<ApiClient>,
    pub(crate) spool: Spool,
    pub(crate) store: Store,
    pub(crate) upload_now: Notify,
    pub(crate) outbox_now: Notify,
    pub(crate) register_now: Notify,
    pub(crate) sharing_now: Notify,
    pub(crate) sharing_error: Mutex<Option<String>>,
    pub(crate) sharing_refresh_error: Mutex<Option<String>>,
    pub(crate) live_error: Mutex<Option<String>>,
    pub(crate) control_error: Mutex<Option<String>>,
    pub(crate) delivery_error: Mutex<Option<String>>,
    pub(crate) sharing_lock: tokio::sync::Mutex<()>,
    upload_lock: tokio::sync::Mutex<Option<Instant>>,
    outbox_lock: tokio::sync::Mutex<()>,
}

impl Delivery {
    pub fn new(recorder: Recorder, api: ApiClient, spool: Spool, store: Store) -> Self {
        Self {
            recorder: Mutex::new(recorder),
            api: RwLock::new(api),
            spool,
            store,
            upload_now: Notify::new(),
            outbox_now: Notify::new(),
            register_now: Notify::new(),
            sharing_now: Notify::new(),
            sharing_error: Mutex::new(None),
            sharing_refresh_error: Mutex::new(None),
            live_error: Mutex::new(None),
            control_error: Mutex::new(None),
            delivery_error: Mutex::new(None),
            sharing_lock: tokio::sync::Mutex::new(()),
            upload_lock: tokio::sync::Mutex::new(None),
            outbox_lock: tokio::sync::Mutex::new(()),
        }
    }

    /// Run the uploader, flushing existing data immediately on startup.
    pub async fn run_uploads(self: Arc<Self>) {
        self.upload_loop().await;
    }

    /// One delivery pass. Failures remain persisted for the next pass/restart.
    pub async fn flush(self: &Arc<Self>) -> bool {
        let controls = self.send_outbox().await;
        let chunks = self.upload_pending().await;

        controls && chunks.is_none()
    }

    fn persist_effects(&self, effects: Vec<Effect>) {
        for effect in effects {
            if let Effect::PersistSession(session) = effect
                && let Err(e) = self.store.save_session(session.as_ref())
            {
                log::error!("Could not save session.bin: {e}");
            }
        }
    }

    /// Keeps the device registered: at startup without credentials, and again
    /// whenever the server stops accepting them (401). Recording works meanwhile:
    /// chunks and calls wait on disk.
    pub(crate) async fn registration(self: Arc<Self>) {
        loop {
            if !self.api.read().await.authenticated() {
                let mut attempt = 0;

                loop {
                    match ApiClient::new(None).register(APP_VERSION).await {
                        Ok(credentials) => {
                            if let Err(e) = self.store.save_device(&credentials) {
                                log::error!("Could not save device.bin: {e}");
                            }

                            *self.api.write().await = ApiClient::new(Some(&credentials));
                            self.outbox_now.notify_one();
                            self.upload_now.notify_one();

                            break;
                        }
                        Err(e) => {
                            log::warn!("Device registration failed: {e}");
                            tokio::time::sleep(Duration::from_millis(backoff_ms(
                                attempt,
                                jitter(),
                            )))
                            .await;

                            attempt += 1;
                        }
                    }
                }
            }

            self.register_now.notified().await;
        }
    }

    /// The server no longer knows these credentials: forget them and register again.
    async fn credentials_rejected(&self) {
        *self.api.write().await = ApiClient::new(None);
        self.register_now.notify_one();
    }

    /// Sends the queued session starts and ends (spec §7.2), replaying them after
    /// restarts.
    pub(crate) async fn outbox_loop(self: Arc<Self>) {
        let mut retries = 0;

        loop {
            let wait = if self.send_outbox().await {
                retries = 0;

                OUTBOX_EVERY
            } else {
                retries += 1;

                Duration::from_millis(backoff_ms(retries - 1, jitter()))
            };

            let _ = tokio::time::timeout(wait, self.outbox_now.notified()).await;
        }
    }

    /// Oldest session first, stopping at the first failure: a session's start must
    /// reach the server before any newer session's, or the older game would end the
    /// newer one as superseded. Returns whether everything was sent.
    async fn send_outbox(self: &Arc<Delivery>) -> bool {
        let _guard = self.outbox_lock.lock().await;
        let api = self.api.read().await.clone();

        if !api.authenticated() {
            return false;
        }

        let mut complete = true;

        for entry in self.spool.outbox() {
            let mut target = entry.target;

            if let Some(request) = &entry.start {
                let started = match target {
                    Some(target) => Ok(tutorlolv2_protocol::SessionStarted {
                        recording_id: target.recording_id,
                        resumed: true,
                        next_chunk_seq: target.seq_offset,
                        share: tutorlolv2_protocol::ShareState {
                            mode: tutorlolv2_protocol::ShareMode::Private,
                            code: None,
                            url: None,
                        },
                    }),
                    None => api.start_session(request).await,
                };

                match started {
                    Ok(started) => {
                        let proposed = Target {
                            recording_id: started.recording_id,
                            seq_offset: started.next_chunk_seq,
                        };

                        let assigned = match self.spool.assign_target(entry.local_id, proposed) {
                            Ok(target) => target,
                            Err(e) => {
                                log::error!("Could not save the spool target: {e}");
                                return false;
                            }
                        };

                        let started = tutorlolv2_protocol::SessionStarted {
                            next_chunk_seq: assigned.seq_offset,
                            ..started
                        };

                        let _ = self.spool.clear_start(entry.local_id);
                        let effects = self
                            .recorder
                            .lock()
                            .unwrap()
                            .on_session_started(entry.local_id, &started);

                        self.persist_effects(effects);
                        self.upload_now.notify_one();

                        target = Some(assigned);
                    }
                    Err(e) => {
                        *self.control_error.lock().unwrap() = Some(e.clone());
                        log::warn!("Start session failed: {e}");
                        if unauthorized(&e) {
                            self.credentials_rejected().await;
                        }
                        return false;
                    }
                }
            }

            if let (Some(request), Some(target)) = (&entry.end, target) {
                // The session's chunks go first, so the server's summary includes them
                self.upload_pending().await;

                match api.end(&target.recording_id, request).await {
                    Ok(()) => {
                        let _ = self.spool.clear_end(entry.local_id);
                        let effects = self.recorder.lock().unwrap().on_end_sent(entry.local_id);

                        self.persist_effects(effects);
                    }
                    Err(e) => {
                        *self.control_error.lock().unwrap() = Some(e.clone());
                        log::warn!("Ending the recording failed: {e}");

                        if unauthorized(&e) {
                            self.credentials_rejected().await;
                        }

                        return false;
                    }
                }
            }

            if let (Some(request), Some(target)) = (&entry.result, target) {
                match api.result(&target.recording_id, request).await {
                    Ok(()) => {
                        if let Err(e) = self.spool.clear_result(entry.local_id) {
                            log::error!("Could not acknowledge the game result: {e}");
                            return false;
                        }
                    }
                    Err(e) => {
                        *self.control_error.lock().unwrap() = Some(e.clone());
                        log::warn!("Sending the game result failed: {e}");

                        if unauthorized(&e) {
                            self.credentials_rejected().await;
                            return false;
                        }

                        complete = false;
                    }
                }
            }
        }

        if complete {
            *self.control_error.lock().unwrap() = None;
        }

        complete
    }

    pub(crate) async fn upload_loop(self: Arc<Delivery>) {
        let mut retries = 0;

        loop {
            match self.upload_pending().await {
                None => retries = 0,
                Some(after_ms) => {
                    let wait = after_ms.unwrap_or_else(|| backoff_ms(retries, jitter()));
                    retries += 1;

                    tokio::time::sleep(Duration::from_millis(wait)).await;
                    continue;
                }
            }

            let _ = tokio::time::timeout(UPLOAD_EVERY, self.upload_now.notified()).await;
        }
    }

    /// Uploads every spooled chunk whose recording is known, oldest first.
    /// `Some(retry_after_ms)` when the server asked to slow down (429) or was unreachable.
    pub(crate) async fn upload_pending(&self) -> Option<Option<u64>> {
        let mut cooldown = self.upload_lock.lock().await;

        if let Some(remaining) =
            cooldown.and_then(|until| until.checked_duration_since(Instant::now()))
        {
            return Some(Some(remaining.as_millis().max(1) as u64));
        }

        *cooldown = None;

        let api = self.api.read().await.clone();

        if !api.authenticated() {
            return None;
        }

        let mut retry = None;
        let mut pass_failed = false;

        for chunk in self.spool.pending() {
            let Some(target) = chunk.target else { continue };
            let Ok(body) = std::fs::read(&chunk.path) else {
                continue;
            };

            let Some(seq) = target
                .seq_offset
                .checked_add(chunk.local_seq)
                .filter(|s| *s <= i32::MAX as u32)
            else {
                log::error!("Chunk sequence exceeds the server range");
                let _ = self.spool.reject(&chunk);
                continue;
            };

            let prepared = if target.seq_offset == 0 {
                body_sha(&body).map(|sha| (body, sha))
            } else {
                renumber(&body, seq)
            };

            let Ok((body, sha)) = prepared else {
                let _ = self.spool.reject(&chunk);
                continue;
            };

            match api
                .upload_chunk(&target.recording_id, seq, body, &sha)
                .await
            {
                ChunkOutcome::Stored => {
                    let _ = self.spool.done(&chunk);
                    self.recorder.lock().unwrap().on_chunk_uploaded(now_ms());
                }
                ChunkOutcome::Retry { after_ms } => {
                    pass_failed = true;

                    *self.delivery_error.lock().unwrap() =
                        Some("Recording upload failed; retrying".into());

                    retry = Some(after_ms);
                }
                ChunkOutcome::RateLimited { after_ms } => {
                    *self.delivery_error.lock().unwrap() =
                        Some("Recording upload rate limited; retrying".into());

                    let delay = after_ms.unwrap_or(1000).max(1);
                    *cooldown = Some(Instant::now() + Duration::from_millis(delay));

                    return Some(Some(delay));
                }
                ChunkOutcome::Unauthorized => {
                    *self.delivery_error.lock().unwrap() =
                        Some("Device authentication expired; registering again".into());
                    self.credentials_rejected().await;
                    return Some(None);
                }
                ChunkOutcome::Rejected(status) => {
                    pass_failed = true;
                    *self.delivery_error.lock().unwrap() =
                        Some(format!("Recording chunk rejected ({status})"));

                    log::warn!("The server rejected chunk {seq} ({status})");
                    let _ = self.spool.reject(&chunk);
                }
            }
        }

        if !pass_failed {
            *self.delivery_error.lock().unwrap() = None;
        }

        retry
    }
}

pub(crate) fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn jitter() -> f64 {
    let mut byte = [0u8; 1];
    let _ = getrandom::fill(&mut byte);
    f64::from(byte[0]) / 256.0
}

fn unauthorized(error: &str) -> bool {
    error.starts_with("401")
}
