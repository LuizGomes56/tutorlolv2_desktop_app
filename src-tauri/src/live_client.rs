//! Port 2999, the game's Live Client Data API (spec §6.2).

use std::{sync::Arc, time::Duration};
use tauri_plugin_http::reqwest::Client;
use tutorlolv2_protocol::{Snapshot, inspect};

const ALL_GAME_DATA: &str = "https://127.0.0.1:2999/liveclientdata/allgamedata";
const TIMEOUT: Duration = Duration::from_secs(2);

/// One poll of port 2999.
pub enum LivePoll {
    /// A valid snapshot; `bytes` is the response body, verbatim.
    Snapshot {
        bytes: Arc<[u8]>,
        snapshot: Snapshot,
    },
    /// The port answered without a game this app can record: loading screen
    /// (`404`), replay or spectator mode, or any other error body.
    NotReady,
    /// Connection refused or timed out: no game is running.
    Unreachable,
}

/// `status` is `None` when the request itself failed.
pub fn classify(status: Option<u16>, body: &[u8]) -> LivePoll {
    match status {
        None => LivePoll::Unreachable,
        Some(200) => match inspect(body) {
            Ok(snapshot) => LivePoll::Snapshot {
                bytes: Arc::from(body),
                snapshot,
            },
            Err(_) => LivePoll::NotReady,
        },
        Some(_) => LivePoll::NotReady,
    }
}

pub struct LiveClient {
    http: Client,
}

impl LiveClient {
    pub fn new() -> Self {
        let http = Client::builder()
            // The game serves 127.0.0.1 with Riot's self-signed certificate
            .danger_accept_invalid_certs(true)
            .timeout(TIMEOUT)
            .build()
            .expect("static client configuration is valid");

        Self { http }
    }

    pub async fn poll(&self) -> LivePoll {
        let Ok(response) = self.http.get(ALL_GAME_DATA).send().await else {
            return classify(None, &[]);
        };

        let status = response.status().as_u16();

        match response.bytes().await {
            Ok(body) => classify(Some(status), &body),
            Err(_) => classify(None, &[]),
        }
    }
}

impl Default for LiveClient {
    fn default() -> Self {
        Self::new()
    }
}
