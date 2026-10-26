use crate::shortcut::Shortcut;
use serde::Serialize;
use std::{path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Emitter, State, WebviewWindow};

#[derive(Clone, Serialize)]
pub struct ShortcutSettings {
    shortcut: Shortcut,
    warning: Option<String>,
}

pub struct ShortcutState {
    path: PathBuf,
    settings: Mutex<ShortcutSettings>,
}

impl ShortcutState {
    pub fn load(path: PathBuf) -> Self {
        let (shortcut, warning) = match Shortcut::load(&path) {
            Ok(shortcut) => (shortcut, None),
            Err(error) => {
                log::warn!("{error}; using Ctrl+K");
                (
                    Shortcut::default(),
                    Some(format!(
                        "{error}. Ctrl+K is active; save a shortcut to replace these settings."
                    )),
                )
            }
        };

        #[cfg(windows)]
        crate::wkb::set_binding(shortcut.validate().expect("validated shortcut"));

        Self {
            path,
            settings: Mutex::new(ShortcutSettings { shortcut, warning }),
        }
    }
}

#[tauri::command]
pub fn get_overlay_shortcut(state: State<'_, ShortcutState>) -> Result<ShortcutSettings, String> {
    state
        .settings
        .lock()
        .map(|settings| settings.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_overlay_shortcut(
    app: AppHandle,
    state: State<'_, ShortcutState>,
    shortcut: Shortcut,
) -> Result<ShortcutSettings, String> {
    let binding = shortcut.validate()?;
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;

    shortcut.save(&state.path)?;

    #[cfg(windows)]
    crate::wkb::set_binding(binding);

    #[cfg(not(windows))]
    let _ = binding;

    *settings = ShortcutSettings {
        shortcut,
        warning: None,
    };

    if let Err(error) = app.emit("overlay-shortcut-changed", settings.clone()) {
        log::error!("Cannot notify shortcut change: {error}");
    }

    Ok(settings.clone())
}

#[tauri::command]
pub fn set_shortcut_recording(window: WebviewWindow, recording: bool) -> Result<(), String> {
    if recording && !window.is_focused().map_err(|error| error.to_string())? {
        return Err("Focus this window before recording a shortcut".into());
    }

    #[cfg(windows)]
    crate::wkb::set_recording(window.label(), recording);

    #[cfg(not(windows))]
    let _ = (window, recording);

    Ok(())
}
