use tutorlolv2_desktop_app::BINCODE_CONFIG;
use tutorlolv2_desktop_app::{recorder::Recorder, sharing::authorize_share_window};
use tutorlolv2_protocol::{RecordingStatus, ShareMode};

#[test]
fn idle_recording_is_private_and_has_no_live_snapshot() {
    let recorder = Recorder::new(None, None, "test".into());
    let state = recorder.recording_state();
    assert_eq!(state.status, RecordingStatus::Idle);
    assert_eq!(state.share.mode, ShareMode::Private);
    assert!(state.recording_id.is_none());
    assert!(recorder.live_snapshot().is_none());
}

#[test]
fn only_main_window_can_change_sharing() {
    assert!(authorize_share_window("main").is_ok());
    assert!(authorize_share_window("overlay").is_err());
    assert!(authorize_share_window("other").is_err());
}

use serde_json::json;
use tutorlolv2_desktop_app::{
    lcu::LcuPhase,
    live_client::{LivePoll, classify},
    recorder::Effect,
};
use tutorlolv2_protocol::*;
fn snap(t: f32, start: f64, champion: &str, extra_events: serde_json::Value) -> LivePoll {
    let mut events = vec![json!({"EventID": 0, "EventName": "GameStart", "EventTime": start})];
    if let serde_json::Value::Array(more) = extra_events {
        events.extend(more);
    }
    let body = json!({
        "activePlayer": {"riotId": "Me#BR1", "level": 3},
        "allPlayers": [
            {"riotId": "Me#BR1", "championName": champion, "rawSkinName": format!("game_character_displayname_{champion}"), "team": "ORDER"},
            {"riotId": "Foe#1", "championName": "Zed", "rawSkinName": "game_character_displayname_Zed", "team": "CHAOS"}
        ],
        "events": {"Events": events},
        "gameData": {"gameMode": "CLASSIC", "gameTime": t, "mapNumber": 11}
    });
    classify(Some(200), &serde_json::to_vec(&body).unwrap())
}

#[test]
fn ending_game_clears_code_and_prevents_live_upload_and_restore_of_stale_code() {
    let mut recorder = Recorder::new(None, None, "test".into());
    let effects = recorder.on_live(1000, snap(10.0, 0.0, "Ahri", json!([])));
    let local_id = effects
        .iter()
        .find_map(|e| match e {
            Effect::StartSession { local_id, .. } => Some(*local_id),
            _ => None,
        })
        .unwrap();
    recorder.on_session_started(
        local_id,
        &SessionStarted {
            recording_id: [9; 16],
            resumed: false,
            next_chunk_seq: 0,
            share: tutorlolv2_desktop_app::sharing::private_share(),
        },
    );
    let live = ShareState {
        mode: ShareMode::Live,
        code: Some("abc123".into()),
        url: Some("https://example.test/watch/abc123".into()),
    };
    assert!(recorder.set_share([9; 16], live.clone()));
    assert!(recorder.live_snapshot().is_some());
    let effects = recorder.on_lcu(2000, LcuPhase::EndOfGame, None, None);
    assert_eq!(recorder.recording_state().share.mode, ShareMode::Private);
    assert!(recorder.live_snapshot().is_none());
    assert!(!recorder.set_share([9; 16], live));
    let session = effects
        .into_iter()
        .find_map(|e| match e {
            Effect::PersistSession(Some(s)) if s.ended => Some(s),
            _ => None,
        })
        .unwrap();
    let restored = Recorder::new(Some(session), None, "test".into());
    assert_eq!(restored.recording_state().share.mode, ShareMode::Private);
    assert!(restored.recording_state().recording_id.is_none());
}

