//! HTTP calls to the TutorLoL server (spec §10.2) and chunk encoding.

use crate::BINCODE_CONFIG;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Read, Write},
    time::Duration,
};
use tauri_plugin_http::reqwest::{Client, Method, StatusCode};
use tutorlolv2_protocol::{
    BROTLI_LGWIN, BROTLI_QUALITY, DeviceCredentials, EndRecording, Id16, MAX_CHUNK_DECOMPRESSED,
    RawChunk, RawSnapshot, RecordingResult, RegisterDevice, SessionStarted, SetShareMode,
    ShareMode, ShareState, StartSession,
};

const OCTET_STREAM: &str = "application/octet-stream";
const CONTENT_SHA256: &str = "X-Content-Sha256";
const MAX_BACKOFF_MS: u64 = 300_000;

fn sha_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Upload body (`brotli(bincode(chunk))`) and its `X-Content-Sha256`.
pub fn encode_chunk(chunk: &RawChunk) -> (Vec<u8>, String) {
    let raw =
        bincode::encode_to_vec(chunk, BINCODE_CONFIG).expect("encoding into a Vec cannot fail");

    let mut body = Vec::with_capacity(raw.len() / 8);

    {
        let mut writer =
            brotli::CompressorWriter::new(&mut body, 4096, BROTLI_QUALITY, BROTLI_LGWIN);

        writer
            .write_all(&raw)
            .expect("writing into a Vec cannot fail");
    }

    (body, sha_hex(&raw))
}

fn decompress(body: &[u8]) -> io::Result<Vec<u8>> {
    let mut raw = Vec::new();

    brotli::Decompressor::new(body, 4096)
        .take(MAX_CHUNK_DECOMPRESSED as u64 + 1)
        .read_to_end(&mut raw)?;

    Ok(raw)
}

/// `X-Content-Sha256` of a spooled upload body.
pub fn body_sha(body: &[u8]) -> io::Result<String> {
    Ok(sha_hex(&decompress(body)?))
}

/// Rewrites the seq inside a spooled chunk (only for resumed recordings, spec §7.2).
pub fn renumber(body: &[u8], new_seq: u32) -> io::Result<(Vec<u8>, String)> {
    let raw = decompress(body)?;
    let (mut chunk, _): (RawChunk, _) =
        bincode::decode_from_slice(&raw, BINCODE_CONFIG).map_err(io::Error::other)?;

    chunk.seq = new_seq;

    Ok(encode_chunk(&chunk))
}

/// Hyphenated lowercase UUID text, as the server's routes expect.
fn uuid_text(id: &Id16) -> String {
    let hex: String = id.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    )
}

/// `Device <uuid-hyphenated>.<base64url-nopad(secret)>` (spec §13.1).
pub fn device_auth_header(device_id: &Id16, secret: &[u8; 32]) -> String {
    format!(
        "Device {}.{}",
        uuid_text(device_id),
        URL_SAFE_NO_PAD.encode(secret)
    )
}

/// Exponential backoff with jitter: 1 s, 2 s, 4 s … capped at 5 min, scaled
/// by `0.5 + jitter / 2` (`jitter` ∈ [0, 1)).
pub fn backoff_ms(attempt: u32, jitter: f64) -> u64 {
    let base = 1000u64
        .saturating_mul(1u64.checked_shl(attempt).unwrap_or(u64::MAX))
        .min(MAX_BACKOFF_MS);

    (base as f64 * (0.5 + jitter / 2.0)) as u64
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkOutcome {
    Stored,
    /// Rate limited or server trouble: keep the chunk and try later.
    Retry {
        after_ms: Option<u64>,
    },
    /// Stop the pass when the server asks to slow down.
    RateLimited {
        after_ms: Option<u64>,
    },
    /// The device credentials are no longer valid (e.g. a database reset):
    /// keep the chunk and register again.
    Unauthorized,
    /// The server will never accept this chunk.
    Rejected(u16),
}

fn retry_delay(value: Option<&str>) -> Option<u64> {
    value
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(|s| s.saturating_mul(1000).min(MAX_BACKOFF_MS))
}

pub fn chunk_outcome(status: u16, retry_after: Option<&str>) -> ChunkOutcome {
    match status {
        200 | 201 => ChunkOutcome::Stored,
        401 => ChunkOutcome::Unauthorized,
        429 => ChunkOutcome::RateLimited {
            after_ms: retry_delay(retry_after),
        },
        408 | 500..=599 => ChunkOutcome::Retry {
            after_ms: retry_delay(retry_after),
        },
        other => ChunkOutcome::Rejected(other),
    }
}

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    base: String,
    auth: Option<String>,
}

