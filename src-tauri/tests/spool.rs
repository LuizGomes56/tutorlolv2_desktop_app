use std::path::PathBuf;
use tutorlolv2_desktop_app::spool::{Spool, Target, new_local_id};

fn dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("tutorlolv2-spool-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn local_ids_sort_by_time() {
    assert!(new_local_id(1, [0xFF; 10]) < new_local_id(2, [0; 10]));
    let id = new_local_id(0x0123_4567_89AB, [0xFF; 10]);
    assert_eq!(&id[..6], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);
    assert_eq!(id[6] >> 4, 7, "version 7");
    assert_eq!(id[8] >> 6, 0b10, "RFC 4122 variant");
}

#[test]
fn pending_is_ordered_oldest_session_then_seq() {
    let spool = Spool::new(dir("order"), Spool::DEFAULT_MAX_BYTES);
    let old = new_local_id(1_000, [1; 10]);
    let new = new_local_id(2_000, [0; 10]);
    spool.put(new, 0, b"n0").unwrap();
    spool.put(old, 1, b"o1").unwrap();
    spool.put(old, 0, b"o0").unwrap();
    spool.put(old, 10, b"o10").unwrap();
    spool
        .set_target(
            old,
            &Target {
                recording_id: [9; 16],
                seq_offset: 3,
            },
        )
        .unwrap();

    let pending = spool.pending();
    let order: Vec<_> = pending
        .iter()
        .map(|c| (c.local_id == old, c.local_seq))
        .collect();
    assert_eq!(order, [(true, 0), (true, 1), (true, 10), (false, 0)]);
    assert_eq!(
        pending[0].target,
        Some(Target {
            recording_id: [9; 16],
            seq_offset: 3
        })
    );
    assert_eq!(pending[3].target, None);
    assert_eq!(std::fs::read(&pending[2].path).unwrap(), b"o10");
}

#[test]
fn put_over_budget_drops_oldest_chunks_of_oldest_session() {
    let spool = Spool::new(dir("budget"), 3 * 1024 + 64);
    let id = new_local_id(1_000, [0; 10]);
    for seq in 0..3 {
        assert_eq!(spool.put(id, seq, &[seq as u8; 1024]).unwrap(), 0);
    }
    assert_eq!(spool.put(id, 3, &[3; 1024]).unwrap(), 1);
    let seqs: Vec<_> = spool.pending().iter().map(|c| c.local_seq).collect();
    assert_eq!(seqs, [1, 2, 3]);
}

#[test]
fn done_and_reject_remove_from_pending() {
    let root = dir("done");
    let spool = Spool::new(root.clone(), Spool::DEFAULT_MAX_BYTES);
    let id = new_local_id(1_000, [0; 10]);
    spool.put(id, 0, b"a").unwrap();
    spool.put(id, 1, b"b").unwrap();
    let pending = spool.pending();
    spool.done(&pending[0]).unwrap();
    spool.reject(&pending[1]).unwrap();
    assert!(spool.pending().is_empty());
    assert!(
        pending[1].path.with_extension("rejected").exists(),
        "rejected chunks are kept for diagnosis"
    );
}

#[test]
fn prune_removes_finished_sessions_but_keeps_the_active_one() {
    let root = dir("prune");
    let spool = Spool::new(root.clone(), Spool::DEFAULT_MAX_BYTES);
    let finished = new_local_id(1_000, [0; 10]);
    let active = new_local_id(2_000, [0; 10]);
    let waiting = new_local_id(3_000, [0; 10]);
    for id in [finished, active] {
        spool
            .set_target(
                id,
                &Target {
                    recording_id: [1; 16],
                    seq_offset: 0,
                },
            )
            .unwrap();
    }
    spool.put(waiting, 0, b"x").unwrap();
    spool.prune(Some(active)).unwrap();
    let dirs = std::fs::read_dir(&root).unwrap().count();
    assert_eq!(dirs, 2, "only the finished session's folder is removed");
    assert_eq!(spool.pending().len(), 1);
}

