//! Platform-independent shortcut validation, persistence and keyboard transitions.
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shortcut {
    pub code: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

impl Default for Shortcut {
    fn default() -> Self {
        Self {
            code: "KeyK".into(),
            ctrl: true,
            alt: false,
            shift: false,
            meta: false,
        }
    }
}

impl Shortcut {
    pub fn validate(&self) -> Result<u32, String> {
        let code = self.code.as_str();

        let vk = match code {
            _ if code.len() == 4
                && code.starts_with("Key")
                && code.as_bytes()[3].is_ascii_uppercase() =>
            {
                code.as_bytes()[3] as u32
            }
            _ if code.len() == 6
                && code.starts_with("Digit")
                && code.as_bytes()[5].is_ascii_digit() =>
            {
                code.as_bytes()[5] as u32
            }
            _ if code.starts_with('F') => code[1..]
                .parse::<u32>()
                .ok()
                .filter(|n| (1..=24).contains(n))
                .map(|n| 0x6f + n)
                .ok_or("Use F1 through F24")?,
            "Space" => 0x20,
            "Enter" => 0x0d,
            "Tab" => 0x09,
            "Backspace" => 0x08,
            "Insert" => 0x2d,
            "Delete" => 0x2e,
            "Home" => 0x24,
            "End" => 0x23,
            "PageUp" => 0x21,
            "PageDown" => 0x22,
            "ArrowLeft" => 0x25,
            "ArrowUp" => 0x26,
            "ArrowRight" => 0x27,
            "ArrowDown" => 0x28,
            "Semicolon" => 0xba,
            "Equal" => 0xbb,
            "Comma" => 0xbc,
            "Minus" => 0xbd,
            "Period" => 0xbe,
            "Slash" => 0xbf,
            "Backquote" => 0xc0,
            "BracketLeft" => 0xdb,
            "Backslash" => 0xdc,
            "BracketRight" => 0xdd,
            "Quote" => 0xde,
            "Escape" => return Err("Escape is reserved for closing the overlay".into()),
            _ => {
                return Err(
                    "Choose a letter, number, function key, navigation key or punctuation key"
                        .into(),
                );
            }
        };

        let binding = vk
            | (u32::from(self.ctrl) << 9)
            | (u32::from(self.alt) << 10)
            | (u32::from(self.shift) << 11)
            | (u32::from(self.meta) << 12);

        if binding == 0x24f {
            return Err("Ctrl+O is reserved for cycling enemies".into());
        }

        Ok(binding)
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        match fs::read(path) {
            Ok(bytes) => {
                let binding = serde_json::from_slice::<Self>(&bytes)
                    .map_err(|e| format!("Invalid shortcut settings: {e}"))?;

                binding.validate()?;

                Ok(binding)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(format!("Cannot read shortcut settings: {e}")),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        self.validate()?;

        let bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        let temporary = path.with_extension("json.tmp");

        let write = || -> std::io::Result<()> {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = fs::File::create(&temporary)?;

            file.write_all(&bytes)?;
            file.sync_all()?;

            drop(file);

            replace_file(&temporary, path)
        };

        write().map_err(|e| {
            let _ = fs::remove_file(&temporary);
            format!("Cannot save shortcut settings: {e}")
        })
    }
}

#[cfg(not(windows))]
fn replace_file(source: &Path, target: &Path) -> std::io::Result<()> {
    fs::rename(source, target)
}

#[cfg(windows)]
fn replace_file(source: &Path, target: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{
        Win32::Storage::FileSystem::{
            MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
        },
        core::PCWSTR,
    };

    let source = source
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>();

    let target = target
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>();

    unsafe {
        MoveFileExW(
            PCWSTR(source.as_ptr()),
            PCWSTR(target.as_ptr()),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    }
    .map_err(std::io::Error::other)
}

#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Toggle,
    Close,
    Change,
}

#[cfg(any(windows, test))]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Decision {
    pub action: Option<Action>,
    pub consume: bool,
}

/// Normalize physical browser key codes independently of the active Windows layout.
#[cfg(any(windows, test))]
pub fn physical_key(vk: u32, scan: u32, extended: bool) -> u32 {
    // Scan Set 1 positions correspond to KeyboardEvent.code, not the typed character.
    if !extended {
        match scan {
            0x02..=0x0a => return b'1' as u32 + scan - 0x02,
            0x0b => return b'0' as u32,
            0x10..=0x19 => return b"QWERTYUIOP"[(scan - 0x10) as usize] as u32,
            0x1e..=0x26 => return b"ASDFGHJKL"[(scan - 0x1e) as usize] as u32,
            0x2c..=0x32 => return b"ZXCVBNM"[(scan - 0x2c) as usize] as u32,
            0x0c => return 0xbd,
            0x0d => return 0xbb,
            0x1a => return 0xdb,
            0x1b => return 0xdd,
            0x27 => return 0xba,
            0x28 => return 0xde,
            0x29 => return 0xc0,
            0x2b => return 0xdc,
            0x33 => return 0xbc,
            0x34 => return 0xbe,
            0x35 => return 0xbf,
            _ => {}
        }
    }

    if (!extended && matches!(vk, 0x21..=0x28 | 0x2d | 0x2e))
        || (extended && (scan == 0x1c || scan == 0x35))
    {
        return 0xff;
    }

    vk
}

#[cfg(any(windows, test))]
pub struct KeyboardState {
    pressed: [bool; 256],
    consumed: [bool; 256],
}

#[cfg(any(windows, test))]
impl KeyboardState {
    pub fn new() -> Self {
        Self {
            pressed: [false; 256],
            consumed: [false; 256],
        }
    }

    /// Called only by the hook thread. No window calls, locks or disk access.
    pub fn process(
        &mut self,
        vk: u32,
        down: bool,
        binding: u32,
        recording: bool,
        opened: bool,
    ) -> Decision {
        let index = vk as usize;

        if index >= self.pressed.len() {
            return Decision::default();
        }

        let repeated = self.pressed[index];
        self.pressed[index] = down;

        if !down {
            return Decision {
                action: None,
                consume: std::mem::take(&mut self.consumed[index]),
            };
        }

        // Closing is idempotent. Do not let a missed key-up in another app disable Escape.
        if vk == 0x1b && opened && !recording {
            self.consumed[index] = true;

            return Decision {
                action: Some(Action::Close),
                consume: true,
            };
        }

        if repeated {
            return Decision {
                action: None,
                consume: self.consumed[index],
            };
        }

        if recording {
            return Decision::default();
        }

        let any = |keys: &[usize]| keys.iter().any(|&key| self.pressed[key]);
        let modifiers = (u32::from(any(&[0x11, 0xa2, 0xa3])) << 9)
            | (u32::from(any(&[0x12, 0xa4, 0xa5])) << 10)
            | (u32::from(any(&[0x10, 0xa0, 0xa1])) << 11)
            | (u32::from(any(&[0x5b, 0x5c])) << 12);

        let action = if vk | modifiers == binding {
            Some(Action::Toggle)
        } else if vk | modifiers == 0x24f {
            Some(Action::Change)
        } else {
            None
        };

        self.consumed[index] = action.is_some();

        Decision {
            action,
            consume: action.is_some(),
        }
    }

    #[cfg(windows)]
    pub fn allow_passthrough(&mut self, vk: u32) {
        self.consumed[vk as usize] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_still_closes_after_a_missed_release() {
        let mut state = KeyboardState::new();
        // A game or another hook may swallow key-up before focus switches.
        assert_eq!(state.process(0x1b, true, 0x24b, false, false).action, None);
        assert_eq!(
            state.process(0x1b, true, 0x24b, false, true).action,
            Some(Action::Close)
        );
        assert_eq!(state.process(0x1b, true, 0x24b, true, true).action, None);
        assert!(state.process(0x1b, false, 0x24b, false, false).consume);
    }

    #[test]
    fn physical_binding_survives_non_us_layouts_and_distinguishes_numpad() {
        // On AZERTY the physical KeyQ position reports virtual A.
        assert_eq!(physical_key(0x41, 0x10, false), 0x51);
        assert_eq!(physical_key(0x51, 0x1e, false), 0x41);
        assert_eq!(physical_key(0x23, 0x4f, false), 0xff);
        assert_eq!(physical_key(0x23, 0x4f, true), 0x23);
        assert_eq!(physical_key(0x0d, 0x1c, true), 0xff);
    }

    fn single(code: &str) -> Shortcut {
        Shortcut {
            code: code.into(),
            ctrl: false,
            ..Shortcut::default()
        }
    }

    #[test]
    fn validates_physical_keys_and_reserves_close_and_cycle() {
        assert_eq!(Shortcut::default().validate().unwrap(), 0x24b);
        assert_eq!(single("F8").validate().unwrap(), 0x77);
        assert_eq!(single("Digit0").validate().unwrap(), 0x30);
        assert_eq!(single("Backquote").validate().unwrap(), 0xc0);
        for code in [
            "Escape",
            "ControlLeft",
            "Key",
            "F0",
            "F25",
            "",
            "Unidentified",
        ] {
            assert!(single(code).validate().is_err(), "{code}");
        }
        assert!(
            Shortcut {
                code: "KeyO".into(),
                ..Shortcut::default()
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn toggles_once_per_press_and_consumes_the_whole_trigger() {
        let mut state = KeyboardState::new();
        state.process(0xa2, true, 0x24b, false, false);
        assert_eq!(
            state.process(0x4b, true, 0x24b, false, false),
            Decision {
                action: Some(Action::Toggle),
                consume: true
            }
        );
        assert_eq!(
            state.process(0x4b, true, 0x24b, false, true),
            Decision {
                action: None,
                consume: true
            }
        );
        state.process(0xa2, false, 0x24b, false, true);
        assert_eq!(
            state.process(0x4b, false, 0x77, false, true),
            Decision {
                action: None,
                consume: true
            }
        );
        assert_eq!(
            state.process(0x4b, true, 0x24b, false, true),
            Decision::default()
        );
    }

    #[test]
    fn ignores_extra_modifiers_and_rebinding_a_held_key() {
        let mut state = KeyboardState::new();
        state.process(0xa2, true, 0x24b, false, false);
        state.process(0xa0, true, 0x24b, false, false);
        assert_eq!(
            state.process(0x4b, true, 0x24b, false, false),
            Decision::default()
        );
        state.process(0xa0, false, 0x24b, false, false);
        assert_eq!(
            state.process(0x4b, true, 0x24b, false, false),
            Decision::default()
        );
        state.process(0x4b, false, 0x24b, false, false);
        assert_eq!(
            state.process(0x4b, true, 0x24b, false, false).action,
            Some(Action::Toggle)
        );
    }

    #[test]
    fn closes_only_when_open_and_suspends_all_actions_during_recording() {
        let mut state = KeyboardState::new();
        assert_eq!(
            state.process(0x1b, true, 0x77, false, false),
            Decision::default()
        );
        state.process(0x1b, false, 0x77, false, false);
        assert_eq!(
            state.process(0x1b, true, 0x77, false, true).action,
            Some(Action::Close)
        );
        state.process(0x1b, false, 0x77, false, false);
        assert_eq!(
            state.process(0x77, true, 0x77, true, false),
            Decision::default()
        );
        assert_eq!(
            state.process(0x77, true, 0x77, false, false),
            Decision::default()
        );
        state.process(0x77, false, 0x77, false, false);
        assert_eq!(
            state.process(0x77, true, 0x77, false, false).action,
            Some(Action::Toggle)
        );
    }

    #[test]
    fn preserves_cycle_and_handles_both_control_keys() {
        let mut state = KeyboardState::new();
        state.process(0xa2, true, 0x77, false, false);
        state.process(0xa3, true, 0x77, false, false);
        state.process(0xa2, false, 0x77, false, false);
        assert_eq!(
            state.process(0x4f, true, 0x77, false, false).action,
            Some(Action::Change)
        );
    }

    #[test]
    fn saves_and_loads_shortcut_and_rejects_corrupt_settings() {
        let directory =
            std::env::temp_dir().join(format!("tutorlol-shortcut-test-{}", std::process::id()));
        let path = directory.join("shortcut.json");
        assert_eq!(Shortcut::load(&path).unwrap(), Shortcut::default());
        let binding = single("F8");
        binding.save(&path).unwrap();
        assert_eq!(Shortcut::load(&path).unwrap(), binding);
        assert!(single("Escape").save(&path).is_err());
        assert_eq!(Shortcut::load(&path).unwrap(), binding);
        std::fs::write(&path, "broken").unwrap();
        assert!(Shortcut::load(&path).is_err());
        std::fs::remove_dir_all(directory).unwrap();
    }
}
