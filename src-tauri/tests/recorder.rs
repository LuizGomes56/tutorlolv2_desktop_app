use serde_json::json;
use tutorlolv2_desktop_app::{
    lcu::LcuPhase,
    live_client::{LivePoll, classify},
    recorder::*,
};
use tutorlolv2_protocol::*;

const T0: i64 = 1_800_000_000_000;

fn at(seconds: i64) -> i64 {
    T0 + seconds * 1000
}

/// A valid snapshot at game time `t` of the match that started at `start` (its GameStart time).
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

fn plain(t: f32) -> LivePoll {
    snap(t, 0.0276, "Ahri", json!([]))
}

fn recorder() -> Recorder {
    Recorder::new(None, Some(Region::Na), "0.3.0".into())
}

fn started(recording: u8, next_chunk_seq: u32) -> SessionStarted {
    SessionStarted {
        recording_id: [recording; 16],
        resumed: next_chunk_seq > 0,
        next_chunk_seq,
        share: ShareState {
            mode: ShareMode::Private,
            code: None,
            url: None,
        },
    }
}

fn starts(effects: &[Effect]) -> Vec<Id16> {
    effects
        .iter()
        .filter_map(|e| match e {
            Effect::StartSession { local_id, .. } => Some(*local_id),
            _ => None,
        })
        .collect()
}

fn seals(effects: &[Effect]) -> Vec<&RawChunk> {
    effects
        .iter()
        .filter_map(|e| match e {
            Effect::SealChunk { chunk, .. } => Some(chunk),
            _ => None,
        })
        .collect()
}

fn ends(effects: &[Effect]) -> Vec<&EndRecording> {
    effects
        .iter()
        .filter_map(|e| match e {
            Effect::EndRecording { req, .. } => Some(req),
            _ => None,
        })
        .collect()
}

fn persisted(effects: &[Effect]) -> Option<&Option<SessionFile>> {
    effects.iter().rev().find_map(|e| match e {
        Effect::PersistSession(s) => Some(s),
        _ => None,
    })
}

/// A recorder that started a session at game time 100 and got its server id.
fn recording() -> (Recorder, Id16) {
    let mut r = recorder();
    let effects = r.on_live(at(0), plain(100.0));
    let local_id = starts(&effects)[0];
    r.on_session_started(local_id, &started(9, 0));
    (r, local_id)
}

