//! The recording state machine (spec §7.2, §8): pure logic, no I/O.

use crate::{lcu::LcuPhase, live_client::LivePoll, spool::new_local_id};
use bincode::{Decode, Encode};
use std::sync::Arc;
use tutorlolv2_protocol::{
    CHUNK_SEAL_MS, DifferentReason, EndReason, EndRecording, GameResult, Id16, LcuGame,
    MAX_SNAPSHOTS_PER_CHUNK, MatchDecision, Probe, RAW_CHUNK_FORMAT, RawChunk, RawSnapshot, Region,
    RiotGameRef, SessionStarted, Snapshot, StartSession,
};

/// What survives an app restart about the current recording (`session.bin`).
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct SessionFile {
    /// Time-ordered id chosen by the app; names the spool directory.
    pub local_id: Id16,
    /// `(recording_id, seq_offset)` once the server answered `StartSession`.
    pub server: Option<(Id16, u32)>,
    /// Probe of the last snapshot seen.
    pub probe: Probe,
    /// Wall clock (Unix ms) of that snapshot.
    pub last_seen_ms: i64,
    pub next_local_seq: u32,
    /// An end signal was seen; snapshots of the same match are dropped.
    pub ended: bool,
    /// An end not acknowledged by the server yet.
    pub pending_end: Option<EndRecording>,
    pub lcu_game_id: Option<u64>,
}

/// Without data from port 2999 for this long, the recording is given up
/// locally; the server's janitor ends it (spec §7.2).
const SUSPEND_LIMIT_MS: i64 = 15 * 60 * 1000;
/// A heartbeat is sent when no chunk or heartbeat reached the server for this long (spec §7.3).
const HEARTBEAT_MS: i64 = 60_000;
/// Port 2999 down for this long while the LCU is out of game ⇒ the player left (spec §8.4).
const LEFT_GAME_MS: i64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// No game being recorded.
    Idle,
    /// Recording, but the server hasn't assigned the recording yet.
    Starting,
    Recording,
    /// The game stopped answering (crash, reconnect, app restart); the recording stays open.
    Suspended,
}

/// Work the driver performs for the recorder.
#[derive(Debug)]
pub enum Effect {
    StartSession {
        local_id: Id16,
        req: StartSession,
    },
    /// `chunk.seq == local_seq`; the uploader maps it to the server seq.
    SealChunk {
        local_id: Id16,
        local_seq: u32,
        chunk: RawChunk,
    },
    Heartbeat {
        local_id: Id16,
    },
    EndRecording {
        local_id: Id16,
        req: EndRecording,
    },
    /// Read the LCU end-of-game stats while they last (spec §8.4).
    FetchEog {
        local_id: Id16,
        game_id: Option<u64>,
    },
    PersistSession(Option<SessionFile>),
}

pub struct Recorder {
    session: Option<SessionFile>,
    buffer: Vec<RawSnapshot>,
    /// Wall clock of the first buffered snapshot.
    buffer_started_ms: Option<i64>,
    last_captured_ms: i64,
    last_bytes: Option<Arc<[u8]>>,
    /// The last poll returned a snapshot.
    alive: bool,
    /// Port 2999 stopped answering at this time while a recording was active.
    suspended_since_ms: Option<i64>,
    /// Last chunk upload or heartbeat.
    last_contact_ms: i64,
    /// The restored session still has to replay its pending calls.
    startup_due: bool,
    lcu_game: Option<LcuGame>,
    /// Never attach a completed or contradicted game id to a later match.
    retired_lcu_game_id: Option<u64>,
    region: Option<Region>,
    app_version: String,
}

fn random10() -> [u8; 10] {
    let mut bytes = [0u8; 10];
    let _ = getrandom::fill(&mut bytes);
    bytes
}