impl ApiClient {
    /// Server origin, set at build time with `TUTORLOLV2_API_URL`.
    pub const BASE: &'static str = match option_env!("TUTORLOLV2_API_URL") {
        Some(url) => url,
        None => "http://127.0.0.1:8082",
    };

    pub fn new(credentials: Option<&DeviceCredentials>) -> Self {
        Self::with_base_url(Self::BASE.to_owned(), credentials)
    }

    pub fn with_base_url(base: String, credentials: Option<&DeviceCredentials>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("static client configuration is valid");

        let auth = credentials.map(|c| device_auth_header(&c.device_id, &c.device_secret));

        Self { http, base, auth }
    }

    pub fn authenticated(&self) -> bool {
        self.auth.is_some()
    }

    async fn send(
        &self,
        method: Method,
        path: &str,
        body: Vec<u8>,
        sha: Option<&str>,
    ) -> Result<(StatusCode, Option<String>, Vec<u8>), String> {
        let mut request = self
            .http
            .request(method, format!("{}/api{path}", self.base))
            .header("Content-Type", OCTET_STREAM)
            .body(body);

        if let Some(auth) = &self.auth {
            request = request.header("Authorization", auth);
        }

        if let Some(sha) = sha {
            request = request.header(CONTENT_SHA256, sha);
        }

        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let retry_after = response
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);

        let bytes = response.bytes().await.map_err(|e| e.to_string())?.to_vec();

        Ok((status, retry_after, bytes))
    }

    async fn call<T: Decode<()>>(
        &self,
        method: Method,
        path: &str,
        body: impl Encode,
    ) -> Result<Option<T>, String> {
        let body = bincode::encode_to_vec(body, BINCODE_CONFIG).map_err(|e| e.to_string())?;

        let (status, _, bytes) = self.send(method, path, body, None).await?;

        if !status.is_success() {
            return Err(format!("{status}: {}", String::from_utf8_lossy(&bytes)));
        }

        if status == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        bincode::decode_from_slice(&bytes, BINCODE_CONFIG)
            .map(|(value, _)| Some(value))
            .map_err(|e| e.to_string())
    }

    pub async fn register(&self, app_version: &str) -> Result<DeviceCredentials, String> {
        self.call(
            Method::POST,
            "/devices",
            RegisterDevice {
                app_version: app_version.into(),
            },
        )
        .await?
        .ok_or_else(|| "empty registration response".into())
    }

    pub async fn start_session(&self, request: &StartSession) -> Result<SessionStarted, String> {
        self.call(Method::POST, "/recordings/sessions", request)
            .await?
            .ok_or_else(|| "empty session response".into())
    }

    pub async fn upload_chunk(
        &self,
        recording_id: &Id16,
        seq: u32,
        body: Vec<u8>,
        sha_hex: &str,
    ) -> ChunkOutcome {
        let path = format!("/recordings/{}/chunks/{seq}", uuid_text(recording_id));

        match self.send(Method::POST, &path, body, Some(sha_hex)).await {
            Ok((status, retry_after, _)) => chunk_outcome(status.as_u16(), retry_after.as_deref()),
            Err(_) => ChunkOutcome::Retry { after_ms: None },
        }
    }

    pub async fn share(&self, id: &Id16) -> Result<ShareState, String> {
        self.call(
            Method::GET,
            &format!("/recordings/{}/share", uuid_text(id)),
            (),
        )
        .await?
        .ok_or_else(|| "empty share response".into())
    }

    pub async fn set_share_mode(&self, id: &Id16, mode: ShareMode) -> Result<ShareState, String> {
        self.call(
            Method::PUT,
            &format!("/recordings/{}/share", uuid_text(id)),
            SetShareMode { mode },
        )
        .await?
        .ok_or_else(|| "empty share response".into())
    }

    pub async fn upload_live(&self, id: &Id16, snapshot: &RawSnapshot) -> Result<(), String> {
        let raw = bincode::encode_to_vec(snapshot, BINCODE_CONFIG).map_err(|e| e.to_string())?;

        if raw.len() > 512 * 1024 {
            return Err("live snapshot exceeds size limit".into());
        }

        let mut body = Vec::new();

        {
            let mut writer =
                brotli::CompressorWriter::new(&mut body, 4096, BROTLI_QUALITY, BROTLI_LGWIN);

            writer.write_all(&raw).map_err(|e| e.to_string())?;
        }

        if body.len() > 64 * 1024 {
            return Err("compressed live snapshot exceeds size limit".into());
        }

        let (status, _, bytes) = self
            .send(
                Method::PUT,
                &format!("/recordings/{}/live", uuid_text(id)),
                body,
                None,
            )
            .await?;

        if status.is_success() {
            Ok(())
        } else {
            Err(format!("{status}: {}", String::from_utf8_lossy(&bytes)))
        }
    }

    pub async fn heartbeat(&self, recording_id: &Id16) -> Result<(), String> {
        let path = format!("/recordings/{}/heartbeat", uuid_text(recording_id));

        self.call::<()>(Method::POST, &path, ()).await.map(drop)
    }

    pub async fn end(&self, recording_id: &Id16, request: &EndRecording) -> Result<(), String> {
        let path = format!("/recordings/{}/end", uuid_text(recording_id));

        self.call::<()>(Method::POST, &path, request)
            .await
            .map(drop)
    }

    pub async fn result(
        &self,
        recording_id: &Id16,
        request: &RecordingResult,
    ) -> Result<(), String> {
        let path = format!("/recordings/{}/result", uuid_text(recording_id));

        self.call::<()>(Method::PATCH, &path, request)
            .await
            .map(drop)
    }
}
