use std::{
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tutorlolv2_desktop_app::BINCODE_CONFIG;
use tutorlolv2_desktop_app::{
    delivery::Delivery,
    recorder::Recorder,
    spool::{Spool, Target, new_local_id},
    store::Store,
    uploader::{ApiClient, encode_chunk},
};
use tutorlolv2_protocol::*;

fn dir(name: &str) -> PathBuf {
    let path = std::env::temp_dir()
        .join("tutorlolv2-delivery-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).unwrap();
    path
}

// A real local HTTP transport, with bounded waits so failures cannot hang tests.
fn server(replies: Vec<(u16, Vec<u8>)>) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let worker = thread::spawn(move || {
        let mut paths = Vec::new();
        for (status, body) in replies {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(e)
                        if e.kind() == std::io::ErrorKind::WouldBlock
                            && Instant::now() < deadline =>
                    {
                        thread::sleep(Duration::from_millis(5))
                    }
                    Err(e) => panic!("Expected request: {e}"),
                }
            };
            // Windows may inherit the listener's nonblocking mode. Reading
            // a partially received HTTP request must wait for the next bytes.
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut request = Vec::new();
            let mut buf = [0; 4096];
            loop {
                let n = stream.read(&mut buf).unwrap();
                assert!(n > 0);
                request.extend_from_slice(&buf[..n]);
                if let Some(end) = request.windows(4).position(|s| s == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end]);
                    let len = headers
                        .lines()
                        .find_map(|line| {
                            line.to_ascii_lowercase()
                                .strip_prefix("content-length:")
                                .map(|s| s.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if request.len() >= end + 4 + len {
                        break;
                    }
                }
            }
            paths.push(
                String::from_utf8_lossy(&request)
                    .lines()
                    .next()
                    .unwrap()
                    .to_owned(),
            );
            write!(
                stream,
                "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .unwrap();
            stream.write_all(&body).unwrap();
        }
        paths
    });
    (base, worker)
}

fn driver(root: &std::path::Path, base: String) -> Arc<Delivery> {
    Arc::new(Delivery::new(
        Recorder::new(None, None, "test".into()),
        ApiClient::with_base_url(
            base,
            Some(&DeviceCredentials {
                device_id: [1; 16],
                device_secret: [2; 32],
            }),
        ),
        Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES),
        Store::new(root.to_owned()),
    ))
}

fn chunk(seq: u32) -> Vec<u8> {
    encode_chunk(&RawChunk {
        format: RAW_CHUNK_FORMAT,
        seq,
        snapshots: vec![RawSnapshot {
            captured_at_ms: 1,
            game_time: 1.0,
            json: b"{}".to_vec(),
        }],
    })
    .0
}

#[tokio::test]
async fn successful_end_retry_clears_control_error_without_a_chunk_upload() {
    let root = dir("end_error_recovery");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool
        .set_end(
            id,
            &EndRecording {
                reason: EndReason::GameEndEvent,
                result: None,
                final_game_time: Some(1200.0),
            },
        )
        .unwrap();
    let (base, worker) = server(vec![(503, vec![]), (204, vec![])]);
    let delivery = driver(&root, base);
    assert!(!delivery.flush().await);
    assert!(delivery.recording_state().upload.last_error.is_some());
    assert!(delivery.flush().await);
    assert!(spool.outbox().is_empty());
    assert!(delivery.recording_state().upload.last_error.is_none());
    let paths = worker.join().unwrap();
    assert!(paths.iter().all(|p| p.contains("/end ")));
}

#[tokio::test]
async fn successful_chunk_upload_does_not_clear_pending_control_error() {
    let root = dir("result_error_with_chunk");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool
        .set_result(
            id,
            &RecordingResult {
                result: GameResult::Win,
                game_length_s: Some(1200),
                riot_game: None,
            },
        )
        .unwrap();
    let (base, worker) = server(vec![
        (503, vec![]),
        (503, vec![]),
        (201, vec![]),
        (204, vec![]),
    ]);
    let delivery = driver(&root, base);
    assert!(!delivery.flush().await);
    spool.put(id, 0, &chunk(0)).unwrap();
    assert!(!delivery.flush().await);
    assert!(spool.pending().is_empty());
    assert!(delivery.recording_state().upload.last_error.is_some());
    assert!(delivery.flush().await);
    assert!(delivery.recording_state().upload.last_error.is_none());
    assert_eq!(worker.join().unwrap().len(), 4);
}

#[tokio::test]
async fn result_retries_after_restart() {
    let root = dir("result_retry");
    let id = new_local_id(1, [0; 10]);
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
    spool
        .set_target(
            id,
            &Target {
                recording_id: [9; 16],
                seq_offset: 0,
            },
        )
        .unwrap();
    spool
        .set_result(
            id,
            &RecordingResult {
                result: GameResult::Win,
                game_length_s: Some(1200),
                riot_game: None,
            },
        )
        .unwrap();
    let (base, worker) = server(vec![(503, vec![]), (204, vec![])]);
    assert!(!driver(&root, base.clone()).flush().await);
    assert_eq!(spool.outbox().len(), 1);
    assert!(driver(&root, base).flush().await);
    assert!(spool.outbox().is_empty());
    let paths = worker.join().unwrap();
    assert_eq!(paths[0], paths[1]);
    assert!(paths[0].starts_with("PATCH ") && paths[0].contains("/result "));
}