impl Recorder {
    pub fn new(restored: Option<SessionFile>, region: Option<Region>, app_version: String) -> Self {
        Self {
            startup_due: restored.is_some(),
            retired_lcu_game_id: restored
                .as_ref()
                .filter(|s| s.ended)
                .and_then(|s| s.lcu_game_id),
            session: restored,
            buffer: Vec::new(),
            buffer_started_ms: None,
            last_captured_ms: i64::MIN,
            last_bytes: None,
            alive: false,
            suspended_since_ms: None,
            last_contact_ms: i64::MIN,
            lcu_game: None,
            region,
            app_version,
        }
    }

    pub fn status(&self) -> Status {
        match &self.session {
            None => Status::Idle,
            Some(s) if s.ended => Status::Idle,
            Some(_) if !self.alive => Status::Suspended,
            Some(s) if s.server.is_none() => Status::Starting,
            Some(_) => Status::Recording,
        }
    }

    /// The latest snapshot while port 2999 answers (served to `get_live_game`).
    pub fn latest(&self) -> Option<Arc<[u8]>> {
        self.alive.then(|| self.last_bytes.clone()).flatten()
    }

    fn active(&mut self) -> Option<&mut SessionFile> {
        self.session.as_mut().filter(|s| !s.ended)
    }

    fn persist(&self) -> Effect {
        Effect::PersistSession(self.session.clone())
    }

    pub fn on_live(&mut self, now_ms: i64, poll: LivePoll) -> Vec<Effect> {
        let (bytes, snapshot) = match poll {
            LivePoll::Snapshot { bytes, snapshot } => (bytes, snapshot),
            LivePoll::NotReady | LivePoll::Unreachable => return self.lost_game(now_ms),
        };

        self.alive = true;
        self.suspended_since_ms = None;

        if self.last_bytes.as_deref() == Some(&bytes[..]) {
            return Vec::new();
        }

        self.last_bytes = Some(bytes.clone());

        // A cached LCU id is optional metadata, not fresh proof that a new
        // live-client snapshot belongs to the previous match.
        if self.cached_game_contradicts(&snapshot.probe, now_ms) {
            self.retire_lcu_game(self.lcu_game.as_ref().map(|game| game.game_id));
        }

        let mut probe = snapshot.probe.clone();

        probe.riot_game = self.riot_game();

        let decision = self
            .session
            .as_ref()
            .map(|s| (s.ended, s.probe.same_match(s.last_seen_ms, &probe, now_ms)));

        match decision {
            None => self.start_new(now_ms, &bytes, &snapshot, probe),
            Some((true, MatchDecision::Same)) => Vec::new(),
            Some((true, MatchDecision::Different(_))) => {
                self.start_new(now_ms, &bytes, &snapshot, probe)
            }
            Some((false, MatchDecision::Different(_))) => {
                let mut effects = self.end(EndReason::Superseded, None, None, false);

                effects.extend(self.start_new(now_ms, &bytes, &snapshot, probe));
                effects
            }
            Some((false, MatchDecision::Same)) => {
                let first_contact = self.last_contact_ms == i64::MIN;
                let lcu_game_id = self.lcu_game.as_ref().map(|g| g.game_id);
                let session = self.active().expect("active session");

                session.probe = probe;
                session.last_seen_ms = now_ms;
                session.lcu_game_id = session.lcu_game_id.or(lcu_game_id);

                if first_contact {
                    self.last_contact_ms = now_ms;
                }

                let mut effects = self.buffer_snapshot(now_ms, &bytes, &snapshot);

                if let Some(end) = snapshot.game_end {
                    effects.extend(self.end(
                        EndReason::GameEndEvent,
                        end.result,
                        Some(end.event_time),
                        true,
                    ));
                }

                effects
            }
        }
    }

    /// Port 2999 stopped answering (or answers without a recordable game).
    fn lost_game(&mut self, now_ms: i64) -> Vec<Effect> {
        self.alive = false;

        if self.active().is_none() || self.suspended_since_ms.is_some() {
            return Vec::new();
        }

        self.suspended_since_ms = Some(now_ms);
        self.seal()
    }