#[test]
fn first_snapshot_starts_session_and_persists() {
    let mut r = recorder();
    assert_eq!(r.status(), Status::Idle);
    let effects = r.on_live(at(0), plain(100.0));
    let local_id = starts(&effects)[0];
    match &effects[0] {
        Effect::StartSession { req, .. } => {
            assert_eq!(req.region, Some(Region::Na));
            assert_eq!(req.probe.active_riot_id, "Me#BR1");
            assert_eq!(req.app_version, "0.3.0");
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        persisted(&effects).unwrap().as_ref().unwrap().local_id,
        local_id
    );
    assert_eq!(r.status(), Status::Starting);
    r.on_session_started(local_id, &started(9, 0));
    assert_eq!(r.status(), Status::Recording);
    assert!(r.latest().is_some());
}

#[test]
fn identical_bytes_are_not_buffered() {
    let (mut r, _) = recording();
    assert!(r.on_live(at(1), plain(100.0)).is_empty());
    let effects = r.on_tick(at(61));
    assert_eq!(seals(&effects)[0].snapshots.len(), 1);
}

#[test]
fn seals_after_60s_and_at_120_snapshots() {
    let (mut r, _) = recording();
    for s in 1..59 {
        assert!(seals(&r.on_live(at(s), plain(100.0 + s as f32))).is_empty());
    }
    assert!(seals(&r.on_tick(at(59))).is_empty());
    let first = r.on_tick(at(60));
    let chunk = seals(&first)[0];
    assert_eq!((chunk.seq, chunk.snapshots.len()), (0, 59));

    let mut sealed = Vec::new();
    for s in 61..=180 {
        sealed.extend(
            r.on_live(at(s), plain(100.0 + s as f32))
                .into_iter()
                .filter(|e| matches!(e, Effect::SealChunk { .. })),
        );
    }
    assert_eq!(seals(&sealed).len(), 1);
    assert_eq!(
        (seals(&sealed)[0].seq, seals(&sealed)[0].snapshots.len()),
        (1, 120)
    );
}

#[test]
fn captured_at_stays_increasing_when_the_clock_goes_back() {
    let (mut r, _) = recording();
    r.on_live(at(10), plain(110.0));
    r.on_live(at(5), plain(111.0));
    let effects = r.on_tick(at(100));
    assert_eq!(seals(&effects)[0].validate(0), Ok(()));
}

#[test]
fn unreachable_seals_and_suspends_then_resume_same_game() {
    let (mut r, local_id) = recording();
    r.on_live(at(1), plain(101.0));
    let effects = r.on_live(at(2), LivePoll::Unreachable);
    assert_eq!(seals(&effects)[0].snapshots.len(), 2);
    assert_eq!(r.status(), Status::Suspended);
    assert!(r.latest().is_none(), "no live game while 2999 is down");

    let effects = r.on_live(at(120), plain(220.0));
    assert!(starts(&effects).is_empty() && ends(&effects).is_empty());
    assert_eq!(r.status(), Status::Recording);
    let effects = r.on_exit(at(121));
    let session = persisted(&effects).unwrap().as_ref().unwrap();
    assert_eq!((session.local_id, session.next_local_seq), (local_id, 2));
}

#[test]
fn suspended_15_min_goes_idle_and_clears_session() {
    let (mut r, _) = recording();
    r.on_live(at(1), LivePoll::Unreachable);
    assert!(persisted(&r.on_tick(at(1 + 14 * 60))).is_none());
    let effects = r.on_tick(at(1 + 15 * 60));
    assert_eq!(persisted(&effects), Some(&None));
    assert_eq!(r.status(), Status::Idle);
}

#[test]
fn new_game_within_window_supersedes() {
    let (mut r, old) = recording();
    r.on_live(at(1), plain(101.0));
    let effects = r.on_live(at(60), snap(40.0, 0.0312, "Zoe", json!([])));
    assert_eq!(
        seals(&effects).len(),
        1,
        "the old partial chunk is sealed first"
    );
    assert_eq!(ends(&effects)[0].reason, EndReason::Superseded);
    assert!(
        matches!(effects.iter().find(|e| matches!(e, Effect::EndRecording { .. })), Some(Effect::EndRecording { local_id, .. }) if *local_id == old)
    );
    let new = starts(&effects);
    assert_eq!(new.len(), 1);
    assert_ne!(new[0], old);
}

#[test]
fn game_end_event_ends_with_result_and_drops_tail() {
    let mut r = recorder();
    let local_id = starts(&r.on_live(at(0), plain(1010.0)))[0];
    r.on_session_started(local_id, &started(9, 0));
    let end = json!([{"EventID": 135, "EventName": "GameEnd", "EventTime": 1020.364501953125, "Result": "Lose"}]);
    let effects = r.on_live(at(10), snap(1020.6, 0.0276, "Ahri", end.clone()));
    assert_eq!(seals(&effects).len(), 1);
    let end_req = ends(&effects)[0];
    assert_eq!(end_req.reason, EndReason::GameEndEvent);
    assert_eq!(end_req.result, Some(GameResult::Lose));
    assert_eq!(end_req.final_game_time, Some(1020.364501953125f64 as f32));
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, Effect::FetchEog { local_id: id, .. } if *id == local_id))
    );
    assert_eq!(r.status(), Status::Idle);

    for s in 1..=7 {
        assert!(
            r.on_live(
                at(10 + s),
                snap(1020.6 + s as f32, 0.0276, "Ahri", end.clone())
            )
            .is_empty(),
            "tail {s}"
        );
    }
    assert!(r.latest().is_some(), "the page still shows the final state");
}