#[tokio::test]
async fn native_transport_refreshes_changes_and_uploads_brotli_raw_snapshot() {
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };
    use tutorlolv2_desktop_app::uploader::ApiClient;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let expected = ShareState {
        mode: ShareMode::Live,
        code: Some("abcdefgh".into()),
        url: Some("https://example.test/watch/abcdefgh".into()),
    };
    let response = bincode::encode_to_vec(&expected, BINCODE_CONFIG).unwrap();
    let snapshot = RawSnapshot {
        captured_at_ms: 1234,
        game_time: 4.5,
        json: b"{\"game\":true}".to_vec(),
    };
    let expected_snapshot = snapshot.clone();
    let worker = std::thread::spawn(move || {
        for method in ["GET", "PUT", "PUT"] {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                .unwrap();
            let mut request = Vec::new();
            let end;
            loop {
                let mut buf = [0; 4096];
                let count = stream.read(&mut buf).unwrap();
                assert!(count > 0);
                request.extend_from_slice(&buf[..count]);
                if let Some(i) = request.windows(4).position(|s| s == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..i]).to_ascii_lowercase();
                    let len = headers
                        .lines()
                        .find_map(|s| {
                            s.strip_prefix("content-length:")
                                .map(|v| v.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if request.len() >= i + 4 + len {
                        end = i;
                        break;
                    }
                }
            }
            let headers = String::from_utf8_lossy(&request[..end]);
            assert!(headers.starts_with(method));
            assert!(
                headers
                    .to_ascii_lowercase()
                    .contains("authorization: device ")
            );
            let body = &request[end + 4..];
            let live = headers.lines().next().unwrap().contains("/live ");
            if live {
                let mut raw = Vec::new();
                brotli::Decompressor::new(body, 4096)
                    .read_to_end(&mut raw)
                    .unwrap();
                let (actual, used): (RawSnapshot, usize) =
                    bincode::decode_from_slice(&raw, BINCODE_CONFIG).unwrap();
                assert_eq!(used, raw.len());
                assert_eq!(actual, expected_snapshot);
            } else if method == "PUT" {
                let (actual, _): (SetShareMode, usize) =
                    bincode::decode_from_slice(body, BINCODE_CONFIG).unwrap();
                assert_eq!(actual.mode, ShareMode::Live);
            }
            let body = if live { &[][..] } else { response.as_slice() };
            write!(
                stream,
                "HTTP/1.1 {} OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                if live { 204 } else { 200 },
                body.len()
            )
            .unwrap();
            stream.write_all(body).unwrap();
        }
    });
    let client = ApiClient::with_base_url(
        base,
        Some(&DeviceCredentials {
            device_id: [1; 16],
            device_secret: [2; 32],
        }),
    );
    assert_eq!(client.share(&[9; 16]).await.unwrap(), expected);
    assert_eq!(
        client
            .set_share_mode(&[9; 16], ShareMode::Live)
            .await
            .unwrap(),
        expected
    );
    client.upload_live(&[9; 16], &snapshot).await.unwrap();
    worker.join().unwrap();
}

#[test]
fn lcu_metadata_controls_public_availability_and_new_match_is_private() {
    let mut recorder = Recorder::new(None, None, "test".into());
    recorder.on_lcu(
        1000,
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: 7,
            queue_id: Some(420),
            is_custom: Some(false),
        }),
        None,
    );
    let effects = recorder.on_live(1000, snap(10.0, 0.0, "Ahri", json!([])));
    assert_eq!(recorder.recording_state().is_custom, Some(false));
    assert!(!recorder.recording_state().is_practice);
    let local_id = effects
        .iter()
        .find_map(|e| match e {
            Effect::StartSession { local_id, .. } => Some(*local_id),
            _ => None,
        })
        .unwrap();
    recorder.on_session_started(
        local_id,
        &SessionStarted {
            recording_id: [9; 16],
            resumed: false,
            next_chunk_seq: 0,
            share: ShareState {
                mode: ShareMode::Public,
                code: Some("abcdefgh".into()),
                url: None,
            },
        },
    );
    assert_eq!(recorder.recording_state().share.mode, ShareMode::Public);
    recorder.on_live(2000, snap(1.0, 0.5, "Zed", json!([])));
    assert_eq!(recorder.recording_state().share.mode, ShareMode::Private);
    assert!(recorder.recording_state().share.code.is_none());
    assert!(recorder.recording_state().is_custom.is_none());
}

