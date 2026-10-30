use std::path::PathBuf;
use tutorlolv2_desktop_app::{recorder::SessionFile, store::Store};
use tutorlolv2_protocol::{DeviceCredentials, EndReason, EndRecording, EventKey, GameMode, Probe};

/// A fresh, empty directory per test.
fn dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("tutorlolv2-store-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn credentials() -> DeviceCredentials {
    DeviceCredentials {
        device_id: [7; 16],
        device_secret: std::array::from_fn(|i| i as u8 * 3 + 1),
    }
}

fn session_file() -> SessionFile {
    SessionFile {
        local_id: [1; 16],
        server: Some(([2; 16], 3)),
        probe: Probe {
            active_riot_id: "Me#BR1".into(),
            game_mode: GameMode::Classic,
            map_number: 11,
            roster: vec![],
            game_time: 120.5,
            events: vec![EventKey {
                id: 0,
                name_hash: 1,
                time_bits: 2,
            }],
            riot_game: None,
        },
        last_seen_ms: 1_800_000_000_000,
        next_local_seq: 4,
        ended: false,
        pending_end: Some(EndRecording {
            reason: EndReason::GameEndEvent,
            result: None,
            final_game_time: Some(1020.4),
        }),
        lcu_game_id: Some(5669550383),
    }
}

#[test]
fn device_roundtrip_and_secret_not_plaintext_on_disk() {
    let dir = dir("device");
    let store = Store::new(dir.clone());
    assert_eq!(store.load_device(), None);
    store.save_device(&credentials()).unwrap();
    assert_eq!(store.load_device(), Some(credentials()));

    let bytes = std::fs::read(dir.join("device.bin")).unwrap();
    let secret = credentials().device_secret;
    assert!(
        !bytes.windows(secret.len()).any(|w| w == secret),
        "the secret must not be stored in plain text"
    );
    assert!(!dir.join("device.bin.tmp").exists());
}

#[test]
fn session_roundtrip_and_clear() {
    let dir = dir("session");
    let store = Store::new(dir.clone());
    assert_eq!(store.load_session(), None);
    store.save_session(Some(&session_file())).unwrap();
    assert_eq!(store.load_session(), Some(session_file()));

    store.save_session(None).unwrap();
    assert_eq!(store.load_session(), None);
    assert!(
        dir.join("session.bin.done").exists(),
        "cleared sessions are moved aside, not deleted"
    );
}

#[test]
fn corrupt_file_is_moved_aside() {
    let dir = dir("corrupt");
    std::fs::write(dir.join("device.bin"), b"garbage").unwrap();
    let store = Store::new(dir.clone());
    assert_eq!(store.load_device(), None);
    assert!(dir.join("device.bin.corrupt").exists());
    assert!(!dir.join("device.bin").exists());
}

#[test]
fn wrong_magic_or_version_is_corrupt() {
    let dir = dir("magic");
    let store = Store::new(dir.clone());
    store.save_session(Some(&session_file())).unwrap();
    let mut bytes = std::fs::read(dir.join("session.bin")).unwrap();

    let mut wrong_version = bytes.clone();
    wrong_version[4] = 9;
    std::fs::write(dir.join("session.bin"), &wrong_version).unwrap();
    assert_eq!(store.load_session(), None);
    assert!(dir.join("session.bin.corrupt").exists());

    bytes[0] = b'X';
    std::fs::write(dir.join("session.bin"), &bytes).unwrap();
    assert_eq!(store.load_session(), None);
}

#[test]
fn session_file_is_not_read_as_device() {
    let dir = dir("kind");
    let store = Store::new(dir.clone());
    store.save_session(Some(&session_file())).unwrap();
    std::fs::copy(dir.join("session.bin"), dir.join("device.bin")).unwrap();
    assert_eq!(store.load_device(), None);
}
