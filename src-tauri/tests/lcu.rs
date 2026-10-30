use tutorlolv2_desktop_app::lcu::*;
use tutorlolv2_protocol::{GameResult, LcuGame};

#[test]
fn parse_lockfile_cases() {
    let lock = parse_lockfile("LeagueClient:1234:65343:pw:https").unwrap();
    assert_eq!((lock.port, lock.password.as_str()), (65343, "pw"));
    assert!(
        parse_lockfile("LeagueClient:1234:65343:pw:https\r\n").is_some(),
        "trailing newline"
    );
    assert!(parse_lockfile("garbage").is_none());
    assert!(parse_lockfile("LeagueClient:1:notaport:pw:https").is_none());
}

#[test]
fn session_to_game_reads_whitelist_only() {
    let json = br#"{
        "phase": "InProgress",
        "gameClient": {"serverIp": "1.2.3.4", "serverPort": 5000, "running": true},
        "gameData": {"gameId": 5669550383, "isCustomGame": false, "password": "x", "spectatorKey": "y",
                     "queue": {"id": 2400, "type": "KIWI", "gameMode": "KIWI"}, "teamOne": [], "teamTwo": []},
        "map": {"id": 12}
    }"#;
    assert_eq!(
        session_to_game(json),
        Some(LcuGame {
            game_id: 5669550383,
            queue_id: Some(2400),
            is_custom: Some(false)
        })
    );
}

#[test]
fn practice_tool_session_is_custom() {
    let json =
        br#"{"gameData": {"gameId": 5669549208, "isCustomGame": true, "queue": {"id": 3140}}}"#;
    assert_eq!(
        session_to_game(json),
        Some(LcuGame {
            game_id: 5669549208,
            queue_id: Some(3140),
            is_custom: Some(true)
        })
    );
}

#[test]
fn session_without_game_is_none() {
    assert_eq!(
        session_to_game(br#"{"gameData": {"gameId": 0, "queue": {"id": -1}}}"#),
        None
    );
    assert_eq!(
        session_to_game(br#"{"errorCode": "RPC_ERROR", "httpStatus": 404}"#),
        None
    );
}

fn eog(team_won: Option<bool>, early: bool) -> Eog {
    Eog {
        game_id: 1,
        game_length: 1020,
        early_surrender: early,
        player_team_won: team_won,
    }
}

#[test]
fn eog_result_cases() {
    assert_eq!(eog(Some(false), false).result(), GameResult::Lose);
    assert_eq!(eog(Some(true), false).result(), GameResult::Win);
    assert_eq!(eog(Some(true), true).result(), GameResult::Remake);
    assert_eq!(eog(None, false).result(), GameResult::Unknown);
}

#[test]
fn parse_eog_reads_player_team() {
    let json = br#"{
        "gameId": 5669550383, "gameLength": 1020, "gameEndedInEarlySurrender": false,
        "mucJwtDto": {"jwt": "secret"}, "multiUserChatPassword": "secret",
        "localPlayer": {"puuid": "4c2f37eb-2fd0-0000-0000-000000000000"},
        "teams": [{"teamId": 100, "isPlayerTeam": true, "isWinningTeam": false}, {"teamId": 200, "isPlayerTeam": false, "isWinningTeam": true}]
    }"#;
    assert_eq!(
        parse_eog(json),
        Some(Eog {
            game_id: 5669550383,
            game_length: 1020,
            early_surrender: false,
            player_team_won: Some(false)
        })
    );
    assert_eq!(parse_eog(br#"{"errorCode": "RPC_ERROR"}"#), None);
}

#[test]
fn phase_parsing() {
    assert_eq!(LcuPhase::from("PreEndOfGame"), LcuPhase::PreEndOfGame);
    assert!(LcuPhase::from("Lobby").is_out_of_game());
    assert!(LcuPhase::from("EndOfGame").is_end());
    assert!(
        !LcuPhase::from("InProgress").is_end() && !LcuPhase::from("InProgress").is_out_of_game()
    );
    assert!(!LcuPhase::from("Reconnect").is_out_of_game());
    assert_eq!(LcuPhase::from("Foo").as_str(), "Foo");
    assert_eq!(
        serde_json::from_slice::<LcuPhase>(br#""InProgress""#).unwrap(),
        LcuPhase::InProgress
    );
}

const CAPTURE: &str =
    r"\\wsl.localhost\Ubuntu\home\luizg\Projects\tutorlolv2\captures\aram-20261029-101315\lcu";

#[test]
#[ignore = "needs local captures"]
fn real_captures_parse() {
    let session = std::fs::read(format!(r"{CAPTURE}\00152-session-InProgress.json")).unwrap();
    assert_eq!(
        session_to_game(&session).map(|g| g.game_id),
        Some(5669550383)
    );
    let eog = parse_eog(&std::fs::read(format!(r"{CAPTURE}\01227-eog-stats-block.json")).unwrap())
        .unwrap();
    assert_eq!(
        eog,
        Eog {
            game_id: 5669550383,
            game_length: 1020,
            early_surrender: false,
            player_team_won: Some(false)
        }
    );
    assert_eq!(eog.result(), GameResult::Lose);
}

#[test]
#[ignore = "needs a running League client"]
fn live_client_is_discovered_and_answers() {
    let lock = discover().expect("lockfile of the running client");
    assert!(lock.port > 0);
    tauri::async_runtime::block_on(async {
        let mut lcu = Lcu::new();
        let phase = lcu.phase().await.expect("gameflow phase");
        let region = lcu.region().await.expect("region");
        println!("phase {phase}, region {region}");
    });
}