#[test]
fn lcu_pre_end_of_game_ends_when_no_game_end_seen() {
    let (mut r, _) = recording();
    let effects = r.on_lcu(at(5), LcuPhase::PreEndOfGame, None, None);
    assert_eq!(ends(&effects)[0].reason, EndReason::LcuEndOfGame);
    assert!(effects.iter().any(|e| matches!(e, Effect::FetchEog { .. })));
    assert!(
        r.on_lcu(at(10), LcuPhase::EndOfGame, None, None).is_empty(),
        "ends once"
    );
}

#[test]
fn lcu_lobby_after_30s_unreachable_ends_left_game() {
    let (mut r, _) = recording();
    r.on_live(at(1), LivePoll::Unreachable);
    assert!(ends(&r.on_lcu(at(20), LcuPhase::Lobby, None, None)).is_empty());
    let effects = r.on_lcu(at(31), LcuPhase::Lobby, None, None);
    assert_eq!(ends(&effects)[0].reason, EndReason::LcuLeftGame);
    assert!(!effects.iter().any(|e| matches!(e, Effect::FetchEog { .. })));
}

#[test]
fn heartbeat_after_60s_without_upload() {
    let (mut r, local_id) = recording();
    r.on_chunk_uploaded(at(0));
    assert!(
        !r.on_tick(at(59))
            .iter()
            .any(|e| matches!(e, Effect::Heartbeat { .. }))
    );
    let effects = r.on_tick(at(60));
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, Effect::Heartbeat { local_id: id } if *id == local_id))
    );
    assert!(
        !r.on_tick(at(61))
            .iter()
            .any(|e| matches!(e, Effect::Heartbeat { .. })),
        "not every tick"
    );
}

#[test]
fn no_heartbeat_while_suspended() {
    let (mut r, _) = recording();
    r.on_live(at(1), LivePoll::Unreachable);
    assert!(
        !r.on_tick(at(300))
            .iter()
            .any(|e| matches!(e, Effect::Heartbeat { .. }))
    );
}

fn restored(server: Option<(Id16, u32)>) -> SessionFile {
    let LivePoll::Snapshot { snapshot, .. } = plain(100.0) else {
        unreachable!()
    };
    SessionFile {
        local_id: [5; 16],
        server,
        probe: snapshot.probe,
        last_seen_ms: at(0),
        next_local_seq: 4,
        ended: false,
        pending_end: None,
        lcu_game_id: None,
    }
}

#[test]
fn restored_session_resumes_without_start_session() {
    let mut r = Recorder::new(Some(restored(Some(([9; 16], 0)))), None, "0.3.0".into());
    let mut effects = r.on_tick(at(180));
    effects.extend(r.on_live(at(180), plain(280.0)));
    effects.extend(r.on_tick(at(240)));
    assert!(starts(&effects).is_empty());
    assert!(ends(&effects).is_empty());
    let chunk = seals(&effects)[0];
    assert_eq!(chunk.seq, 4, "continues next_local_seq");
    assert!(matches!(
        effects
            .iter()
            .find(|e| matches!(e, Effect::SealChunk { .. })),
        Some(Effect::SealChunk {
            local_id: [5, ..],
            ..
        })
    ));
}

#[test]
fn restored_session_without_server_id_restarts_session_call() {
    let mut r = Recorder::new(Some(restored(None)), None, "0.3.0".into());
    let effects = r.on_tick(at(1));
    assert_eq!(starts(&effects), vec![[5; 16]]);
    assert!(starts(&r.on_tick(at(2))).is_empty(), "only once");
}