fn publisher(
    base: String,
) -> (
    std::sync::Arc<tutorlolv2_desktop_app::delivery::Delivery>,
    RawSnapshot,
    ShareState,
) {
    use tutorlolv2_desktop_app::{
        delivery::Delivery, spool::Spool, store::Store, uploader::ApiClient,
    };
    let mut recorder = Recorder::new(None, None, "test".into());
    let effects = recorder.on_live(1000, snap(10.0, 0.0, "Ahri", json!([])));
    let local_id = effects
        .iter()
        .find_map(|e| match e {
            Effect::StartSession { local_id, .. } => Some(*local_id),
            _ => None,
        })
        .unwrap();
    let share = ShareState {
        mode: ShareMode::Live,
        code: Some("abcdefgh".into()),
        url: None,
    };
    recorder.on_session_started(
        local_id,
        &SessionStarted {
            recording_id: [9; 16],
            resumed: false,
            next_chunk_seq: 0,
            share: share.clone(),
        },
    );
    let snapshot = recorder.live_snapshot().unwrap();
    let mut random = [0u8; 8];
    getrandom::fill(&mut random).unwrap();
    let dir = std::env::temp_dir().join(format!(
        "tutorlolv2-sharing-{:x}",
        u64::from_le_bytes(random)
    ));
    (
        std::sync::Arc::new(Delivery::new(
            recorder,
            ApiClient::with_base_url(
                base,
                Some(&DeviceCredentials {
                    device_id: [1; 16],
                    device_secret: [2; 32],
                }),
            ),
            Spool::new(dir.join("spool"), Spool::DEFAULT_MAX_BYTES),
            Store::new(dir),
        )),
        snapshot,
        share,
    )
}

fn read_request(stream: &mut std::net::TcpStream) -> String {
    use std::io::Read;
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .unwrap();
    let mut request = Vec::new();
    loop {
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).unwrap();
        assert!(n > 0);
        request.extend_from_slice(&buf[..n]);
        if let Some(end) = request.windows(4).position(|s| s == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&request[..end]).to_ascii_lowercase();
            let len = headers
                .lines()
                .find_map(|s| {
                    s.strip_prefix("content-length:")
                        .map(|v| v.trim().parse::<usize>().unwrap())
                })
                .unwrap_or(0);
            if request.len() >= end + 4 + len {
                return String::from_utf8_lossy(&request[..end]).into_owned();
            }
        }
    }
}

#[tokio::test]
async fn failed_share_refresh_preserves_last_known_access_and_pauses_publication() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (delivery, snapshot, share) =
        publisher(format!("http://{}", listener.local_addr().unwrap()));
    let response = bincode::encode_to_vec(&share, BINCODE_CONFIG).unwrap();
    let worker = std::thread::spawn(move || {
        for (method, status, body) in [
            ("GET", 500, b"unavailable".as_slice()),
            ("GET", 200, response.as_slice()),
            ("PUT", 204, &[]),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            assert!(read_request(&mut stream).starts_with(method));
            respond(&mut stream, status, body);
        }
    });
    assert!(delivery.refresh_share(&[9; 16]).await.is_err());
    let state = delivery.recording_state();
    assert_eq!(
        state.share, share,
        "a failed lookup does not revoke server access"
    );
    assert!(
        state
            .upload
            .last_error
            .as_deref()
            .unwrap()
            .contains("unverified")
    );
    assert!(
        !delivery
            .publish_live(&[9; 16], &share, &snapshot)
            .await
            .unwrap()
    );
    delivery.refresh_share(&[9; 16]).await.unwrap();
    assert!(delivery.recording_state().upload.last_error.is_none());
    assert!(
        delivery
            .publish_live(&[9; 16], &share, &snapshot)
            .await
            .unwrap()
    );
    worker.join().unwrap();
}

#[tokio::test]
async fn live_upload_error_survives_share_lookup_until_publication_succeeds() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (delivery, snapshot, share) =
        publisher(format!("http://{}", listener.local_addr().unwrap()));
    let response = bincode::encode_to_vec(&share, BINCODE_CONFIG).unwrap();
    let worker = std::thread::spawn(move || {
        for (method, status, body) in [
            ("PUT", 500, b"upload failed".as_slice()),
            ("GET", 200, response.as_slice()),
            ("PUT", 204, &[]),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            assert!(read_request(&mut stream).starts_with(method));
            respond(&mut stream, status, body);
        }
    });
    assert!(
        delivery
            .publish_live(&[9; 16], &share, &snapshot)
            .await
            .is_err()
    );
    let error = delivery.recording_state().upload.last_error;
    assert!(error.as_deref().is_some_and(|e| e.contains("500")));
    delivery.refresh_share(&[9; 16]).await.unwrap();
    assert_eq!(delivery.recording_state().upload.last_error, error);
    assert!(
        delivery
            .publish_live(&[9; 16], &share, &snapshot)
            .await
            .unwrap()
    );
    assert!(delivery.recording_state().upload.last_error.is_none());
    worker.join().unwrap();
}