#[test]
fn target_of_reads_the_session_target() {
    let spool = Spool::new(dir("target_of"), Spool::DEFAULT_MAX_BYTES);
    let id = new_local_id(1_000, [0; 10]);
    assert_eq!(spool.target_of(id), None);
    spool
        .set_target(
            id,
            &Target {
                recording_id: [4; 16],
                seq_offset: 2,
            },
        )
        .unwrap();
    assert_eq!(
        spool.target_of(id),
        Some(Target {
            recording_id: [4; 16],
            seq_offset: 2
        })
    );
}

fn start_request(t: f32) -> tutorlolv2_protocol::StartSession {
    tutorlolv2_protocol::StartSession {
        probe: tutorlolv2_protocol::Probe {
            active_riot_id: "Me#BR1".into(),
            game_mode: tutorlolv2_protocol::GameMode::Classic,
            map_number: 11,
            roster: vec![],
            game_time: t,
            events: vec![],
            riot_game: None,
        },
        region: None,
        lcu: None,
        app_version: "0.3.0".into(),
    }
}

fn end_request() -> tutorlolv2_protocol::EndRecording {
    tutorlolv2_protocol::EndRecording {
        reason: tutorlolv2_protocol::EndReason::Superseded,
        result: None,
        final_game_time: None,
    }
}

#[test]
fn outbox_lists_pending_calls_oldest_session_first() {
    let spool = Spool::new(dir("outbox"), Spool::DEFAULT_MAX_BYTES);
    let old = new_local_id(1_000, [1; 10]);
    let new = new_local_id(2_000, [0; 10]);
    spool.set_start(new, &start_request(2.0)).unwrap();
    spool.set_start(old, &start_request(1.0)).unwrap();
    spool.set_end(old, &end_request()).unwrap();

    let outbox = spool.outbox();
    assert_eq!(
        outbox.iter().map(|e| e.local_id).collect::<Vec<_>>(),
        [old, new]
    );
    assert_eq!(
        outbox[0].start.as_ref().map(|s| s.probe.game_time),
        Some(1.0)
    );
    assert_eq!(outbox[0].end, Some(end_request()));
    assert_eq!(outbox[1].end, None);

    spool.clear_start(old).unwrap();
    spool.clear_end(old).unwrap();
    assert_eq!(
        spool
            .outbox()
            .iter()
            .map(|e| e.local_id)
            .collect::<Vec<_>>(),
        [new]
    );
}

#[test]
fn the_first_pending_call_wins() {
    let spool = Spool::new(dir("first_wins"), Spool::DEFAULT_MAX_BYTES);
    let id = new_local_id(1_000, [0; 10]);
    spool.set_start(id, &start_request(1.0)).unwrap();
    spool.set_start(id, &start_request(9.0)).unwrap();
    assert_eq!(
        spool.outbox()[0].start.as_ref().map(|s| s.probe.game_time),
        Some(1.0)
    );
}

#[test]
fn prune_keeps_sessions_with_pending_calls() {
    let root = dir("prune_outbox");
    let spool = Spool::new(root.clone(), Spool::DEFAULT_MAX_BYTES);
    let id = new_local_id(1_000, [0; 10]);
    spool.set_end(id, &end_request()).unwrap();
    spool.prune(None).unwrap();
    assert_eq!(spool.outbox().len(), 1);
}

#[test]
fn rejected_chunks_count_toward_budget() {
    let spool = Spool::new(dir("rejected_budget"), 2048);
    let id = new_local_id(1, [0; 10]);
    spool.put(id, 0, &[0; 1024]).unwrap();
    let old = spool.pending()[0].clone();
    spool.reject(&old).unwrap();
    spool.put(id, 1, &[1; 1536]).unwrap();
    assert!(!old.path.with_extension("rejected").exists());
    assert_eq!(spool.pending().len(), 1);
}

