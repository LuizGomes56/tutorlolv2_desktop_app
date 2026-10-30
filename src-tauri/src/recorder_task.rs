//! Drives the [`Recorder`] (spec §7): polls port 2999 every second and the
//! League client every 5 s, and carries out the recorder's effects: server
//! calls, the spool, `session.bin`. All I/O lives here; the decisions live in
//! the recorder.

use crate::delivery::now_ms;
use crate::{
    delivery::Delivery as Shared,
    lcu::{Lcu, LcuPhase},
    live_client::LiveClient,
    recorder::{Effect, Recorder},
    spool::{Spool, Target},
    store::Store,
    uploader::{ApiClient, encode_chunk},
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::{AppHandle, Manager, async_runtime};
use tokio::sync::{mpsc, oneshot};
use tutorlolv2_protocol::{Id16, LcuGame, RecordingResult};

const POLL: Duration = Duration::from_secs(1);
const LCU_EVERY_TICKS: u64 = 5;
const EOG_POLL: Duration = Duration::from_secs(2);
/// Give up reading end-of-game stats after this many polls (~3 min).
const EOG_ATTEMPTS: u32 = 90;
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The latest snapshot, read by `get_live_game`.
#[derive(Clone, Default)]
pub struct LatestSnapshot(Arc<Mutex<Option<Arc<[u8]>>>>);

impl LatestSnapshot {
    pub fn get(&self) -> Option<Arc<[u8]>> {
        self.0.lock().unwrap().clone()
    }

    fn set(&self, bytes: Option<Arc<[u8]>>) {
        *self.0.lock().unwrap() = bytes;
    }
}

enum Control {
    Exit(oneshot::Sender<()>),
}

pub struct RecorderHandle {
    control: mpsc::Sender<Control>,
    pub(crate) shared: Arc<Shared>,
}

impl RecorderHandle {
    /// Seals the partial chunk, saves the session and tries one upload pass,
    /// waiting at most `limit` (spec §7.2: closing the app never ends a recording).
    pub fn exit(&self, limit: Duration) {
        let (done, wait) = oneshot::channel();

        if self.control.try_send(Control::Exit(done)).is_ok() {
            async_runtime::block_on(async {
                let _ = tokio::time::timeout(limit, wait).await;
            });
        }
    }
}

pub fn spawn(app: &AppHandle, latest: LatestSnapshot) -> RecorderHandle {
    let dir: PathBuf = app
        .path()
        .app_local_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("com.tutorlolv2"));

    let store = Store::new(dir.clone());
    let spool = Spool::new(dir.join("spool"), Spool::DEFAULT_MAX_BYTES);
    let restored = store.load_session();
    let _ = spool.prune(restored.as_ref().map(|s| s.local_id));

    // session.bin is authoritative if a previous target write was interrupted.
    if let Some(session) = restored.as_ref()
        && let Some((recording_id, seq_offset)) = session.server
        && spool.target_of(session.local_id).is_none()
        && let Err(e) = spool.set_target(
            session.local_id,
            &Target {
                recording_id,
                seq_offset,
            },
        )
    {
        log::error!("Could not restore the upload target: {e}");
    }

    let shared = Arc::new(Shared::new(
        Recorder::new(restored, None, APP_VERSION.into()),
        ApiClient::new(store.load_device().as_ref()),
        spool,
        store,
    ));

    let (control, inbox) = mpsc::channel(1);

    async_runtime::spawn(shared.clone().registration());
    async_runtime::spawn(shared.clone().outbox_loop());
    async_runtime::spawn(shared.clone().run_uploads());
    async_runtime::spawn(shared.clone().sharing_loop(app.clone()));
    async_runtime::spawn(main_loop(shared.clone(), latest, inbox));

    RecorderHandle { control, shared }
}

struct LcuPoller {
    client: Lcu,
    region_known: bool,
}

impl LcuPoller {
    fn new() -> Self {
        Self {
            client: Lcu::new(),
            region_known: false,
        }
    }

    async fn poll(&mut self, shared: &Arc<Shared>) {
        let Some(phase) = self.client.phase().await else {
            return;
        };

        let in_game = matches!(phase, LcuPhase::InProgress | LcuPhase::Reconnect);

        // Refresh even if every observed phase is InProgress: the end and
        // loading phases can be missed while the LCU is unavailable.
        let game = if in_game {
            self.client.game().await
        } else {
            None
        };

        let region = if self.region_known {
            None
        } else {
            self.client.region().await
        };

        self.region_known |= region.is_some();

        let effects = shared
            .recorder
            .lock()
            .unwrap()
            .on_lcu(now_ms(), phase, game, region);

        for effect in effects {
            shared.apply(effect);
        }
    }
}

