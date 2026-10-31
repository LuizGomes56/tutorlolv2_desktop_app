//! Serialized metadata cache. Replay payloads are never persisted here.
use crate::{sharing::private_share, store::Store, uploader::ApiClient};
use bincode::{Decode, Encode};
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
use tutorlolv2_protocol::*;

#[derive(Clone, PartialEq, Encode, Decode)]
struct Index {
    device_id: Option<Id16>,
    entries: Vec<HistoryEntry>,
    unavailable: Vec<Id16>,
    // Last authoritative journal contribution per local, including removed sessions.
    // Retain aliases after row collapse so surviving journals can decrease independently.
    losses: Vec<(Id16, Option<Id16>, u32)>,
    legacy_losses: Vec<(Option<Id16>, Option<Id16>, u32)>,
}

#[derive(Decode)]
struct PreviousIndex {
    device_id: Option<Id16>,
    entries: Vec<HistoryEntry>,
    unavailable: Vec<Id16>,
}

// Accept the initial unreleased M3 metadata payload without losing cached hide flags.
#[derive(Decode)]
struct LegacyIndex {
    device_id: Option<Id16>,
    entries: Vec<HistoryEntry>,
}

const UNAVAILABLE: &str = "Recording unavailable: no longer owned or removed from the server";

fn decode_index(bytes: &[u8]) -> Option<Index> {
    if let Ok((index, used)) = bincode::decode_from_slice::<Index, _>(bytes, crate::BINCODE_CONFIG)
        && used == bytes.len()
    {
        return Some(index);
    }

    if let Ok((previous, used)) =
        bincode::decode_from_slice::<PreviousIndex, _>(bytes, crate::BINCODE_CONFIG)
        && used == bytes.len()
    {
        return Some(migrate_index(
            previous.device_id,
            previous.entries,
            previous.unavailable,
        ));
    }

    let (legacy, used) =
        bincode::decode_from_slice::<LegacyIndex, _>(bytes, crate::BINCODE_CONFIG).ok()?;

    (used == bytes.len()).then(|| migrate_index(legacy.device_id, legacy.entries, vec![]))
}

fn migrate_index(
    device_id: Option<Id16>,
    entries: Vec<HistoryEntry>,
    unavailable: Vec<Id16>,
) -> Index {
    // Older aggregate totals have no recoverable per-local provenance. Preserve them
    // as a floor, rather than inventing additive losses or guessing which journal
    // supplied them. Newly tracked contributions can still decrease independently.
    let legacy_losses = entries
        .iter()
        .filter(|e| e.lost_chunks > 0)
        .map(|e| (e.local_id, e.recording_id, e.lost_chunks))
        .collect();
    Index {
        device_id,
        entries,
        unavailable,
        losses: vec![],
        legacy_losses,
    }
}

impl Index {
    fn loss(&mut self, local: Id16, id: Option<Id16>, lost: u32) {
        if let Some(entry) = self.losses.iter_mut().find(|e| e.0 == local) {
            entry.1 = id.or(entry.1);
            entry.2 = lost;
        } else {
            self.losses.push((local, id, lost));
        }
        for (alias, recording, _) in &mut self.legacy_losses {
            if *alias == Some(local) {
                *recording = id.or(*recording);
            }
        }
    }

    fn reconcile_losses(&mut self) {
        for row in &mut self.entries {
            let matches = |local, id| {
                row.local_id.is_some() && row.local_id == local
                    || row.recording_id.is_some() && row.recording_id == id
            };
            let known = self
                .losses
                .iter()
                .filter(|(local, id, _)| matches(Some(*local), *id))
                .fold(0u32, |sum, (_, _, lost)| sum.saturating_add(*lost));
            let legacy = self
                .legacy_losses
                .iter()
                .filter(|(local, id, _)| matches(*local, *id))
                .map(|(_, _, lost)| *lost)
                .max()
                .unwrap_or(0);
            row.lost_chunks = known.max(legacy);
        }
    }
}

struct State {
    index: Index,
    refreshing: bool,
    repair_requested: bool,
    error: Option<String>,
    attempted: Option<Instant>,
    revision: u64,
    details_after: Option<Id16>,
}

