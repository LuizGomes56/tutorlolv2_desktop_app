//! The League client's local API (LCU, spec §9). Optional: every feature
//! works without it. Only whitelisted fields are ever deserialized; the
//! session's passwords, spectator key and server address, and the end-of-game
//! chat credentials are never read.

use serde::Deserialize;
use std::time::Duration;
use tauri_plugin_http::reqwest::{Client, StatusCode};
use tutorlolv2_protocol::{GameResult, LcuGame, Region, open_enum};

const TIMEOUT: Duration = Duration::from_secs(3);

#[cfg(windows)]
const CLIENT_EXE: &str = "LeagueClientUx.exe";

#[cfg(windows)]
const FALLBACK_LOCKFILE: &str = r"C:\Riot Games\League of Legends\lockfile";

/// `LeagueClient:<pid>:<port>:<password>:https`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lockfile {
    pub port: u16,
    pub password: String,
}

pub fn parse_lockfile(contents: &str) -> Option<Lockfile> {
    let mut parts = contents.trim().split(':');
    let (_name, _pid) = (parts.next()?, parts.next()?);
    let port = parts.next()?.parse().ok()?;
    let password = parts.next()?.to_owned();

    (parts.next()? == "https" && !password.is_empty()).then_some(Lockfile { port, password })
}

open_enum!(
    /// `GET /lol-gameflow/v1/gameflow-phase`
    pub enum LcuPhase {
        None = "None",
        Lobby = "Lobby",
        Matchmaking = "Matchmaking",
        ReadyCheck = "ReadyCheck",
        ChampSelect = "ChampSelect",
        GameStart = "GameStart",
        InProgress = "InProgress",
        Reconnect = "Reconnect",
        WaitingForStats = "WaitingForStats",
        PreEndOfGame = "PreEndOfGame",
        EndOfGame = "EndOfGame",
    }
);

impl LcuPhase {
    /// The game is over and its end-of-game stats are (or will be) available.
    pub fn is_end(&self) -> bool {
        matches!(
            self,
            Self::WaitingForStats | Self::PreEndOfGame | Self::EndOfGame
        )
    }

    /// The player is outside any game.
    pub fn is_out_of_game(&self) -> bool {
        matches!(
            self,
            Self::None | Self::Lobby | Self::Matchmaking | Self::ReadyCheck | Self::ChampSelect
        )
    }
}

/// Whitelisted fields of `GET /lol-end-of-game/v1/eog-stats-block`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Eog {
    pub game_id: u64,
    pub game_length: u32,
    pub early_surrender: bool,
    pub player_team_won: Option<bool>,
}