#[tokio::test]
async fn recovery_does_not_reuse_an_older_pending_chunk_sequence() {
    let root = dir("sequence_recovery");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool.put(old, 5, &chunk(5)).unwrap();
    spool.put(new, 0, &chunk(0)).unwrap();
    let probe = Probe {
        active_riot_id: "Me#1".into(),
        game_mode: GameMode::Classic,
        map_number: 11,
        roster: vec![],
        game_time: 100.0,
        events: vec![],
        riot_game: None,
    };
    spool
        .set_start(
            new,
            &StartSession {
                probe,
                region: None,
                lcu: None,
                app_version: "test".into(),
            },
        )
        .unwrap();
    let started = SessionStarted {
        recording_id,
        resumed: true,
        next_chunk_seq: 5,
        share: ShareState {
            mode: ShareMode::Private,
            code: None,
            url: None,
        },
    };
    let response = bincode::encode_to_vec(started, BINCODE_CONFIG).unwrap();
    let (base, worker) = server(vec![(200, response), (201, vec![]), (201, vec![])]);
    assert!(driver(&root, base).flush().await);
    assert!(spool.pending().is_empty());
    let paths = worker.join().unwrap();
    assert!(paths[1].contains("/chunks/5 "));
    assert!(paths[2].contains("/chunks/6 "));
}

#[tokio::test]
async fn failed_chunk_does_not_block_later_uploads() {
    let root = dir("chunk_retry");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool.put(id, 0, &chunk(0)).unwrap();
    spool.put(id, 1, &chunk(1)).unwrap();
    let (base, worker) = server(vec![
        (500, vec![]),
        (201, vec![]),
        (500, vec![]),
        (201, vec![]),
        (201, vec![]),
    ]);
    let delivery = driver(&root, base);
    assert!(!delivery.flush().await);
    assert!(delivery.recording_state().upload.last_error.is_some());
    assert_eq!(
        spool
            .pending()
            .iter()
            .map(|c| c.local_seq)
            .collect::<Vec<_>>(),
        [0]
    );
    spool.put(id, 2, &chunk(2)).unwrap();
    assert!(!delivery.flush().await);
    assert_eq!(spool.pending().len(), 1);
    assert!(delivery.recording_state().upload.last_error.is_some());
    assert!(delivery.flush().await);
    assert!(spool.pending().is_empty());
    assert!(delivery.recording_state().upload.last_error.is_none());
    assert_eq!(worker.join().unwrap().len(), 5);
}

#[tokio::test]
async fn uploader_flushes_at_startup_and_serializes_concurrent_passes() {
    let root = dir("startup");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool.put(id, 0, &chunk(0)).unwrap();
    let (base, worker) = server(vec![(201, vec![])]);
    let delivery = driver(&root, base);
    let task = tokio::spawn(delivery.clone().run_uploads());
    let flush = delivery.flush();
    assert!(
        tokio::time::timeout(Duration::from_secs(2), flush)
            .await
            .unwrap()
    );
    task.abort();
    assert_eq!(worker.join().unwrap().len(), 1);
    assert!(spool.pending().is_empty());
}

#[tokio::test]
async fn background_uploader_does_not_wait_for_the_first_minute() {
    let root = dir("startup_only");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool.put(id, 0, &chunk(0)).unwrap();
    let (base, worker) = server(vec![(201, vec![])]);
    let task = tokio::spawn(driver(&root, base).run_uploads());
    tokio::time::timeout(Duration::from_secs(2), async {
        while !spool.pending().is_empty() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    task.abort();
    assert_eq!(worker.join().unwrap().len(), 1);
}

#[tokio::test]
async fn upload_passes_share_rate_limit_cooldown() {
    let root = dir("rate_cooldown");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
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
    spool.put(id, 0, &chunk(0)).unwrap();
    spool.put(id, 1, &chunk(1)).unwrap();
    let (base, worker) = server(vec![(429, vec![]), (201, vec![]), (201, vec![])]);
    let delivery = driver(&root, base);
    assert!(!delivery.flush().await);
    assert!(
        !delivery.flush().await,
        "Immediate retries must respect the shared cooldown"
    );
    assert_eq!(spool.pending().len(), 2);
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert!(delivery.flush().await);
    assert_eq!(worker.join().unwrap().len(), 3);
}

#[tokio::test]
async fn failed_old_result_does_not_block_a_new_session() {
    let root = dir("result_then_start");
    let spool = Spool::new(root.join("spool"), Spool::DEFAULT_MAX_BYTES);
    let old = new_local_id(1, [0; 10]);
    let new = new_local_id(2, [0; 10]);
    spool
        .set_target(
            old,
            &Target {
                recording_id: [9; 16],
                seq_offset: 0,
            },
        )
        .unwrap();
    spool
        .set_result(
            old,
            &RecordingResult {
                result: GameResult::Win,
                game_length_s: Some(1200),
                riot_game: None,
            },
        )
        .unwrap();
    let probe = Probe {
        active_riot_id: "Me#1".into(),
        game_mode: GameMode::Classic,
        map_number: 11,
        roster: vec![],
        game_time: 100.0,
        events: vec![],
        riot_game: None,
    };
    spool
        .set_start(
            new,
            &StartSession {
                probe,
                region: None,
                lcu: None,
                app_version: "test".into(),
            },
        )
        .unwrap();
    let started = SessionStarted {
        recording_id: [8; 16],
        resumed: false,
        next_chunk_seq: 0,
        share: ShareState {
            mode: ShareMode::Private,
            code: None,
            url: None,
        },
    };
    let (base, worker) = server(vec![
        (503, vec![]),
        (
            200,
            bincode::encode_to_vec(started, BINCODE_CONFIG).unwrap(),
        ),
    ]);
    assert!(!driver(&root, base).flush().await);
    assert_eq!(spool.target_of(new).unwrap().recording_id, [8; 16]);
    assert!(spool.outbox()[0].result.is_some());
    let paths = worker.join().unwrap();
    assert!(paths[0].starts_with("PATCH "));
    assert!(paths[1].contains("/sessions "));
}
