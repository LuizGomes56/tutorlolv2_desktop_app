//! On-disk queue of sealed chunks not acknowledged by the server yet (spec
//! §12.4). Layout: `spool/<local_id hex>/<local_seq:08>.chunk`, plus
//! `target.bin` once the server assigned the recording.

use crate::BINCODE_CONFIG;
use bincode::{Decode, Encode};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::Mutex,
    time::{Duration, SystemTime},
};
use tutorlolv2_protocol::{EndRecording, Id16, RecordingResult, StartSession};

const CHUNK_EXT: &str = "chunk";
const TARGET_FILE: &str = "target.bin";
const START_FILE: &str = "start.bin";
const END_FILE: &str = "end.bin";
const RESULT_FILE: &str = "result.bin";
const RESERVED_FILE: &str = "reserved.bin";
const REJECTED_RETENTION: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const REJECTED_EXT: &str = "rejected";

/// Where a local session's chunks go on the server.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode)]
pub struct Target {
    pub recording_id: Id16,
    /// Added to every local seq (non-zero only when the server resumed an
    /// existing recording for a session the app had lost).
    pub seq_offset: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingChunk {
    pub local_id: Id16,
    pub local_seq: u32,
    pub path: PathBuf,
    /// `None` until the server answered `StartSession`; not uploadable yet.
    pub target: Option<Target>,
}

/// Server calls of one local session not acknowledged yet. Kept on disk so
/// they survive restarts and a newer session replacing this one (outbox).
#[derive(Clone, Debug, PartialEq)]
pub struct OutboxEntry {
    pub local_id: Id16,
    pub start: Option<StartSession>,
    pub end: Option<EndRecording>,
    pub result: Option<RecordingResult>,
    pub target: Option<Target>,
}

pub struct Spool {
    root: PathBuf,
    max_bytes: u64,
    mutation: Mutex<()>,
}

/// Time-ordered local session id with the UUIDv7 layout, so folder names sort by age.
pub fn new_local_id(now_ms: i64, random: [u8; 10]) -> Id16 {
    let mut id = [0u8; 16];

    id[..6].copy_from_slice(&(now_ms as u64).to_be_bytes()[2..]);
    id[6] = 0x70 | (random[0] & 0x0F);
    id[7] = random[1];
    id[8] = 0x80 | (random[2] & 0x3F);
    id[9..].copy_from_slice(&random[3..]);
    id
}

fn hex(id: &Id16) -> String {
    id.iter().map(|b| format!("{b:02x}")).collect()
}

fn parse_hex(name: &str) -> Option<Id16> {
    if name.len() != 32 {
        return None;
    }

    let mut id = [0u8; 16];

    for (i, byte) in id.iter_mut().enumerate() {
        *byte = u8::from_str_radix(name.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }

    Some(id)
}

impl Spool {
    /// 64 MiB (spec §12.4).
    pub const DEFAULT_MAX_BYTES: u64 = 64 << 20;

    pub fn new(root: PathBuf, max_bytes: u64) -> Self {
        Self {
            root,
            max_bytes,
            mutation: Mutex::new(()),
        }
    }

    fn session_dir(&self, local_id: &Id16) -> PathBuf {
        self.root.join(hex(local_id))
    }

    /// Stores a sealed chunk. When the spool would exceed its budget, the
    /// oldest chunks are dropped first; returns how many were dropped.
    pub fn put(&self, local_id: Id16, local_seq: u32, body: &[u8]) -> io::Result<u32> {
        let _guard = self.mutation.lock().unwrap();

        if body.len() as u64 > self.max_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Chunk exceeds spool budget",
            ));
        }

        let next = local_seq
            .checked_add(1)
            .filter(|n| *n <= i32::MAX as u32)
            .ok_or_else(|| io::Error::other("Chunk sequence exhausted"))?;

        let reserved_path = self.session_dir(&local_id).join(RESERVED_FILE);
        let previous = fs::metadata(&reserved_path).map_or(0, |m| m.len());
        let reserve_bytes = bincode::encode_to_vec(next, BINCODE_CONFIG)
            .map_err(io::Error::other)?
            .len() as u64;

        self.budget(
            body.len() as u64 + reserve_bytes.saturating_sub(previous),
            false,
        )?;

