use tutorlolv2_desktop_app::{history::History, store::Store};
#[test]
fn provisional_assignment_and_hide_survive_restart() {
    let dir = std::env::temp_dir().join(format!("tl-history-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.local([2; 16], None, 2, true, 0).unwrap();
    assert_eq!(history.list(0, 50).entries[0].summary, None);
    history.local([2; 16], Some([3; 16]), 1, false, 2).unwrap();
    history.hide([3; 16]).unwrap();
    assert!(
        History::new(Store::new(dir.clone()), Some([1; 16]))
            .list(0, 50)
            .entries
            .is_empty()
    );
    assert!(
        History::new(Store::new(dir.clone()), Some([9; 16]))
            .list(0, 50)
            .entries
            .is_empty()
    );
    let _ = std::fs::remove_dir_all(dir);
}
#[test]
fn failed_hide_does_not_change_memory() {
    let dir = std::env::temp_dir().join(format!("tl-history-fail-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), None);
    history.local([2; 16], Some([3; 16]), 0, false, 0).unwrap();
    std::fs::create_dir(dir.join("history.bin.tmp")).unwrap();
    assert!(history.hide([3; 16]).is_err());
    assert_eq!(history.list(0, 50).entries.len(), 1);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn rejected_loss_survives_diagnostic_eviction_and_restart() {
    use tutorlolv2_desktop_app::spool::Spool;
    let dir = std::env::temp_dir().join(format!("tl-loss-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let spool = Spool::new(dir.clone(), 1024);
    spool.put([1; 16], 0, b"data").unwrap();
    let chunk = spool.pending().remove(0);
    spool.reject(&chunk).unwrap();
    assert_eq!(spool.loss_count([1; 16]), 1);
    std::fs::remove_file(chunk.path.with_extension("rejected")).unwrap();
    assert_eq!(Spool::new(dir.clone(), 1024).loss_count([1; 16]), 1);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn resumed_sessions_aggregate_without_recounting_or_losing_hide() {
    let dir = std::env::temp_dir().join(format!("tl-resume-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    for _ in 0..3 {
        history
            .reconcile_local(
                Some([1; 16]),
                vec![
                    ([2; 16], Some([4; 16]), 1, true, 2),
                    ([3; 16], Some([4; 16]), 2, false, 3),
                ],
            )
            .unwrap();
    }
    let row = history.list(0, 50).entries.remove(0);
    assert_eq!(row.pending_chunks, 3);
    assert_eq!(row.lost_chunks, 5);
    history.hide([4; 16]).unwrap();
    history
        .reconcile_local(
            Some([1; 16]),
            vec![
                ([2; 16], Some([4; 16]), 0, false, 2),
                ([3; 16], Some([4; 16]), 0, false, 3),
            ],
        )
        .unwrap();
    assert!(history.list(0, 50).entries.is_empty());
    let _ = std::fs::remove_dir_all(dir);
}
#[test]
fn spool_identity_never_rebinds_old_pending_work() {
    use tutorlolv2_desktop_app::spool::Spool;
    let dir = std::env::temp_dir().join(format!("tl-owner-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let spool = Spool::new(dir.clone(), 1024);
    assert!(spool.bind_device([1; 16], Some([2; 16])).unwrap());
    assert!(!spool.bind_device([1; 16], Some([3; 16])).unwrap());
    assert!(!spool.bind_device([1; 16], None).unwrap());
    let _ = std::fs::remove_dir_all(dir);
}

fn server(
    responses: Vec<(u16, &'static str, Vec<u8>)>,
) -> (String, std::thread::JoinHandle<Vec<String>>) {
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let handle = std::thread::spawn(move || {
        let mut requests = vec![];
        for (status, extra, bytes) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                .unwrap();
            let mut buf = [0; 8192];
            let n = stream.read(&mut buf).unwrap();
            requests.push(String::from_utf8_lossy(&buf[..n]).to_string());
            write!(stream,"HTTP/1.1 {status} Response\r\nContent-Length: {}\r\nConnection: close\r\n{extra}\r\n",bytes.len()).unwrap();
            stream.write_all(&bytes).unwrap();
        }
        requests
    });
    (base, handle)
}
fn api(base: String) -> tutorlolv2_desktop_app::uploader::ApiClient {
    tutorlolv2_desktop_app::uploader::ApiClient::with_base_url(
        base,
        Some(&tutorlolv2_protocol::DeviceCredentials {
            device_id: [1; 16],
            device_secret: [2; 32],
        }),
    )
}
#[tokio::test]
async fn owner_frame_transport_decompresses_exactly_once_and_keeps_auth_native() {
    use std::io::Write;
    let raw = b"bounded bincode frame bytes";
    let mut compressed = vec![];
    {
        let mut writer = brotli::CompressorWriter::new(&mut compressed, 4096, 4, 22);
        writer.write_all(raw).unwrap();
    }
    let (base, worker) = server(vec![(200, "Content-Encoding: br\r\n", compressed)]);
    assert_eq!(api(base).history_frames([3; 16], 7).await.unwrap(), raw);
    let requests = worker.join().unwrap();
    assert!(
        requests[0]
            .starts_with("GET /api/me/recordings/03030303-0303-0303-0303-030303030303/frames/7")
    );
    assert!(
        requests[0]
            .to_lowercase()
            .contains("authorization: device ")
    );
}
#[tokio::test]
async fn owner_metadata_rejects_trailing_bytes_and_returns_typed_missing() {
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let mut body = bincode::encode_to_vec(
        HistoryPage {
            items: vec![],
            next_before: None,
        },
        BINCODE_CONFIG,
    )
    .unwrap();
    body.push(9);
    let (base, worker) = server(vec![(200, "", body), (404, "", vec![])]);
    let api = api(base);
    assert_eq!(
        api.history_page(None).await.unwrap_err().kind,
        HistoryErrorKind::InvalidData
    );
    assert_eq!(
        api.history_details([3; 16]).await.unwrap_err().kind,
        HistoryErrorKind::NotFound
    );
    worker.join().unwrap();
}
#[test]
fn every_history_operation_requires_main_window_and_ids_are_checked() {
    use tutorlolv2_desktop_app::history::{authorize, parse_id};
    assert!(authorize("main").is_ok());
    assert!(authorize("overlay").is_err());
    assert!(parse_id("123").is_err());
    assert!(parse_id("03030303-0303-0303-0303-030303030303").is_ok());
}
#[test]
fn failed_refresh_is_coalesced_from_attempt_start() {
    let dir = std::env::temp_dir().join(format!("tl-coalesce-{}", std::process::id()));
    let history = History::new(Store::new(dir), None);
    assert!(history.begin(false));
    assert!(!history.begin(false));
    assert!(!history.begin(true));
}

fn item(id: u8) -> tutorlolv2_protocol::HistoryListItem {
    use tutorlolv2_protocol::*;
    HistoryListItem {
        summary: HistorySummary {
            recording: RecordingSummary {
                id: [id; 16],
                champion: "ahri".into(),
                game_mode: GameMode::Classic,
                queue_id: Some(420),
                active_riot_id: "test#1".into(),
                started_at_ms: 1000,
                first_game_time: 0.0,
                last_game_time: 60.0,
                game_length_s: None,
                result: Some(GameResult::Unknown),
                is_custom: Some(false),
                is_practice: false,
            },
            ended_at_ms: Some(2000),
            end_reason: None,
            coverage_s: 60,
            kills: 1,
            deaths: 2,
            assists: 3,
            creep_score: 4,
            level: 5,
        },
        share: tutorlolv2_desktop_app::sharing::private_share(),
    }
}
#[tokio::test]
async fn repair_traverses_every_page_preserves_hidden_and_never_publishes_partial_network_success()
{
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let page = |id, next| {
        bincode::encode_to_vec(
            HistoryPage {
                items: vec![item(id)],
                next_before: next,
            },
            BINCODE_CONFIG,
        )
        .unwrap()
    };
    let (base, worker) = server(vec![
        (200, "", page(4, Some([4; 16]))),
        (200, "", page(3, None)),
    ]);
    let dir = std::env::temp_dir().join(format!("tl-repair-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.local([4; 16], Some([4; 16]), 2, true, 0).unwrap();
    history.hide([4; 16]).unwrap();
    assert!(history.begin(true));
    history.refresh(api(base), true).await;
    assert_eq!(worker.join().unwrap().len(), 2);
    let rows = history.list(0, 50);
    assert_eq!(rows.entries.len(), 1);
    assert_eq!(rows.entries[0].recording_id, Some([3; 16]));
    let (base, worker) = server(vec![(200, "", page(9, Some([9; 16]))), (500, "", vec![])]);
    assert!(history.begin(true));
    history.refresh(api(base), true).await;
    worker.join().unwrap();
    let rows = history.list(0, 50);
    assert!(rows.refresh_error.is_some());
    assert_eq!(rows.entries.len(), 1);
    assert_eq!(rows.entries[0].recording_id, Some([3; 16]));
    let _ = std::fs::remove_dir_all(dir);
}
#[test]
fn repeated_corruption_keeps_each_diagnostic() {
    let dir = std::env::temp_dir().join(format!("tl-corrupt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for _ in 0..3 {
        std::fs::write(dir.join("history.bin"), b"bad").unwrap();
        let _ = History::new(Store::new(dir.clone()), None);
    }
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 3);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn two_offline_rows_collapse_when_both_resume_the_same_recording() {
    let dir = std::env::temp_dir().join(format!("tl-offline-resume-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history
        .reconcile_local(
            Some([1; 16]),
            vec![([2; 16], None, 1, true, 0), ([3; 16], None, 2, true, 0)],
        )
        .unwrap();
    history
        .reconcile_local(
            Some([1; 16]),
            vec![
                ([2; 16], Some([4; 16]), 1, false, 0),
                ([3; 16], Some([4; 16]), 2, false, 0),
            ],
        )
        .unwrap();
    let rows = history.list(0, 50).entries;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].pending_chunks, 3);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn explicit_repair_is_queued_behind_an_active_normal_refresh() {
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let page = |id, next| {
        bincode::encode_to_vec(
            HistoryPage {
                items: vec![item(id)],
                next_before: next,
            },
            BINCODE_CONFIG,
        )
        .unwrap()
    };
    let (base, worker) = server(vec![
        (200, "", page(5, None)),
        (200, "", page(4, Some([4; 16]))),
        (200, "", page(3, None)),
    ]);
    let dir = std::env::temp_dir().join(format!("tl-queued-repair-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    assert!(history.begin(false));
    assert!(!history.begin(true));
    history.refresh(api(base), false).await;
    assert_eq!(history.list(0, 50).entries.len(), 3);
    worker.join().unwrap();
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn failed_rejection_intent_does_not_become_loss_after_successful_upload() {
    use tutorlolv2_desktop_app::spool::Spool;
    let dir = std::env::temp_dir().join(format!("tl-loss-intent-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let spool = Spool::new(dir.clone(), 1024);
    spool.put([1; 16], 0, b"data").unwrap();
    let chunk = spool.pending().remove(0);
    std::fs::create_dir(chunk.path.with_extension("rejected")).unwrap();
    assert!(spool.reject(&chunk).is_err());
    assert_eq!(spool.loss_count([1; 16]), 0);
    spool.done(&chunk).unwrap();
    assert_eq!(spool.loss_count([1; 16]), 0);
    let _ = std::fs::remove_dir_all(dir);
}
#[test]
fn stale_device_details_cannot_publish_into_replacement_identity() {
    use tutorlolv2_protocol::*;
    let dir = std::env::temp_dir().join(format!("tl-stale-device-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.bind(Some([2; 16])).unwrap();
    let source = item(4);
    let result = history.details(
        Some([1; 16]),
        RecordingDetails {
            summary: source.summary,
            chunks: vec![],
            share: source.share,
            frames_format: 1,
        },
    );
    assert_eq!(result.unwrap_err().kind, HistoryErrorKind::Unauthorized);
    assert!(history.list(0, 50).entries.is_empty());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn assigned_spool_without_surviving_credentials_is_never_adopted_by_new_device() {
    use tutorlolv2_desktop_app::spool::{Spool, Target};
    let dir = std::env::temp_dir().join(format!("tl-orphan-spool-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let spool = Spool::new(dir.clone(), 1024);
    spool
        .set_target(
            [1; 16],
            &Target {
                recording_id: [8; 16],
                seq_offset: 0,
            },
        )
        .unwrap();
    assert!(!spool.bind_device([1; 16], None).unwrap());
    assert!(!spool.bind_device([1; 16], Some([2; 16])).unwrap());
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn bounded_unknown_result_refresh_does_not_starve_later_recordings() {
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let empty = bincode::encode_to_vec(
        HistoryPage {
            items: vec![],
            next_before: None,
        },
        BINCODE_CONFIG,
    )
    .unwrap();
    let mut responses = vec![];
    for ids in [
        (1..=20).collect::<Vec<u8>>(),
        (21..=21).chain(1..=19).collect(),
    ] {
        responses.push((200, "", empty.clone()));
        for id in ids {
            let mut source = item(id);
            source.summary.recording.result = None;
            let body = bincode::encode_to_vec(
                RecordingDetails {
                    summary: source.summary,
                    share: source.share,
                    chunks: vec![],
                    frames_format: 1,
                },
                BINCODE_CONFIG,
            )
            .unwrap();
            responses.push((200, "", body));
        }
    }
    let (base, worker) = server(responses);
    let api = api(base);
    let dir = std::env::temp_dir().join(format!("tl-refresh-fair-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history
        .reconcile_local(
            Some([1; 16]),
            (1..=21)
                .map(|id| ([id; 16], Some([id; 16]), 0, false, 0))
                .collect(),
        )
        .unwrap();
    history.refresh(api.clone(), false).await;
    history.refresh(api, false).await;
    assert!(history.list(0, 100).entries.iter().all(|e| {
        e.summary
            .as_ref()
            .is_some_and(|s| s.recording.id == e.recording_id.unwrap())
    }));
    let requests = worker.join().unwrap();
    assert!(
        requests[22].starts_with("GET /api/me/recordings/15151515-1515-1515-1515-151515151515 ")
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stale_details_404_does_not_poison_refresh_and_unavailability_survives_worker_and_restart()
{
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let page = bincode::encode_to_vec(
        HistoryPage {
            items: vec![item(3)],
            next_before: None,
        },
        BINCODE_CONFIG,
    )
    .unwrap();
    let details = |id| {
        let source = item(id);
        bincode::encode_to_vec(
            RecordingDetails {
                summary: source.summary,
                share: source.share,
                chunks: vec![],
                frames_format: 1,
            },
            BINCODE_CONFIG,
        )
        .unwrap()
    };
    let (base, worker) = server(vec![
        (200, "", page.clone()),
        (404, "", vec![]),
        (200, "", details(2)),
    ]);
    let dir = std::env::temp_dir().join(format!("tl-stale-refresh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history
        .reconcile_local(
            Some([1; 16]),
            vec![
                ([1; 16], Some([1; 16]), 2, true, 1),
                ([2; 16], Some([2; 16]), 0, false, 0),
            ],
        )
        .unwrap();
    history.refresh(api(base), false).await;
    let rows = history.list(0, 100);
    assert_eq!(rows.entries.len(), 3);
    worker.join().unwrap();
    assert!(
        rows.entries
            .iter()
            .find(|e| e.recording_id == Some([2; 16]))
            .unwrap()
            .summary
            .is_some()
    );
    let stale = rows
        .entries
        .iter()
        .find(|e| e.recording_id == Some([1; 16]))
        .unwrap();
    assert_eq!(stale.pending_chunks, 2);
    assert!(
        stale
            .last_error
            .as_ref()
            .is_some_and(|e| e.contains("unavailable"))
    );
    history.errors(Some([1; 16]), vec![]).unwrap();
    drop(history);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.errors(Some([1; 16]), vec![]).unwrap();
    assert!(
        history
            .list(0, 100)
            .entries
            .iter()
            .find(|e| e.recording_id == Some([1; 16]))
            .unwrap()
            .last_error
            .is_some()
    );
    let (base, worker) = server(vec![(200, "", page.clone())]);
    history.refresh(api(base), false).await;
    assert!(history.list(0, 100).refresh_error.is_none());
    assert_eq!(worker.join().unwrap().len(), 1);
    let (base, worker) = server(vec![(200, "", page), (200, "", details(1))]);
    history.refresh(api(base), true).await;
    assert_eq!(worker.join().unwrap().len(), 2);
    let rows = history.list(0, 100);
    let recovered = rows
        .entries
        .iter()
        .find(|e| e.recording_id == Some([1; 16]))
        .unwrap();
    assert!(recovered.summary.is_some());
    assert!(recovered.last_error.is_none());
    assert_eq!(recovered.pending_chunks, 2);
    let _ = std::fs::remove_dir_all(dir);
}
#[tokio::test]
async fn transient_details_failure_keeps_valid_newest_page_and_reports_refresh_error() {
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let page = bincode::encode_to_vec(
        HistoryPage {
            items: vec![item(3)],
            next_before: None,
        },
        BINCODE_CONFIG,
    )
    .unwrap();
    let (base, worker) = server(vec![(200, "", page), (500, "", vec![])]);
    let dir = std::env::temp_dir().join(format!("tl-transient-refresh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.local([1; 16], Some([1; 16]), 0, false, 0).unwrap();
    history.refresh(api(base), false).await;
    worker.join().unwrap();
    let rows = history.list(0, 100);
    assert_eq!(rows.entries.len(), 2);
    assert!(rows.refresh_error.is_some());
    assert!(
        rows.entries
            .iter()
            .find(|e| e.recording_id == Some([1; 16]))
            .unwrap()
            .last_error
            .is_none()
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn initial_m3_index_migrates_without_losing_local_hide_or_pending_state() {
    use tutorlolv2_protocol::*;
    let dir = std::env::temp_dir().join(format!("tl-index-compat-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let store = Store::new(dir.clone());
    let row = HistoryEntry {
        local_id: Some([1; 16]),
        recording_id: Some([2; 16]),
        summary: None,
        share: tutorlolv2_desktop_app::sharing::private_share(),
        pending_chunks: 3,
        pending_control: true,
        lost_chunks: 2,
        last_error: None,
        hidden: true,
    };
    // Bincode's tuple and struct field ordering are the same for this old two-field payload.
    store.save_history(&(Some([1u8; 16]), vec![row])).unwrap();
    let history = History::new(store, Some([1; 16]));
    assert!(history.list(0, 100).entries.is_empty());
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    history
        .reconcile_local(Some([1; 16]), vec![([1; 16], Some([2; 16]), 3, true, 2)])
        .unwrap();
    assert!(history.list(0, 100).entries.is_empty());
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn adopting_server_history_attaches_local_id_without_collapsing_unrelated_rows() {
    use tutorlolv2_desktop_app::BINCODE_CONFIG;
    use tutorlolv2_protocol::*;
    let page = bincode::encode_to_vec(
        HistoryPage {
            items: vec![item(10), item(11), item(12)],
            next_before: None,
        },
        BINCODE_CONFIG,
    )
    .unwrap();
    let (base, worker) = server(vec![(200, "", page)]);
    let dir = std::env::temp_dir().join(format!("tl-adopt-server-row-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.refresh(api(base), true).await;
    worker.join().unwrap();
    history.hide([12; 16]).unwrap();
    assert_eq!(history.list(0, 50).entries.len(), 2);

    for _ in 0..2 {
        history
            .reconcile_local(Some([1; 16]), vec![([99; 16], Some([10; 16]), 3, true, 2)])
            .unwrap();
        let restarted = History::new(Store::new(dir.clone()), Some([1; 16]));
        let rows = restarted.list(0, 50).entries;
        assert_eq!(
            rows.len(),
            2,
            "unrelated visible rows and hidden flags must survive"
        );
        let adopted = rows
            .iter()
            .find(|row| row.recording_id == Some([10; 16]))
            .unwrap();
        assert_eq!(adopted.local_id, Some([99; 16]));
        assert_eq!(adopted.pending_chunks, 3);
        assert!(adopted.pending_control);
        assert_eq!(adopted.lost_chunks, 2);
        assert_eq!(adopted.summary.as_ref().unwrap().recording.id, [10; 16]);
        let unrelated = rows
            .iter()
            .find(|row| row.recording_id == Some([11; 16]))
            .unwrap();
        assert_eq!(unrelated.local_id, None);
        assert_eq!(unrelated.pending_chunks, 0);
    }
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn stale_local_worker_cannot_publish_rows_or_errors_after_device_replacement() {
    use tutorlolv2_protocol::HistoryErrorKind;
    let dir = std::env::temp_dir().join(format!("tl-stale-local-worker-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    // A worker collects this batch before registration replaces the binding.
    let collected = vec![([8; 16], Some([9; 16]), 1, true, 2)];
    history.bind(Some([2; 16])).unwrap();
    history.local([3; 16], Some([4; 16]), 0, false, 0).unwrap();
    history
        .errors(
            Some([2; 16]),
            vec![([3; 16], Some([4; 16]), "replacement error".into())],
        )
        .unwrap();
    let before = history.list(0, 50).entries;
    let revision = history.revision();
    assert_eq!(
        history
            .reconcile_local(Some([1; 16]), collected)
            .unwrap_err()
            .kind,
        HistoryErrorKind::Unauthorized
    );
    assert_eq!(
        history.errors(Some([1; 16]), vec![]).unwrap_err().kind,
        HistoryErrorKind::Unauthorized
    );
    assert_eq!(history.revision(), revision);
    assert_eq!(history.list(0, 50).entries, before);
    assert_eq!(
        History::new(Store::new(dir.clone()), Some([2; 16]))
            .list(0, 50)
            .entries,
        before
    );
    let _ = std::fs::remove_dir_all(dir);
}