#[test]
fn restored_pending_end_is_sent_again() {
    let mut file = restored(Some(([9; 16], 0)));
    file.ended = true;
    file.pending_end = Some(EndRecording {
        reason: EndReason::GameEndEvent,
        result: Some(GameResult::Win),
        final_game_time: Some(900.0),
    });
    let mut r = Recorder::new(Some(file), None, "0.3.0".into());
    let effects = r.on_tick(at(1));
    assert_eq!(ends(&effects)[0].result, Some(GameResult::Win));
    let effects = r.on_end_sent([5; 16]);
    assert_eq!(
        persisted(&effects).unwrap().as_ref().unwrap().pending_end,
        None
    );
}

#[test]
fn replay_never_starts_a_session() {
    let mut r = recorder();
    let replay = json!({
        "activePlayer": {"error": "Spectator mode doesn't currently support this feature"},
        "allPlayers": [], "events": {"Events": []},
        "gameData": {"gameMode": "CLASSIC", "gameTime": 40.0, "mapNumber": 11}
    });
    assert!(
        r.on_live(
            at(0),
            classify(Some(200), &serde_json::to_vec(&replay).unwrap())
        )
        .is_empty()
    );
    assert!(r.on_tick(at(100)).is_empty());
    assert_eq!(r.status(), Status::Idle);
}

#[test]
fn lcu_game_is_attached_and_forgotten_after_the_game() {
    let mut r = recorder();
    r.on_lcu(
        at(0),
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: 77,
            queue_id: Some(420),
            is_custom: Some(false),
        }),
        Some(Region::Na),
    );
    let effects = r.on_live(at(1), plain(1.0));
    match &effects[0] {
        Effect::StartSession { req, .. } => {
            assert_eq!(req.lcu.as_ref().map(|g| g.game_id), Some(77));
            assert_eq!(
                req.probe.riot_game,
                Some(RiotGameRef {
                    region: Region::Na,
                    game_id: 77
                })
            );
        }
        other => panic!("{other:?}"),
    }
    r.on_lcu(at(2), LcuPhase::EndOfGame, None, None);
    r.on_lcu(at(30), LcuPhase::Lobby, None, None);
    let effects = r.on_live(at(60), snap(1.0, 0.0312, "Zoe", json!([])));
    match &effects[0] {
        Effect::StartSession { req, .. } => {
            assert_eq!(req.probe.riot_game, None, "the old game id is never reused")
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn exit_seals_partial_chunk_and_persists() {
    let (mut r, local_id) = recording();
    r.on_live(at(1), plain(101.0));
    let effects = r.on_exit(at(2));
    assert_eq!(seals(&effects)[0].snapshots.len(), 2);
    let session = persisted(&effects).unwrap().as_ref().unwrap();
    assert_eq!(
        (session.local_id, session.next_local_seq, session.server),
        (local_id, 1, Some(([9; 16], 0)))
    );
}

#[test]
fn lcu_before_first_snapshot_starts_an_enriched_midgame_session() {
    let mut r = Recorder::new(None, None, "test".into());
    assert!(
        r.on_lcu(
            at(0),
            LcuPhase::InProgress,
            Some(LcuGame {
                game_id: 123,
                queue_id: Some(420),
                is_custom: Some(false)
            }),
            Some(Region::Na)
        )
        .is_empty()
    );
    let effects = r.on_live(at(1), plain(600.0));
    let request = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::StartSession { req, .. } => Some(req),
            _ => None,
        })
        .unwrap();
    assert_eq!(request.region, Some(Region::Na));
    assert_eq!(request.lcu.as_ref().unwrap().game_id, 123);
    assert_eq!(request.probe.riot_game.as_ref().unwrap().game_id, 123);
}

fn with_lcu_game(id: u64) -> Recorder {
    let mut recorder = recorder();
    recorder.on_lcu(
        at(0),
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: id,
            queue_id: Some(420),
            is_custom: Some(false),
        }),
        Some(Region::Na),
    );
    recorder
}