        let reserved_drops = self.reserve_chunk_inner(local_id, local_seq)?;
        let dropped = reserved_drops + self.enforce_budget(body.len() as u64)?;
        let dir = self.session_dir(&local_id);

        fs::create_dir_all(&dir)?;
        write_atomic(&dir.join(format!("{local_seq:08}.{CHUNK_EXT}")), body)?;

        Ok(dropped)
    }

    /// Reserve before asynchronous compression, so a recovered session cannot
    /// claim a sequence still being sealed by an older local session.
    pub fn reserve_chunk(&self, local_id: Id16, local_seq: u32) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        self.reserve_chunk_inner(local_id, local_seq).map(drop)
    }

    fn reserve_chunk_inner(&self, local_id: Id16, local_seq: u32) -> io::Result<u32> {
        let next = local_seq
            .checked_add(1)
            .filter(|n| *n <= i32::MAX as u32)
            .ok_or_else(|| io::Error::other("Chunk sequence exhausted"))?;

        let dir = self.session_dir(&local_id);
        let reserved: u32 = read_record(&dir.join(RESERVED_FILE)).unwrap_or(0);

        if next > reserved {
            fs::create_dir_all(&dir)?;

            return self.write_bounded(
                &dir.join(RESERVED_FILE),
                &bincode::encode_to_vec(next, BINCODE_CONFIG).map_err(io::Error::other)?,
            );
        }

        Ok(0)
    }

    /// Assign once, accounting for both pending and not-yet-compressed chunks.
    /// Replaying a start after a crash must keep the original mapping.
    pub fn assign_target(&self, local_id: Id16, mut proposed: Target) -> io::Result<Target> {
        let _guard = self.mutation.lock().unwrap();

        if let Some(existing) = self.target_of(local_id) {
            if existing.recording_id != proposed.recording_id {
                return Err(io::Error::other("Session target changed recordings"));
            }

            return Ok(existing);
        }

        for entry in fs::read_dir(&self.root).into_iter().flatten() {
            let entry = entry?;
            let dir = entry.path();

            let Some(other_id) = entry.file_name().to_str().and_then(parse_hex) else {
                continue;
            };

            let Some(target) = Self::target(&dir) else {
                continue;
            };

            if other_id == local_id || target.recording_id != proposed.recording_id {
                continue;
            }

            let mut next: u32 = read_record(&dir.join(RESERVED_FILE)).unwrap_or(0);

            for file in fs::read_dir(&dir)? {
                let path = file?.path();

                if path
                    .extension()
                    .is_some_and(|e| e == CHUNK_EXT || e == REJECTED_EXT)
                    && let Some(seq) = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .and_then(|s| s.parse::<u32>().ok())
                {
                    next = next.max(
                        seq.checked_add(1)
                            .ok_or_else(|| io::Error::other("Chunk sequence exhausted"))?,
                    );
                }
            }

            let next = target
                .seq_offset
                .checked_add(next)
                .filter(|n| *n <= i32::MAX as u32)
                .ok_or_else(|| io::Error::other("Chunk sequence exhausted"))?;

            proposed.seq_offset = proposed.seq_offset.max(next);
        }

        self.set_target_inner(local_id, &proposed)?;

        Ok(proposed)
    }

    /// Includes rejected chunks; metadata is counted but never evicted.
    fn enforce_budget(&self, incoming: u64) -> io::Result<u32> {
        self.budget(incoming, true)
    }

    fn budget(&self, incoming: u64, evict: bool) -> io::Result<u32> {
        let mut used = 0u64;
        let mut chunks = Vec::new();

        for entry in fs::read_dir(&self.root).into_iter().flatten() {
            let dir = entry?.path();

            if !dir.is_dir() {
                continue;
            }

            for file in fs::read_dir(&dir)? {
                let path = file?.path();
                let metadata = fs::metadata(&path)?;

                if !metadata.is_file() {
                    continue;
                }

                used = used.saturating_add(metadata.len());

                if path
                    .extension()
                    .is_some_and(|e| e == CHUNK_EXT || e == REJECTED_EXT)
                {
                    let rejected = path.extension().is_some_and(|e| e == REJECTED_EXT);

                    // Rejected diagnostics go before pending data, then oldest session/seq.
                    chunks.push((!rejected, path, metadata.len()));
                }
            }
        }

        let evictable: u64 = chunks.iter().map(|(_, _, bytes)| *bytes).sum();

        if used.saturating_sub(evictable).saturating_add(incoming) > self.max_bytes {
            return Err(io::Error::other(
                "Spool metadata and incoming record exceed budget",
            ));
        }

        if !evict {
            return Ok(0);
        }

        chunks.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
        let mut dropped = 0;

        for (_, path, size) in chunks {
            if used.saturating_add(incoming) <= self.max_bytes {
                break;
            }

            fs::remove_file(path)?;
            used = used.saturating_sub(size);
            dropped += 1;
        }

        if used.saturating_add(incoming) > self.max_bytes {
            return Err(io::Error::other("Spool metadata exceeds budget"));
        }

        Ok(dropped)
    }

    pub fn set_target(&self, local_id: Id16, target: &Target) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        self.set_target_inner(local_id, target)
    }

    fn set_target_inner(&self, local_id: Id16, target: &Target) -> io::Result<()> {
        let dir = self.session_dir(&local_id);

        fs::create_dir_all(&dir)?;

        let bytes = bincode::encode_to_vec(target, BINCODE_CONFIG).map_err(io::Error::other)?;

        self.write_bounded(&dir.join(TARGET_FILE), &bytes).map(drop)
    }

    /// Where `local_id`'s chunks go, once the server assigned the recording.
    pub fn target_of(&self, local_id: Id16) -> Option<Target> {
        Self::target(&self.session_dir(&local_id))
    }

    fn target(dir: &Path) -> Option<Target> {
        read_record(&dir.join(TARGET_FILE))
    }

    /// Queues `StartSession`; the first request of a session wins.
    pub fn set_start(&self, local_id: Id16, request: &StartSession) -> io::Result<()> {
        self.write_once(local_id, START_FILE, request)
    }

    /// Queues the end; the first end of a session wins (as on the server).
    pub fn set_end(&self, local_id: Id16, request: &EndRecording) -> io::Result<()> {
        self.write_once(local_id, END_FILE, request)
    }

    /// Persist EOG data before attempting any network request.
    pub fn set_result(&self, local_id: Id16, request: &RecordingResult) -> io::Result<()> {
        self.write_once(local_id, RESULT_FILE, request)
    }

    pub fn clear_result(&self, local_id: Id16) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        remove_if_present(&self.session_dir(&local_id).join(RESULT_FILE))
    }

    pub fn clear_start(&self, local_id: Id16) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        remove_if_present(&self.session_dir(&local_id).join(START_FILE))
    }

    pub fn clear_end(&self, local_id: Id16) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        remove_if_present(&self.session_dir(&local_id).join(END_FILE))
    }

    fn write_once(&self, local_id: Id16, name: &str, value: &impl Encode) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        let dir = self.session_dir(&local_id);

        fs::create_dir_all(&dir)?;

        let path = dir.join(name);

        if path.exists() {
            return Ok(());
        }

        let bytes = bincode::encode_to_vec(value, BINCODE_CONFIG).map_err(io::Error::other)?;

        self.write_bounded(&path, &bytes).map(drop)
    }

    /// Caller holds mutation. Controls and reservations share the same budget
    /// as chunks; pending data may be evicted, but controls are never discarded.
    fn write_bounded(&self, path: &Path, bytes: &[u8]) -> io::Result<u32> {
        if bytes.len() as u64 > self.max_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Record exceeds spool budget",
            ));
        }

        let previous = match fs::metadata(path) {
            Ok(metadata) => metadata.len(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => 0,
            Err(e) => return Err(e),
        };

        let dropped = self.enforce_budget((bytes.len() as u64).saturating_sub(previous))?;

        write_atomic(path, bytes)?;

        if dropped > 0 {
            log::warn!("Spool full: dropped {dropped} chunk(s) to preserve control data");
        }

        Ok(dropped)
    }

    /// Sessions with a pending start or end, oldest first: they must reach the
    /// server in this order, so an old game never supersedes a newer one.
    pub fn outbox(&self) -> Vec<OutboxEntry> {
        let Ok(sessions) = fs::read_dir(&self.root) else {
            return Vec::new();
        };

        let mut entries = sessions
            .flatten()
            .filter_map(|entry| {
                let local_id = parse_hex(entry.file_name().to_str()?)?;
                let dir = entry.path();
                let start = read_record(&dir.join(START_FILE));
                let end = read_record(&dir.join(END_FILE));
                let result = read_record(&dir.join(RESULT_FILE));
                (start.is_some() || end.is_some() || result.is_some()).then(|| OutboxEntry {
                    local_id,
                    start,
                    end,
                    result,
                    target: Self::target(&dir),
                })
            })
            .collect::<Vec<_>>();

        entries.sort_by_key(|e| e.local_id);
        entries
    }

    /// Every stored chunk, oldest session first, then by seq.
    pub fn pending(&self) -> Vec<PendingChunk> {
        let Ok(sessions) = fs::read_dir(&self.root) else {
            return Vec::new();
        };

        let mut chunks = sessions
            .flatten()
            .filter_map(|entry| Some((parse_hex(entry.file_name().to_str()?)?, entry.path())))
            .flat_map(|(local_id, dir)| {
                let target = Self::target(&dir);
                fs::read_dir(&dir)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .filter_map(move |file| {
                        let path = file.path();
                        (path.extension()? == CHUNK_EXT).then_some(())?;
                        let local_seq = path.file_stem()?.to_str()?.parse().ok()?;
                        Some(PendingChunk {
                            local_id,
                            local_seq,
                            path,
                            target,
                        })
                    })
            })
            .collect::<Vec<_>>();

        chunks.sort_by_key(|c| (c.local_id, c.local_seq));
        chunks
    }

    /// The server stored the chunk.
    pub fn done(&self, chunk: &PendingChunk) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        remove_if_present(&chunk.path)
    }

    /// The server refused the chunk for good; kept as `.rejected` for diagnosis.
    pub fn reject(&self, chunk: &PendingChunk) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        fs::rename(&chunk.path, chunk.path.with_extension(REJECTED_EXT))
    }

    /// Removes session folders that hold no chunk (pending or rejected) and no
    /// pending call anymore, except `keep` (the session still being recorded).
    pub fn prune(&self, keep: Option<Id16>) -> io::Result<()> {
        let _guard = self.mutation.lock().unwrap();
        let Ok(sessions) = fs::read_dir(&self.root) else {
            return Ok(());
        };

        for entry in fs::read_dir(&self.root)? {
            let dir = entry?.path();

            if !dir.is_dir() {
                continue;
            }

            for file in fs::read_dir(dir)? {
                let path = file?.path();

                if path.extension().is_some_and(|e| e == "tmp") {
                    fs::remove_file(path)?;
                    continue;
                }

                if path.extension().is_some_and(|e| e == REJECTED_EXT)
                    && fs::metadata(&path)?
                        .modified()
                        .ok()
                        .and_then(|t| SystemTime::now().duration_since(t).ok())
                        .is_some_and(|age| age >= REJECTED_RETENTION)
                {
                    fs::remove_file(path)?;
                }
            }
        }

        self.enforce_budget(0)?;

        for entry in sessions.flatten() {
            let Some(local_id) = entry.file_name().to_str().and_then(parse_hex) else {
                continue;
            };

            let has_chunks = fs::read_dir(entry.path())?.flatten().any(|f| {
                let path = f.path();

                path.extension()
                    .is_some_and(|x| x == CHUNK_EXT || x == REJECTED_EXT)
                    || path
                        .file_name()
                        .is_some_and(|n| n == START_FILE || n == END_FILE || n == RESULT_FILE)
            });

            if !has_chunks && Some(local_id) != keep {
                fs::remove_dir_all(entry.path())?;
            }
        }

        Ok(())
    }
}

fn read_record<T: Decode<()>>(path: &Path) -> Option<T> {
    let bytes = fs::read(path).ok()?;

    bincode::decode_from_slice(&bytes, BINCODE_CONFIG)
        .ok()
        .map(|(value, _)| value)
}

fn remove_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    use std::io::Write;
    let mut file = fs::File::create(&tmp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    fs::rename(tmp, path)
}