#[test]
fn oversize_chunk_does_not_evict_existing_data() {
    let spool = Spool::new(dir("oversize"), 1024);
    let id = new_local_id(1, [0; 10]);
    spool.put(id, 0, &[0; 512]).unwrap();
    assert!(spool.put(id, 1, &[1; 1025]).is_err());
    assert_eq!(spool.pending().len(), 1);
}

#[test]
fn recovery_reserves_pending_and_inflight_sequences_and_reuses_target() {
    let root = dir("reservations");
    let spool = Spool::new(root.clone(), Spool::DEFAULT_MAX_BYTES);
    let old = new_local_id(1, [0; 10]);
    let new = new_local_id(2, [0; 10]);
    let recording_id = [9; 16];
    spool
        .set_target(
            old,
            &Target {
                recording_id,
                seq_offset: 0,
            },
        )
        .unwrap();
    spool.put(old, 5, b"old").unwrap();
    spool.reserve_chunk(old, 6).unwrap(); // Compression is still running.
    let assigned = spool
        .assign_target(
            new,
            Target {
                recording_id,
                seq_offset: 5,
            },
        )
        .unwrap();
    assert_eq!(assigned.seq_offset, 7);
    let reopened = Spool::new(root, Spool::DEFAULT_MAX_BYTES);
    assert_eq!(
        reopened
            .assign_target(
                new,
                Target {
                    recording_id,
                    seq_offset: 100
                }
            )
            .unwrap(),
        assigned
    );
}

#[test]
fn result_outbox_survives_restart_and_pruning_until_acknowledged() {
    use tutorlolv2_protocol::{GameResult, RecordingResult};
    let root = dir("result_outbox");
    let spool = Spool::new(root.clone(), Spool::DEFAULT_MAX_BYTES);
    let id = new_local_id(1, [0; 10]);
    let result = RecordingResult {
        result: GameResult::Win,
        game_length_s: Some(1200),
        riot_game: None,
    };
    spool.set_result(id, &result).unwrap();
    let reopened = Spool::new(root, Spool::DEFAULT_MAX_BYTES);
    reopened.prune(None).unwrap();
    assert_eq!(reopened.outbox()[0].result, Some(result));
    reopened.clear_result(id).unwrap();
    assert!(reopened.outbox().is_empty());
}

#[test]
fn impossible_write_preserves_existing_chunks_and_metadata_is_bounded() {
    let spool = Spool::new(dir("metadata_budget"), 128);
    let id = new_local_id(1, [0; 10]);
    spool
        .set_target(
            id,
            &Target {
                recording_id: [9; 16],
                seq_offset: 0,
            },
        )
        .unwrap();
    spool.put(id, 0, &[0; 64]).unwrap();
    assert!(spool.put(id, 1, &[1; 128]).is_err());
    assert_eq!(spool.pending().len(), 1);
    let huge = tutorlolv2_protocol::RecordingResult {
        result: tutorlolv2_protocol::GameResult::Win,
        game_length_s: Some(10),
        riot_game: None,
    };
    for n in 2..100 {
        if spool.set_result(new_local_id(n, [0; 10]), &huge).is_err() {
            return;
        }
    }
    panic!("Metadata writes bypassed the budget");
}

#[test]
fn prune_expires_rejected_chunks_and_cleans_interrupted_writes() {
    use std::{
        fs::{File, FileTimes},
        time::{Duration, SystemTime},
    };
    let spool = Spool::new(dir("expire"), Spool::DEFAULT_MAX_BYTES);
    let id = new_local_id(1, [0; 10]);
    spool.put(id, 0, b"old").unwrap();
    let chunk = spool.pending()[0].clone();
    spool.reject(&chunk).unwrap();
    let path = chunk.path.with_extension("rejected");
    File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_times(
            FileTimes::new().set_modified(SystemTime::now() - Duration::from_secs(8 * 86400)),
        )
        .unwrap();
    let tmp = path.with_extension("bin.tmp");
    std::fs::write(&tmp, b"interrupted").unwrap();
    spool.prune(Some(id)).unwrap();
    assert!(!path.exists());
    assert!(!tmp.exists());
}