    fn cached_game_contradicts(&self, probe: &Probe, now_ms: i64) -> bool {
        let Some(session) = self.session.as_ref() else {
            return false;
        };

        let Some(cached) = self.riot_game() else {
            return false;
        };

        if session.probe.riot_game.as_ref() != Some(&cached) {
            return false;
        }

        let mut previous = session.probe.clone();
        previous.riot_game = None;

        matches!(previous.same_match(session.last_seen_ms, probe, now_ms),
            MatchDecision::Different(reason) if reason != DifferentReason::TimeTooFast)
    }

    fn retire_lcu_game(&mut self, game_id: Option<u64>) {
        if let Some(game_id) = game_id {
            self.retired_lcu_game_id = Some(game_id);
            if self
                .lcu_game
                .as_ref()
                .is_some_and(|game| game.game_id == game_id)
            {
                self.lcu_game = None;
            }
        }
    }

    fn riot_game(&self) -> Option<RiotGameRef> {
        Some(RiotGameRef {
            region: self.region.clone()?,
            game_id: self.lcu_game.as_ref()?.game_id,
        })
    }

    fn start_new(
        &mut self,
        now_ms: i64,
        bytes: &[u8],
        snapshot: &Snapshot,
        probe: Probe,
    ) -> Vec<Effect> {
        let local_id = new_local_id(now_ms, random10());
        let session = SessionFile {
            local_id,
            server: None,
            probe: probe.clone(),
            last_seen_ms: now_ms,
            next_local_seq: 0,
            ended: false,
            pending_end: None,
            lcu_game_id: self.lcu_game.as_ref().map(|g| g.game_id),
        };

        self.session = Some(session);
        self.buffer.clear();
        self.buffer_started_ms = None;
        self.last_contact_ms = now_ms;
        self.startup_due = false;

        let req = StartSession {
            probe,
            region: self.region.clone(),
            lcu: self.lcu_game.clone(),
            app_version: self.app_version.clone(),
        };

        let mut effects = vec![Effect::StartSession { local_id, req }, self.persist()];

        effects.extend(self.buffer_snapshot(now_ms, bytes, snapshot));

        if let Some(end) = snapshot.game_end {
            effects.extend(self.end(
                EndReason::GameEndEvent,
                end.result,
                Some(end.event_time),
                true,
            ));
        }

        effects
    }

    fn buffer_snapshot(&mut self, now_ms: i64, bytes: &[u8], snapshot: &Snapshot) -> Vec<Effect> {
        // Strictly increasing even if the wall clock goes back (the server rejects otherwise)
        let captured_at_ms = now_ms.max(self.last_captured_ms.saturating_add(1));

        self.last_captured_ms = captured_at_ms;
        self.buffer_started_ms.get_or_insert(now_ms);
        self.buffer.push(RawSnapshot {
            captured_at_ms,
            game_time: snapshot.game_time,
            json: bytes.to_vec(),
        });

        if self.buffer.len() >= MAX_SNAPSHOTS_PER_CHUNK {
            self.seal()
        } else {
            Vec::new()
        }
    }

    fn seal(&mut self) -> Vec<Effect> {
        if self.buffer.is_empty() {
            return Vec::new();
        }

        let snapshots = std::mem::take(&mut self.buffer);

        self.buffer_started_ms = None;

        let Some(session) = self.session.as_mut() else {
            return Vec::new();
        };

        let local_seq = session.next_local_seq;
        session.next_local_seq += 1;

        let chunk = RawChunk {
            format: RAW_CHUNK_FORMAT,
            seq: local_seq,
            snapshots,
        };

        vec![
            Effect::SealChunk {
                local_id: session.local_id,
                local_seq,
                chunk,
            },
            self.persist(),
        ]
    }

    /// Seals and ends the active recording; later snapshots of the same match are dropped.
    fn end(
        &mut self,
        reason: EndReason,
        result: Option<GameResult>,
        final_game_time: Option<f32>,
        fetch_eog: bool,
    ) -> Vec<Effect> {
        let mut effects = self.seal();

        let Some(session) = self.active() else {
            return effects;
        };

        let req = EndRecording {
            reason,
            result,
            final_game_time,
        };

        session.ended = true;
        session.pending_end = Some(req.clone());

        let (local_id, game_id) = (session.local_id, session.lcu_game_id);
        self.retire_lcu_game(game_id);

        effects.push(Effect::EndRecording { local_id, req });

        if fetch_eog {
            effects.push(Effect::FetchEog { local_id, game_id });
        }

        effects.push(self.persist());
        effects
    }