async fn main_loop(
    shared: Arc<Shared>,
    latest: LatestSnapshot,
    mut inbox: mpsc::Receiver<Control>,
) {
    let live = LiveClient::new();
    let mut lcu = LcuPoller::new();
    let mut ticks: u64 = 0;
    let mut interval = tokio::time::interval(POLL);

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            Some(Control::Exit(done)) = inbox.recv() => {
                let effects = shared.recorder.lock().unwrap().on_exit(now_ms());

                for effect in effects {
                    shared.apply_now(effect);
                }

                shared.upload_pending().await;

                let _ = done.send(());
                return;
            }
        }

        let tick = async {
            let mut effects = Vec::new();

            if ticks == 0 {
                lcu.poll(&shared).await;
            }

            let poll = live.poll().await;
            let live_effects = {
                let mut recorder = shared.recorder.lock().unwrap();
                let effects = recorder.on_live(now_ms(), poll);

                latest.set(recorder.latest());
                effects
            };

            effects.extend(live_effects);
            effects.extend(shared.recorder.lock().unwrap().on_tick(now_ms()));

            for effect in effects {
                shared.apply(effect);
            }

            if ticks > 0 && ticks.is_multiple_of(LCU_EVERY_TICKS) {
                lcu.poll(&shared).await;
            }

            shared.recording_changed();
            ticks += 1;
        };
        tokio::select! {
            biased;
            Some(Control::Exit(done)) = inbox.recv() => {
                let effects = shared.recorder.lock().unwrap().on_exit(now_ms());

                for effect in effects {
                    shared.apply_now(effect);
                }

                shared.upload_pending().await;

                let _ = done.send(());
                return;
            }
            _ = tick => {}
        }
    }
}

impl Shared {
    /// Carries out one effect without blocking the poll loop.
    fn apply(self: &Arc<Self>, effect: Effect) {
        match effect {
            Effect::PersistSession(session) => self.persist(session.as_ref()),
            Effect::SealChunk {
                local_id,
                local_seq,
                chunk,
            } => {
                if let Err(e) = self.spool.reserve_chunk(local_id, local_seq) {
                    log::error!("Could not reserve the chunk sequence: {e}");
                    return;
                }

                let shared = self.clone();

                async_runtime::spawn_blocking(move || {
                    below_normal_priority(|| shared.store_chunk(local_id, local_seq, &chunk));
                    shared.upload_now.notify_one();
                });
            }
            Effect::StartSession { local_id, req } => {
                if let Err(e) = self.spool.set_start(local_id, &req) {
                    log::error!("Could not queue the session start: {e}");
                }

                self.outbox_now.notify_one();
            }
            Effect::Heartbeat { local_id } => {
                let shared = self.clone();

                async_runtime::spawn(async move {
                    if let Some(target) = shared.spool.target_of(local_id) {
                        let api = shared.api.read().await.clone();
                        let _ = api.heartbeat(&target.recording_id).await;
                    }
                });
            }
            Effect::EndRecording { local_id, req } => {
                if let Err(e) = self.spool.set_end(local_id, &req) {
                    log::error!("Could not queue the end of the recording: {e}");
                }

                self.outbox_now.notify_one();
            }
            Effect::FetchEog { local_id, game_id } => {
                async_runtime::spawn(fetch_eog(self.clone(), local_id, game_id));
            }
        }
    }

    /// On exit only the local, synchronous effects matter; the rest resumes on next start.
    fn apply_now(self: &Arc<Self>, effect: Effect) {
        match effect {
            Effect::PersistSession(session) => self.persist(session.as_ref()),
            Effect::SealChunk {
                local_id,
                local_seq,
                chunk,
            } => self.store_chunk(local_id, local_seq, &chunk),
            Effect::StartSession { local_id, req } => {
                let _ = self.spool.set_start(local_id, &req);
            }
            Effect::EndRecording { local_id, req } => {
                let _ = self.spool.set_end(local_id, &req);
            }
            Effect::Heartbeat { .. } | Effect::FetchEog { .. } => {}
        }
    }

    fn persist(&self, session: Option<&crate::recorder::SessionFile>) {
        if let Err(e) = self.store.save_session(session) {
            log::error!("Could not save session.bin: {e}");
        }
    }

    fn store_chunk(&self, local_id: Id16, local_seq: u32, chunk: &tutorlolv2_protocol::RawChunk) {
        let (body, _) = encode_chunk(chunk);

        match self.spool.put(local_id, local_seq, &body) {
            Ok(0) => {}
            Ok(dropped) => log::warn!("Spool full: dropped {dropped} old chunk(s)"),
            Err(e) => log::error!("Could not spool a chunk: {e}"),
        }
    }
}

/// Compression runs below normal priority so it never competes with the game (spec §7.1).
#[cfg(windows)]
fn below_normal_priority(work: impl FnOnce()) {
    use windows::Win32::System::Threading::{
        GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_NORMAL,
    };

    unsafe {
        let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_BELOW_NORMAL);
    }

    work();
    unsafe {
        let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_NORMAL);
    }
}

#[cfg(not(windows))]
fn below_normal_priority(work: impl FnOnce()) {
    work();
}

/// Reads the end-of-game stats right away: they disappear once the player
/// leaves the end screen (spec §8.4, V2).
async fn fetch_eog(shared: Arc<Shared>, local_id: Id16, game_id: Option<u64>) {
    let mut lcu = Lcu::new();

    for _ in 0..EOG_ATTEMPTS {
        if lcu
            .phase()
            .await
            .is_some_and(|phase| phase.is_out_of_game())
        {
            return;
        }

        if let Some(eog) = lcu.eog().await
            && game_id.is_none_or(|id| id == eog.game_id)
        {
            let result = RecordingResult {
                result: eog.result(),
                game_length_s: Some(eog.game_length),
                riot_game: Some(LcuGame {
                    game_id: eog.game_id,
                    queue_id: None,
                    is_custom: None,
                }),
            };

            if let Err(e) = shared.spool.set_result(local_id, &result) {
                log::error!("Could not queue the game result: {e}");
                tokio::time::sleep(EOG_POLL).await;
                continue;
            }

            shared.outbox_now.notify_one();
            return;
        }

        tokio::time::sleep(EOG_POLL).await;
    }
}