fn respond(stream: &mut std::net::TcpStream, status: u16, body: &[u8]) {
    use std::io::Write;
    write!(
        stream,
        "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .unwrap();
    stream.write_all(body).unwrap();
}

#[tokio::test]
async fn slow_live_upload_survives_multiple_snapshot_tick_notifications() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (delivery, snapshot, share) =
        publisher(format!("http://{}", listener.local_addr().unwrap()));
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        assert!(read_request(&mut stream).starts_with("PUT /api/recordings/"));
        std::thread::sleep(std::time::Duration::from_millis(1400));
        respond(&mut stream, 204, &[]);
    });
    let ticks = async {
        for _ in 0..5 {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            delivery.recording_changed();
        }
    };
    let (result, _) = tokio::join!(delivery.publish_live(&[9; 16], &share, &snapshot), ticks);
    assert!(
        result.unwrap(),
        "snapshot-only ticks must preserve the in-flight request"
    );
    worker.join().unwrap();
}

#[tokio::test]
async fn rejected_live_upload_refreshes_expired_code_to_private_without_republishing() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (delivery, snapshot, share) =
        publisher(format!("http://{}", listener.local_addr().unwrap()));
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        assert!(
            read_request(&mut stream)
                .lines()
                .next()
                .unwrap()
                .contains("/live ")
        );
        respond(&mut stream, 409, b"sharing expired");
        let (mut stream, _) = listener.accept().unwrap();
        assert!(read_request(&mut stream).starts_with("GET /api/recordings/"));
        respond(
            &mut stream,
            200,
            &bincode::encode_to_vec(
                tutorlolv2_desktop_app::sharing::private_share(),
                BINCODE_CONFIG,
            )
            .unwrap(),
        );
    });
    assert!(
        delivery
            .publish_live(&[9; 16], &share, &snapshot)
            .await
            .is_err()
    );
    assert_eq!(delivery.recording_state().share.mode, ShareMode::Private);
    assert!(delivery.recording_state().share.code.is_none());
    assert!(
        !delivery
            .publish_live(&[9; 16], &share, &snapshot)
            .await
            .unwrap()
    );
    worker.join().unwrap();
}

#[tokio::test]
async fn user_private_transition_cancels_live_request_without_waiting_for_response() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (delivery, snapshot, share) =
        publisher(format!("http://{}", listener.local_addr().unwrap()));
    let (release, wait) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let (mut live, _) = listener.accept().unwrap();
        assert!(
            read_request(&mut live)
                .lines()
                .next()
                .unwrap()
                .contains("/live ")
        );
        let (mut control, _) = listener.accept().unwrap();
        assert!(
            read_request(&mut control)
                .lines()
                .next()
                .unwrap()
                .contains("/share ")
        );
        respond(
            &mut control,
            200,
            &bincode::encode_to_vec(
                tutorlolv2_desktop_app::sharing::private_share(),
                BINCODE_CONFIG,
            )
            .unwrap(),
        );
        // Keep the live response pending until the cancellation assertion finished.
        wait.recv_timeout(std::time::Duration::from_secs(4))
            .unwrap();
    });
    let make_private = async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        delivery.set_share_mode(ShareMode::Private).await.unwrap();
    };
    let start = std::time::Instant::now();
    let (published, _) = tokio::time::timeout(std::time::Duration::from_secs(3), async {
        tokio::join!(
            delivery.publish_live(&[9; 16], &share, &snapshot),
            make_private
        )
    })
    .await
    .unwrap();
    assert!(!published.unwrap());
    assert!(start.elapsed() < std::time::Duration::from_secs(1));
    release.send(()).unwrap();
    worker.join().unwrap();
}