impl Eog {
    pub fn result(&self) -> GameResult {
        match (self.early_surrender, self.player_team_won) {
            (true, _) => GameResult::Remake,
            (false, Some(true)) => GameResult::Win,
            (false, Some(false)) => GameResult::Lose,
            (false, None) => GameResult::Unknown,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionDto {
    game_data: GameDataDto,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameDataDto {
    game_id: u64,
    #[serde(default)]
    is_custom_game: Option<bool>,
    #[serde(default)]
    queue: Option<QueueDto>,
}

#[derive(Deserialize)]
struct QueueDto {
    id: i64,
}

/// `GET /lol-gameflow/v1/session`; `None` outside a game (`gameId` 0).
pub fn session_to_game(json: &[u8]) -> Option<LcuGame> {
    let session = serde_json::from_slice::<SessionDto>(json).ok()?;
    let game = session.game_data;

    (game.game_id != 0).then(|| LcuGame {
        game_id: game.game_id,
        queue_id: game.queue.and_then(|q| u16::try_from(q.id).ok()),
        is_custom: game.is_custom_game,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EogDto {
    game_id: u64,
    game_length: u32,
    #[serde(default)]
    game_ended_in_early_surrender: bool,
    #[serde(default)]
    teams: Vec<TeamDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TeamDto {
    is_player_team: bool,
    is_winning_team: bool,
}

pub fn parse_eog(json: &[u8]) -> Option<Eog> {
    let eog = serde_json::from_slice::<EogDto>(json).ok()?;

    Some(Eog {
        game_id: eog.game_id,
        game_length: eog.game_length,
        early_surrender: eog.game_ended_in_early_surrender,
        player_team_won: eog
            .teams
            .iter()
            .find(|t| t.is_player_team)
            .map(|t| t.is_winning_team),
    })
}

#[derive(Deserialize)]
struct RegionDto {
    region: Region,
}

/// Finds the running client's lockfile without elevation (spec §9.2).
#[cfg(windows)]
pub fn discover() -> Option<Lockfile> {
    client_dir()
        .map(|dir| dir.join("lockfile"))
        .into_iter()
        .chain([std::path::PathBuf::from(FALLBACK_LOCKFILE)])
        .find_map(|path| parse_lockfile(&std::fs::read_to_string(path).ok()?))
}

#[cfg(not(windows))]
pub fn discover() -> Option<Lockfile> {
    None
}

/// Folder of the running `LeagueClientUx.exe`.
#[cfg(windows)]
fn client_dir() -> Option<std::path::PathBuf> {
    use windows::{
        Win32::{
            Foundation::CloseHandle,
            System::{
                Diagnostics::ToolHelp::{
                    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
                    TH32CS_SNAPPROCESS,
                },
                Threading::{
                    OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
                    QueryFullProcessImageNameW,
                },
            },
        },
        core::PWSTR,
    };

    let pid = unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;

        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        let mut found = None;
        let mut more = Process32FirstW(snapshot, &mut entry).is_ok();

        while more {
            let len = entry
                .szExeFile
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(entry.szExeFile.len());

            if String::from_utf16_lossy(&entry.szExeFile[..len]).eq_ignore_ascii_case(CLIENT_EXE) {
                found = Some(entry.th32ProcessID);
                break;
            }

            more = Process32NextW(snapshot, &mut entry).is_ok();
        }

        let _ = CloseHandle(snapshot);
        found?
    };

    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buffer = [0u16; 1024];
        let mut size = buffer.len() as u32;

        let ok = QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
        .is_ok();

        let _ = CloseHandle(process);

        if !ok {
            return None;
        }

        std::path::PathBuf::from(String::from_utf16_lossy(&buffer[..size as usize]))
            .parent()
            .map(std::path::Path::to_path_buf)
    }
}

pub struct Lcu {
    http: Client,
    lock: Option<Lockfile>,
}

impl Lcu {
    pub fn new() -> Self {
        let http = Client::builder()
            // The client's HTTP/2 handshake fails (spec §20, V3)
            .http1_only()
            // Riot's self-signed certificate on 127.0.0.1
            .danger_accept_invalid_certs(true)
            .timeout(TIMEOUT)
            .build()
            .expect("static client configuration is valid");

        Self { http, lock: None }
    }

    /// GET a path; forgets the lockfile when the client restarted (connection
    /// refused or 401), so the next call rediscovers it.
    async fn get(&mut self, path: &str) -> Option<Vec<u8>> {
        if self.lock.is_none() {
            self.lock = discover();
        }

        let lock = self.lock.as_ref()?;
        let response = self
            .http
            .get(format!("https://127.0.0.1:{}{path}", lock.port))
            .basic_auth("riot", Some(&lock.password))
            .send()
            .await;

        match response {
            Ok(response) if response.status() == StatusCode::OK => {
                response.bytes().await.ok().map(|b| b.to_vec())
            }
            Ok(response) if response.status() == StatusCode::UNAUTHORIZED => {
                self.lock = None;
                None
            }
            Ok(_) => None,
            Err(_) => {
                self.lock = None;
                None
            }
        }
    }

    pub async fn phase(&mut self) -> Option<LcuPhase> {
        serde_json::from_slice(&self.get("/lol-gameflow/v1/gameflow-phase").await?).ok()
    }

    pub async fn game(&mut self) -> Option<LcuGame> {
        session_to_game(&self.get("/lol-gameflow/v1/session").await?)
    }

    pub async fn region(&mut self) -> Option<Region> {
        let dto =
            serde_json::from_slice::<RegionDto>(&self.get("/riotclient/region-locale").await?)
                .ok()?;

        Some(dto.region)
    }

    pub async fn eog(&mut self) -> Option<Eog> {
        parse_eog(&self.get("/lol-end-of-game/v1/eog-stats-block").await?)
    }
}

impl Default for Lcu {
    fn default() -> Self {
        Self::new()
    }
}