pub struct History {
    store: Store,
    state: Mutex<State>,
}

pub fn error(kind: HistoryErrorKind, message: &str, retryable: bool) -> HistoryError {
    HistoryError {
        kind,
        message: message.into(),
        retryable,
    }
}

fn storage(_: std::io::Error) -> HistoryError {
    error(
        HistoryErrorKind::Storage,
        "Could not save local history",
        true,
    )
}

pub fn authorize(label: &str) -> Result<(), HistoryError> {
    if label == "main" {
        Ok(())
    } else {
        Err(error(
            HistoryErrorKind::Unauthorized,
            "History requires the main window",
            false,
        ))
    }
}

pub fn parse_id(text: &str) -> Result<Id16, HistoryError> {
    let invalid = || error(HistoryErrorKind::InvalidData, "Invalid recording ID", false);
    if text.len() != 36 || [8, 13, 18, 23].iter().any(|i| text.as_bytes()[*i] != b'-') {
        return Err(invalid());
    }
    let hex = text.replace('-', "");
    let mut id = [0; 16];
    for (i, b) in id.iter_mut().enumerate() {
        *b = u8::from_str_radix(hex.get(i * 2..i * 2 + 2).ok_or_else(invalid)?, 16)
            .map_err(|_| invalid())?;
    }
    Ok(id)
}

impl History {
    pub fn new(store: Store, device_id: Option<Id16>) -> Self {
        let mut index = store.load_history_with(decode_index).unwrap_or(Index {
            device_id,
            entries: vec![],
            unavailable: vec![],
            losses: vec![],
            legacy_losses: vec![],
        });

        if index.device_id != device_id {
            store.archive_history();
            index = Index {
                device_id,
                entries: vec![],
                unavailable: vec![],
                losses: vec![],
                legacy_losses: vec![],
            };
        }

        Self {
            store,
            state: Mutex::new(State {
                index,
                refreshing: false,
                repair_requested: false,
                error: None,
                attempted: None,
                revision: 0,
                details_after: None,
            }),
        }
    }

    pub fn bind(&self, id: Option<Id16>) -> Result<(), HistoryError> {
        let mut s = self.state.lock().unwrap();

        if s.index.device_id == id {
            return Ok(());
        }

        self.store.archive_history();
        let next = Index {
            device_id: id,
            entries: vec![],
            unavailable: vec![],
            losses: vec![],
            legacy_losses: vec![],
        };

        self.store.save_history(&next).map_err(storage)?;

        s.index = next;
        s.attempted = None;
        s.details_after = None;
        s.revision += 1;

        Ok(())
    }

    fn update_for_device(
        &self,
        device: Option<Id16>,
        f: impl FnOnce(&mut Index),
    ) -> Result<(), HistoryError> {
        let mut s = self.state.lock().unwrap();
        if s.index.device_id != device {
            return Err(error(
                HistoryErrorKind::Unauthorized,
                "History device changed",
                true,
            ));
        }
        let mut next = s.index.clone();

        f(&mut next);
        next.reconcile_losses();

        for entry in &mut next.entries {
            if entry
                .recording_id
                .is_some_and(|id| next.unavailable.contains(&id))
            {
                entry.last_error = Some(UNAVAILABLE.into());
            }
        }

        if next != s.index {
            self.store.save_history(&next).map_err(storage)?;
            s.index = next;
            s.revision += 1;
        }

        Ok(())
    }

    pub fn errors(
        &self,
        device: Option<Id16>,
        errors: Vec<(Id16, Option<Id16>, String)>,
    ) -> Result<(), HistoryError> {
        self.update_for_device(device, |index| {
            let entries = &mut index.entries;
            for e in entries.iter_mut() {
                e.last_error = None;
            }

            for (local, id, message) in errors {
                if let Some(e) = entries
                    .iter_mut()
                    .find(|e| e.local_id == Some(local) || id.is_some() && e.recording_id == id)
                {
                    e.last_error = Some(message);
                }
            }
        })
    }

    pub fn revision(&self) -> u64 {
        self.state.lock().unwrap().revision
    }