fn directory_bytes(root: &std::path::Path) -> u64 {
    std::fs::read_dir(root)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                directory_bytes(&path)
            } else {
                path.metadata().unwrap().len()
            }
        })
        .sum()
}

#[test]
fn overflow_accounts_for_loss_journal_before_deleting_data() {
    let root = dir("overflow_journal_impossible");
    let spool = Spool::new(root.clone(), 1025);
    let id = new_local_id(1, [0; 10]);
    spool.put(id, 0, &[7; 1024]).unwrap();
    assert!(spool.put(id, 1, &[8; 1024]).is_err());
    assert_eq!(directory_bytes(&root), 1025);
    assert_eq!(std::fs::read(&spool.pending()[0].path).unwrap(), [7; 1024]);
    let reopened = Spool::new(root.clone(), 1025);
    assert_eq!(reopened.loss_count(id), 0);
    assert_eq!(reopened.put(id, 1, &[8; 1000]).unwrap(), 1);
    assert!(directory_bytes(&root) <= 1025);
    assert_eq!(reopened.loss_count(id), 1);
}

#[test]
fn overflow_budget_includes_retained_journal_growth_across_restarts() {
    let root = dir("overflow_journal_growth");
    let id = new_local_id(1, [0; 10]);
    let spool = Spool::new(root.clone(), 1050);
    for seq in 0..30 {
        if spool.put(id, seq, &[seq as u8; 1024]).is_err() {
            assert!(directory_bytes(&root) <= 1050);
            let pending = spool.pending();
            assert_eq!(pending.len(), 1);
            let reopened = Spool::new(root.clone(), 1050);
            assert_eq!(reopened.loss_count(id), seq - 1);
            assert_eq!(reopened.pending(), pending);
            return;
        }
        assert!(directory_bytes(&root) <= 1050, "over budget at seq {seq}");
    }
    panic!("Permanent loss metadata must eventually reject writes");
}

#[test]
fn overflow_keeps_chunk_when_loss_intent_cannot_be_synced() {
    let root = dir("overflow_intent_failure");
    let spool = Spool::new(root.clone(), 1100);
    let id = new_local_id(1, [0; 10]);
    let target = Target {
        recording_id: [9; 16],
        seq_offset: 4,
    };
    spool.bind_device(id, Some([3; 16])).unwrap();
    spool.set_target(id, &target).unwrap();
    spool.put(id, 0, &[7; 1024]).unwrap();
    let chunk = spool.pending()[0].clone();
    let blocked_temp = chunk.path.parent().unwrap().join("loss.bin.tmp");
    std::fs::create_dir(&blocked_temp).unwrap();
    assert!(spool.put(id, 1, &[8; 1024]).is_err());
    assert_eq!(std::fs::read(&chunk.path).unwrap(), [7; 1024]);
    assert_eq!(spool.loss_count(id), 0);
    std::fs::remove_dir(blocked_temp).unwrap();
    let reopened = Spool::new(root.clone(), 1100);
    assert_eq!(reopened.put(id, 1, &[8; 1024]).unwrap(), 1);
    assert_eq!(reopened.target_of(id), Some(target));
    assert!(reopened.bind_device(id, Some([3; 16])).unwrap());
    assert!(!reopened.bind_device(id, Some([4; 16])).unwrap());
    assert_eq!(reopened.loss_count(id), 1);
    assert!(directory_bytes(&root) <= 1100);
    let next = new_local_id(2, [0; 10]);
    assert_eq!(reopened.assign_target(next, target).unwrap().seq_offset, 6);
}