#[test]
fn second_game_starts_when_lcu_misses_the_end_and_loading_phases() {
    let mut r = with_lcu_game(123);
    r.on_live(at(0), plain(100.0));
    r.on_live(
        at(900),
        snap(
            1000.0,
            0.0276,
            "Ahri",
            json!([{"EventID": 1, "EventName": "GameEnd", "EventTime": 1000.0, "Result": "Win"}]),
        ),
    );
    // The next observed phase is still InProgress and its old game metadata
    // must not override the new match's live-client evidence.
    r.on_lcu(
        at(950),
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: 123,
            queue_id: Some(420),
            is_custom: Some(false),
        }),
        None,
    );
    let effects = r.on_live(at(951), snap(10.0, 0.0312, "Zed", json!([])));
    assert_eq!(starts(&effects).len(), 1);
    let request = effects
        .iter()
        .find_map(|e| match e {
            Effect::StartSession { req, .. } => Some(req),
            _ => None,
        })
        .unwrap();
    assert!(request.lcu.is_none());
    assert!(request.probe.riot_game.is_none());
}

#[test]
fn stale_game_id_cannot_merge_games_when_no_end_signal_was_observed() {
    let mut r = with_lcu_game(123);
    r.on_live(at(0), plain(900.0));
    let effects = r.on_live(at(60), snap(10.0, 0.0312, "Zed", json!([])));
    assert_eq!(starts(&effects).len(), 1);
    assert_eq!(ends(&effects)[0].reason, EndReason::Superseded);
    assert_eq!(
        seals(&effects)[0].snapshots.len(),
        1,
        "new match data must not enter the old chunk"
    );
    let request = effects
        .iter()
        .find_map(|e| match e {
            Effect::StartSession { req, .. } => Some(req),
            _ => None,
        })
        .unwrap();
    assert!(request.lcu.is_none());
    // A delayed repeat of the retired id stays ignored.
    r.on_lcu(
        at(61),
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: 123,
            queue_id: None,
            is_custom: None,
        }),
        None,
    );
    let effects = r.on_live(at(62), snap(12.0, 0.0312, "Zed", json!([])));
    assert!(starts(&effects).is_empty());
    r.on_lcu(
        at(63),
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: 456,
            queue_id: None,
            is_custom: None,
        }),
        None,
    );
    let effects = r.on_live(at(64), snap(14.0, 0.0312, "Zed", json!([])));
    assert!(
        starts(&effects).is_empty(),
        "fresh identity attaches to the same new session"
    );
    assert_eq!(
        persisted(&r.on_exit(at(65)))
            .unwrap()
            .as_ref()
            .unwrap()
            .lcu_game_id,
        Some(456)
    );
}

#[test]
fn same_champion_new_game_is_detected_despite_a_cached_id() {
    let mut r = with_lcu_game(123);
    r.on_live(at(0), plain(900.0));
    let effects = r.on_live(at(60), plain(10.0));
    assert_eq!(starts(&effects).len(), 1);
    assert_eq!(ends(&effects)[0].reason, EndReason::Superseded);
}

#[test]
fn fresh_lcu_identity_is_kept_when_a_new_game_supersedes_the_previous_one() {
    let mut r = with_lcu_game(123);
    r.on_live(at(0), plain(900.0));
    r.on_lcu(
        at(60),
        LcuPhase::InProgress,
        Some(LcuGame {
            game_id: 456,
            queue_id: Some(450),
            is_custom: Some(false),
        }),
        None,
    );
    let effects = r.on_live(at(61), snap(10.0, 0.0312, "Zed", json!([])));
    let request = effects
        .iter()
        .find_map(|e| match e {
            Effect::StartSession { req, .. } => Some(req),
            _ => None,
        })
        .unwrap();
    assert_eq!(request.lcu.as_ref().unwrap().game_id, 456);
}

#[test]
fn authoritative_lcu_identity_still_tolerates_wall_clock_adjustments() {
    let mut r = with_lcu_game(123);
    r.on_live(at(0), plain(100.0));
    let effects = r.on_live(at(-30), plain(101.0));
    assert!(starts(&effects).is_empty());
    assert!(ends(&effects).is_empty());
}