    pub fn reconcile_local(
        &self,
        device: Option<Id16>,
        locals: Vec<(Id16, Option<Id16>, u32, bool, u32)>,
    ) -> Result<(), HistoryError> {
        self.update_for_device(device, |index| {
            for (local, id, _, _, lost) in &locals {
                index.loss(*local, *id, *lost);
            }
            let entries = &mut index.entries;
            for row in entries.iter_mut() {
                row.pending_chunks = 0;
                row.pending_control = false;
            }

            // Resolve all local aliases before selecting metadata and collapsing rows.
            for (local, id, _, _, _) in &locals {
                if let Some(id) = id {
                    for row in entries.iter_mut().filter(|e| e.local_id == Some(*local)) {
                        row.recording_id = Some(*id);
                    }
                }
            }

            let mut groups: Vec<HistoryEntry> = vec![];

            for (local, id, pending, control, lost) in locals {
                let old = entries
                    .iter()
                    .find(|e| e.local_id == Some(local) || id.is_some() && e.recording_id == id)
                    .cloned();

                let mut row = old.unwrap_or(HistoryEntry {
                    local_id: Some(local),
                    recording_id: id,
                    summary: None,
                    share: private_share(),
                    pending_chunks: 0,
                    pending_control: false,
                    lost_chunks: 0,
                    last_error: None,
                    hidden: false,
                });

                row.local_id = Some(local);
                row.recording_id = id.or(row.recording_id);
                row.pending_chunks = pending;
                row.pending_control = control;
                row.lost_chunks = lost;

                if let Some(group) = groups
                    .iter_mut()
                    .find(|e| id.is_some() && e.recording_id == id)
                {
                    group.pending_chunks = group.pending_chunks.saturating_add(pending);
                    group.pending_control |= control;
                    group.lost_chunks = group.lost_chunks.saturating_add(lost);
                    group.hidden |= row.hidden;
                } else {
                    groups.push(row);
                }
            }

            for mut row in groups {
                let mut retained = vec![];

                for old in entries.drain(..) {
                    if row.local_id.is_some() && old.local_id == row.local_id
                        || row.recording_id.is_some() && old.recording_id == row.recording_id
                    {
                        row.hidden |= old.hidden;

                        if row.summary.is_none() && old.summary.is_some() {
                            row.summary = old.summary.clone();
                            row.share = old.share.clone();
                        }
                    } else {
                        retained.push(old)
                    }
                }

                retained.push(row);
                *entries = retained;
            }
        })
    }

    pub fn local(
        &self,
        local: Id16,
        id: Option<Id16>,
        pending: u32,
        control: bool,
        lost: u32,
    ) -> Result<(), HistoryError> {
        let device = self.state.lock().unwrap().index.device_id;
        self.update_for_device(device, |index| {
            index.loss(local, id, lost);
            let entries = &mut index.entries;
            let mut row = entries
                .iter()
                .find(|e| e.local_id == Some(local))
                .cloned()
                .unwrap_or(HistoryEntry {
                    local_id: Some(local),
                    recording_id: id,
                    summary: None,
                    share: private_share(),
                    pending_chunks: 0,
                    pending_control: false,
                    lost_chunks: 0,
                    last_error: None,
                    hidden: false,
                });

            row.recording_id = id.or(row.recording_id);
            row.pending_chunks = pending;
            row.pending_control = control;
            row.lost_chunks = row.lost_chunks.max(lost);
            entries.retain(|e| e.local_id != Some(local));

            if let Some(other) = entries
                .iter_mut()
                .find(|e| id.is_some() && e.recording_id == id)
            {
                other.hidden |= row.hidden;
                other.lost_chunks = other.lost_chunks.max(row.lost_chunks);
                other.pending_chunks = other.pending_chunks.saturating_add(row.pending_chunks);
                other.pending_control |= row.pending_control;
            } else {
                entries.push(row);
            }
        })
    }

