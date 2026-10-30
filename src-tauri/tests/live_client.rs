use tutorlolv2_desktop_app::live_client::{LivePoll, classify};

const VALID_JSON: &[u8] = br#"{
    "activePlayer": {"riotId": "Me#BR1", "level": 1},
    "allPlayers": [{"riotId": "Me#BR1", "championName": "Ahri", "rawSkinName": "game_character_displayname_Ahri", "team": "ORDER"}],
    "events": {"Events": []},
    "gameData": {"gameMode": "CLASSIC", "gameTime": 0.032, "mapNumber": 11}
}"#;

const REPLAY_JSON: &[u8] = br#"{
    "activePlayer": {"error": "Spectator mode doesn't currently support this feature"},
    "allPlayers": [{"riotId": "Me#BR1", "championName": "Ahri", "team": "ORDER"}],
    "events": {"Events": [{"EventID": 0, "EventName": "GameStart", "EventTime": 0.0}]},
    "gameData": {"gameMode": "CLASSIC", "gameTime": 40.4, "mapNumber": 11}
}"#;

const LOADING_404: &[u8] =
    br#"{"errorCode":"RESOURCE_NOT_FOUND","httpStatus":404,"message":"Invalid URI format"}"#;

#[test]
fn classify_cases() {
    assert!(matches!(classify(None, b""), LivePoll::Unreachable));
    assert!(matches!(
        classify(Some(404), LOADING_404),
        LivePoll::NotReady
    ));
    assert!(matches!(
        classify(Some(200), REPLAY_JSON),
        LivePoll::NotReady
    ));
    assert!(matches!(
        classify(Some(500), VALID_JSON),
        LivePoll::NotReady
    ));
    match classify(Some(200), VALID_JSON) {
        LivePoll::Snapshot { bytes, snapshot } => {
            assert_eq!(&bytes[..], VALID_JSON, "bytes are kept verbatim");
            assert_eq!(snapshot.probe.active_riot_id, "Me#BR1");
        }
        _ => panic!("a valid snapshot"),
    }
}