    pub fn on_lcu(
        &mut self,
        now_ms: i64,
        phase: LcuPhase,
        game: Option<LcuGame>,
        region: Option<Region>,
    ) -> Vec<Effect> {
        if region.is_some() {
            self.region = region;
        }

        if let Some(game) = game.filter(|game| Some(game.game_id) != self.retired_lcu_game_id) {
            if let Some(session) = self.active()
                && session.lcu_game_id.is_none()
            {
                session.lcu_game_id = Some(game.game_id);
            }

            self.lcu_game = Some(game);
        }

        let mut effects = Vec::new();

        if self.active().is_some() {
            if phase.is_end() {
                effects = self.end(EndReason::LcuEndOfGame, None, None, true);
            } else if phase.is_out_of_game()
                && self
                    .suspended_since_ms
                    .is_some_and(|since| now_ms - since >= LEFT_GAME_MS)
            {
                effects = self.end(EndReason::LcuLeftGame, None, None, false);
            }
        }

        if phase.is_end() || phase.is_out_of_game() {
            // Never let the finished game's id reach the next game's probe
            self.lcu_game = None;
        }

        effects
    }

    pub fn on_session_started(&mut self, local_id: Id16, started: &SessionStarted) -> Vec<Effect> {
        match self.session.as_mut().filter(|s| s.local_id == local_id) {
            Some(session) => {
                session.server = Some((started.recording_id, started.next_chunk_seq));
                vec![self.persist()]
            }
            None => Vec::new(),
        }
    }

    /// The server acknowledged the end of `local_id`.
    pub fn on_end_sent(&mut self, local_id: Id16) -> Vec<Effect> {
        match self.session.as_mut().filter(|s| s.local_id == local_id) {
            Some(session) => {
                session.pending_end = None;
                vec![self.persist()]
            }
            None => Vec::new(),
        }
    }

    pub fn on_tick(&mut self, now_ms: i64) -> Vec<Effect> {
        let mut effects = Vec::new();

        if std::mem::take(&mut self.startup_due)
            && let Some(session) = &self.session
        {
            if !session.ended && session.server.is_none() {
                let req = StartSession {
                    probe: session.probe.clone(),
                    region: self.region.clone(),
                    lcu: None,
                    app_version: self.app_version.clone(),
                };

                effects.push(Effect::StartSession {
                    local_id: session.local_id,
                    req,
                });
            }

            if let Some(req) = session.pending_end.clone() {
                effects.push(Effect::EndRecording {
                    local_id: session.local_id,
                    req,
                });
            }
        }

        if self
            .buffer_started_ms
            .is_some_and(|since| now_ms - since >= CHUNK_SEAL_MS)
        {
            effects.extend(self.seal());
        }

        let heartbeat_due = self.alive && now_ms - self.last_contact_ms >= HEARTBEAT_MS;

        if heartbeat_due
            && let Some(session) = self
                .session
                .as_ref()
                .filter(|s| !s.ended && s.server.is_some())
        {
            effects.push(Effect::Heartbeat {
                local_id: session.local_id,
            });

            self.last_contact_ms = now_ms;
        }

        if self.active().is_some()
            && self
                .suspended_since_ms
                .is_some_and(|since| now_ms - since >= SUSPEND_LIMIT_MS)
        {
            self.session = None;
            self.suspended_since_ms = None;

            effects.push(Effect::PersistSession(None));
        }

        effects
    }

    pub fn on_chunk_uploaded(&mut self, now_ms: i64) {
        self.last_contact_ms = now_ms;
    }

    pub fn on_exit(&mut self, _now_ms: i64) -> Vec<Effect> {
        let mut effects = self.seal();

        if self.session.is_some() {
            effects.push(self.persist());
        }

        effects
    }
}