    pub fn hide(&self, id: Id16) -> Result<(), HistoryError> {
        let mut s = self.state.lock().unwrap();
        let mut next = s.index.clone();

        let row = next
            .entries
            .iter_mut()
            .find(|e| e.recording_id == Some(id))
            .ok_or_else(|| {
                error(
                    HistoryErrorKind::NotFound,
                    "Recording not found in local history",
                    false,
                )
            })?;

        row.hidden = true;

        if next != s.index {
            self.store.save_history(&next).map_err(storage)?;
            s.index = next;
            s.revision += 1;
        }

        Ok(())
    }

    pub fn list(&self, offset: u32, limit: u32) -> HistoryList {
        let s = self.state.lock().unwrap();
        let mut rows = s
            .index
            .entries
            .iter()
            .filter(|e| !e.hidden)
            .cloned()
            .collect::<Vec<_>>();

        rows.sort_by(|a, b| {
            a.summary
                .is_some()
                .cmp(&b.summary.is_some())
                .then_with(|| {
                    b.summary
                        .as_ref()
                        .map(|s| s.recording.started_at_ms)
                        .cmp(&a.summary.as_ref().map(|s| s.recording.started_at_ms))
                })
                .then_with(|| {
                    b.recording_id
                        .or(b.local_id)
                        .cmp(&a.recording_id.or(a.local_id))
                })
        });

        let offset = offset as usize;
        let limit = limit.clamp(1, 100) as usize;
        let has_more = rows.len() > offset.saturating_add(limit);

        HistoryList {
            entries: rows.into_iter().skip(offset).take(limit).collect(),
            has_more,
            refreshing: s.refreshing,
            refresh_error: s.error.clone(),
        }
    }

    fn merge(index: &mut Index, item: HistoryListItem) {
        let id = item.summary.recording.id;

        index.unavailable.retain(|missing| *missing != id);

        let entries = &mut index.entries;

        if let Some(entry) = entries.iter_mut().find(|e| e.recording_id == Some(id))
            && entry.last_error.as_deref() == Some(UNAVAILABLE)
        {
            entry.last_error = None;
        }

        if let Some(e) = entries.iter_mut().find(|e| e.recording_id == Some(id)) {
            e.summary = Some(item.summary);
            e.share = item.share;
        } else {
            entries.push(HistoryEntry {
                local_id: None,
                recording_id: Some(id),
                summary: Some(item.summary),
                share: item.share,
                pending_chunks: 0,
                pending_control: false,
                lost_chunks: 0,
                last_error: None,
                hidden: false,
            });
        }
    }

    pub fn check_device(&self, device: Option<Id16>) -> Result<(), HistoryError> {
        if self.state.lock().unwrap().index.device_id != device {
            return Err(error(
                HistoryErrorKind::Unauthorized,
                "History device changed",
                true,
            ));
        }

        Ok(())
    }

    pub fn details(
        &self,
        device: Option<Id16>,
        recording: RecordingDetails,
    ) -> Result<HistoryDetails, HistoryError> {
        let mut s = self.state.lock().unwrap();

        if s.index.device_id != device {
            return Err(error(
                HistoryErrorKind::Unauthorized,
                "History device changed",
                true,
            ));
        }

        let id = recording.summary.recording.id;
        let mut next = s.index.clone();

        Self::merge(
            &mut next,
            HistoryListItem {
                summary: recording.summary.clone(),
                share: recording.share.clone(),
            },
        );

        if next != s.index {
            self.store.save_history(&next).map_err(storage)?;
            s.index = next;
            s.revision += 1;
        }

        let entry = s
            .index
            .entries
            .iter()
            .find(|e| e.recording_id == Some(id))
            .unwrap()
            .clone();

        Ok(HistoryDetails { recording, entry })
    }

    pub fn share(
        &self,
        device: Option<Id16>,
        id: Id16,
        share: ShareState,
    ) -> Result<(), HistoryError> {
        let mut s = self.state.lock().unwrap();

        if s.index.device_id != device {
            return Err(error(
                HistoryErrorKind::Unauthorized,
                "History device changed",
                true,
            ));
        }

        let mut next = s.index.clone();

        if let Some(e) = next.entries.iter_mut().find(|e| e.recording_id == Some(id)) {
            e.share = share;
        }

        if next != s.index {
            self.store.save_history(&next).map_err(storage)?;
            s.index = next;
            s.revision += 1;
        }

        Ok(())
    }

