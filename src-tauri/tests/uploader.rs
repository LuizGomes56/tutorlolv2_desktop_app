use std::io::Read;
use tutorlolv2_desktop_app::uploader::*;
use tutorlolv2_protocol::{RAW_CHUNK_FORMAT, RawChunk, RawSnapshot};

fn chunk(seq: u32) -> RawChunk {
    RawChunk {
        format: RAW_CHUNK_FORMAT,
        seq,
        snapshots: vec![RawSnapshot {
            captured_at_ms: 1,
            game_time: 1.0,
            json: b"{}".to_vec(),
        }],
    }
}

fn decode(body: &[u8]) -> (RawChunk, Vec<u8>) {
    let mut raw = Vec::new();
    brotli::Decompressor::new(body, 4096)
        .read_to_end(&mut raw)
        .unwrap();
    let (chunk, used) = bincode::decode_from_slice(&raw, bincode::config::standard()).unwrap();
    assert_eq!(used, raw.len());
    (chunk, raw)
}

#[test]
fn encode_chunk_compresses_and_hashes_the_bincode() {
    let (body, sha) = encode_chunk(&chunk(2));
    let (decoded, raw) = decode(&body);
    assert_eq!(decoded, chunk(2));
    assert_eq!(sha.len(), 64);
    use sha2::{Digest, Sha256};
    let expected: String = Sha256::digest(&raw)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(sha, expected);
}

#[test]
fn renumber_changes_only_seq() {
    let (body, sha) = encode_chunk(&chunk(0));
    let (renumbered, new_sha) = renumber(&body, 7).unwrap();
    let (decoded, _) = decode(&renumbered);
    assert_eq!(decoded.seq, 7);
    assert_eq!(decoded.snapshots, chunk(0).snapshots);
    assert_ne!(new_sha, sha);
}

#[test]
fn device_auth_header_format() {
    let id: [u8; 16] = std::array::from_fn(|i| i as u8);
    let header = device_auth_header(&id, &[0xFF; 32]);
    let (uuid, secret) = header
        .strip_prefix("Device ")
        .unwrap()
        .split_once('.')
        .unwrap();
    assert_eq!(uuid, "00010203-0405-0607-0809-0a0b0c0d0e0f");
    assert_eq!(secret.len(), 43);
    assert!(
        secret
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    );
}

#[test]
fn backoff_is_bounded() {
    assert_eq!(backoff_ms(0, 0.0), 500);
    assert_eq!(backoff_ms(1, 0.0), 1000);
    assert_eq!(backoff_ms(20, 0.999_999), 299_999);
}

#[test]
fn status_429_is_retry_not_reject() {
    assert!(matches!(
        chunk_outcome(429, Some("7")),
        ChunkOutcome::RateLimited {
            after_ms: Some(7000)
        }
    ));
    assert!(matches!(
        chunk_outcome(503, None),
        ChunkOutcome::Retry { after_ms: None }
    ));
    assert!(matches!(chunk_outcome(201, None), ChunkOutcome::Stored));
    assert!(matches!(chunk_outcome(200, None), ChunkOutcome::Stored));
    assert!(matches!(
        chunk_outcome(409, None),
        ChunkOutcome::Rejected(409)
    ));
    assert!(matches!(
        chunk_outcome(422, None),
        ChunkOutcome::Rejected(422)
    ));
}

#[test]
fn body_sha_matches_encode_chunk() {
    let (body, sha) = encode_chunk(&chunk(5));
    assert_eq!(body_sha(&body).unwrap(), sha);
    assert!(body_sha(b"not brotli").is_err());
}

#[test]
fn auth_and_timeouts_never_reject_chunks() {
    assert!(matches!(
        chunk_outcome(401, None),
        ChunkOutcome::Unauthorized
    ));
    assert!(matches!(
        chunk_outcome(408, None),
        ChunkOutcome::Retry { after_ms: None }
    ));
    assert!(matches!(
        chunk_outcome(404, None),
        ChunkOutcome::Rejected(404)
    ));
}
