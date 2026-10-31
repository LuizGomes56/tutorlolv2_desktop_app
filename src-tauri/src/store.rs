//! Local state in `%LOCALAPPDATA%\com.tutorlolv2` (spec §12.1): versioned
//! bincode records, written atomically. Unreadable files are moved aside as
//! `*.corrupt`, never deleted.

use crate::BINCODE_CONFIG;
use crate::recorder::SessionFile;
use bincode::{Decode, Encode};
use std::{
    fs, io,
    io::Write,
    path::{Path, PathBuf},
};
use tutorlolv2_protocol::{DeviceCredentials, Id16};

const MAGIC: &[u8; 4] = b"TLV2";
const VERSION: u16 = 1;
const HEADER_LEN: usize = 7;
const DEVICE_FILE: &str = "device.bin";
const SESSION_FILE: &str = "session.bin";

#[derive(Clone, Copy)]
#[repr(u8)]
enum Kind {
    Device = 1,
    Session = 2,
}

/// `device.bin` payload: the secret is encrypted for the current Windows user.
#[derive(Encode, Decode)]
struct DeviceRecord {
    device_id: Id16,
    protected_secret: Vec<u8>,
}

pub struct Store {
    dir: PathBuf,
}

impl Store {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn load_device(&self) -> Option<DeviceCredentials> {
        self.load(DEVICE_FILE, Kind::Device, |payload| {
            let record: DeviceRecord = decode(payload)?;
            let secret = unprotect(&record.protected_secret).ok()?;

            Some(DeviceCredentials {
                device_id: record.device_id,
                device_secret: secret.try_into().ok()?,
            })
        })
    }

    pub fn save_device(&self, credentials: &DeviceCredentials) -> io::Result<()> {
        let record = DeviceRecord {
            device_id: credentials.device_id,
            protected_secret: protect(&credentials.device_secret)?,
        };

        self.save(DEVICE_FILE, Kind::Device, &record)
    }

    pub fn load_session(&self) -> Option<SessionFile> {
        self.load(SESSION_FILE, Kind::Session, decode)
    }

    /// `None` moves the file aside as `session.bin.done`.
    pub fn save_session(&self, session: Option<&SessionFile>) -> io::Result<()> {
        match session {
            Some(session) => self.save(SESSION_FILE, Kind::Session, session),
            None => match fs::rename(
                self.dir.join(SESSION_FILE),
                self.dir.join("session.bin.done"),
            ) {
                Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
                other => other,
            },
        }
    }

    fn load<T>(&self, name: &str, kind: Kind, parse: impl FnOnce(&[u8]) -> Option<T>) -> Option<T> {
        let path = self.dir.join(name);
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return None,
            Err(_) => return move_aside(&path),
        };

        let header_ok = bytes.len() >= HEADER_LEN
            && &bytes[..4] == MAGIC
            && u16::from_le_bytes([bytes[4], bytes[5]]) == VERSION
            && bytes[6] == kind as u8;

        match header_ok.then(|| parse(&bytes[HEADER_LEN..])).flatten() {
            Some(value) => Some(value),
            None => move_aside(&path),
        }
    }

    fn save(&self, name: &str, kind: Kind, value: &impl Encode) -> io::Result<()> {
        fs::create_dir_all(&self.dir)?;

        let mut bytes = Vec::with_capacity(256);

        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.push(kind as u8);

        bincode::encode_into_std_write(value, &mut bytes, BINCODE_CONFIG)
            .map_err(io::Error::other)?;

        let path = self.dir.join(name);
        let tmp = path.with_extension("bin.tmp");
        let mut file = fs::File::create(&tmp)?;

        file.write_all(&bytes)?;
        file.sync_all()?;

        drop(file);
        fs::rename(tmp, path)
    }
}

fn decode<T: Decode<()>>(payload: &[u8]) -> Option<T> {
    let (value, used) = bincode::decode_from_slice(payload, BINCODE_CONFIG).ok()?;
    (used == payload.len()).then_some(value)
}

/// Keeps an unreadable file for diagnosis and starts fresh.
fn move_aside<T>(path: &Path) -> Option<T> {
    let mut corrupt = path.as_os_str().to_owned();

    corrupt.push(".corrupt");

    let _ = fs::rename(path, corrupt);
    None
}

/// Encrypts for the current Windows user (DPAPI).
#[cfg(windows)]
fn protect(data: &[u8]) -> io::Result<Vec<u8>> {
    use windows::{
        Win32::Security::Cryptography::{
            CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData,
        },
        core::PCWSTR,
    };

    let input = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };

    let mut output = CRYPT_INTEGER_BLOB::default();

    unsafe {
        CryptProtectData(
            &input,
            PCWSTR::null(),
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    }
    .map_err(io::Error::other)?;

    Ok(take_blob(output))
}

#[cfg(windows)]
fn unprotect(data: &[u8]) -> io::Result<Vec<u8>> {
    use windows::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData,
    };

    let input = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };

    let mut output = CRYPT_INTEGER_BLOB::default();

    unsafe {
        CryptUnprotectData(
            &input,
            None,
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    }
    .map_err(io::Error::other)?;

    Ok(take_blob(output))
}

/// Copies a DPAPI output buffer and frees it.
#[cfg(windows)]
fn take_blob(blob: windows::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB) -> Vec<u8> {
    use windows::Win32::Foundation::{HLOCAL, LocalFree};

    let bytes = unsafe { std::slice::from_raw_parts(blob.pbData, blob.cbData as usize) }.to_vec();

    unsafe { LocalFree(Some(HLOCAL(blob.pbData as _))) };

    bytes
}

#[cfg(not(windows))]
fn protect(data: &[u8]) -> io::Result<Vec<u8>> {
    Ok(data.to_vec())
}

#[cfg(not(windows))]
fn unprotect(data: &[u8]) -> io::Result<Vec<u8>> {
    Ok(data.to_vec())
}