    pub fn begin(&self, repair: bool) -> bool {
        let mut s = self.state.lock().unwrap();

        if s.refreshing {
            s.repair_requested |= repair;
            return false;
        }

        if !repair
            && s.attempted
                .is_some_and(|t| t.elapsed() < Duration::from_secs(30))
        {
            return false;
        }

        s.refreshing = true;
        s.attempted = Some(Instant::now());
        s.revision += 1;

        true
    }

    pub async fn refresh(&self, api: ApiClient, repair: bool) {
        let device = api.device_id;
        let mut repair = repair;

        loop {
            let result = async {
                let mut before = None;
                let mut items = vec![];

                loop {
                    let page = api.history_page(before).await?;

                    items.extend(page.items);

                    if !repair || page.next_before.is_none() {
                        break;
                    }

                    if page.next_before == before
                        || before
                            .is_some_and(|old| page.next_before.is_some_and(|next| next >= old))
                    {
                        return Err(error(
                            HistoryErrorKind::InvalidData,
                            "Invalid history cursor",
                            false,
                        ));
                    }

                    before = page.next_before;
                }

                let mut missing = vec![];
                let mut detail_error = None;

                {
                    let ids = {
                        let mut state = self.state.lock().unwrap();
                        let mut ids = state
                            .index
                            .entries
                            .iter()
                            .filter(|e| {
                                let unavailable = e
                                    .recording_id
                                    .is_some_and(|id| state.index.unavailable.contains(&id));

                                if repair {
                                    return !items.iter().any(|item| {
                                        Some(item.summary.recording.id) == e.recording_id
                                    });
                                }

                                if unavailable {
                                    return false;
                                }

                                e.pending_control
                                    || e.pending_chunks > 0
                                    || e.summary
                                        .as_ref()
                                        .is_none_or(|s| s.recording.result.is_none())
                            })
                            .filter_map(|e| e.recording_id)
                            .collect::<Vec<_>>();

                        ids.sort_unstable();
                        ids.dedup();

                        if !repair && !ids.is_empty() {
                            let start = state
                                .details_after
                                .and_then(|last| ids.iter().position(|id| *id > last))
                                .unwrap_or(0);

                            ids.rotate_left(start);
                        }

                        if !repair {
                            ids.truncate(20);
                            state.details_after = ids.last().copied();
                        }
                        ids
                    };

                    for id in ids {
                        match api.history_details(id).await {
                            Ok(details) => items.push(HistoryListItem {
                                summary: details.summary,
                                share: details.share,
                            }),
                            Err(failure) if failure.kind == HistoryErrorKind::NotFound => {
                                missing.push(id)
                            }
                            Err(failure) => {
                                if detail_error.is_none() {
                                    detail_error = Some(failure);
                                }
                            }
                        }
                    }
                }

                let mut s = self.state.lock().unwrap();

                if s.index.device_id != device {
                    return Err(error(
                        HistoryErrorKind::Unauthorized,
                        "History device changed",
                        true,
                    ));
                }

                let mut next = s.index.clone();

                for item in items {
                    Self::merge(&mut next, item);
                }

                for id in missing {
                    if !next.unavailable.contains(&id) {
                        next.unavailable.push(id);
                    }

                    if let Some(entry) =
                        next.entries.iter_mut().find(|e| e.recording_id == Some(id))
                    {
                        entry.last_error = Some(UNAVAILABLE.into());
                    }
                }

                if next != s.index {
                    self.store.save_history(&next).map_err(storage)?;
                    s.index = next;
                    s.revision += 1;
                }

                if let Some(failure) = detail_error {
                    return Err(failure);
                }

                Ok(())
            }
            .await;

            let mut s = self.state.lock().unwrap();

            s.refreshing = false;
            s.error = result.err().map(|e| e.message);
            s.revision += 1;

            if s.repair_requested && s.index.device_id == device {
                s.repair_requested = false;
                s.refreshing = true;
                s.attempted = Some(Instant::now());
                repair = true;
                drop(s);
                continue;
            }

            s.repair_requested = false;

            break;
        }
    }
}
