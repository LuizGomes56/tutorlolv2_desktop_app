//! The native window and frontend share one interaction state.
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};
use tauri::{AppHandle, Emitter, Manager};

static FOCUSED: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
pub struct OverlayState(Mutex<Inner>);

#[derive(Default)]
struct Inner {
    focused: bool,

    #[cfg(windows)]
    previous_window: usize,
}

pub fn is_focused() -> bool {
    FOCUSED.load(Ordering::Acquire)
}

pub fn set_focused(app: &AppHandle, focused: bool) -> Result<(), String> {
    let state = app.state::<OverlayState>();
    let mut state = state.0.lock().map_err(|e| e.to_string())?;

    apply(app, &mut state, focused)
}

pub fn toggle(app: &AppHandle) -> Result<bool, String> {
    let state = app.state::<OverlayState>();
    let mut state = state.0.lock().map_err(|e| e.to_string())?;
    let focused = !state.focused;

    apply(app, &mut state, focused)?;

    Ok(focused)
}

fn apply(app: &AppHandle, state: &mut Inner, focused: bool) -> Result<(), String> {
    let window = app
        .get_webview_window("overlay")
        .ok_or("Overlay window not found")?;

    if focused {
        #[cfg(windows)]
        if !state.focused {
            state.previous_window =
                unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() }.0
                    as usize;
        }

        window.set_focusable(true).map_err(|e| e.to_string())?;
        window
            .set_ignore_cursor_events(false)
            .map_err(|e| e.to_string())?;
    }
    // Do not activate the window here: this can minimize exclusive fullscreen games.
    // Clicking an enabled overlay control can give the webview keyboard focus.
    else {
        window
            .set_ignore_cursor_events(true)
            .map_err(|e| e.to_string())?;

        window.set_focusable(false).map_err(|e| e.to_string())?;

        #[cfg(windows)]
        unsafe {
            use windows::Win32::{
                Foundation::HWND,
                UI::WindowsAndMessaging::{GetForegroundWindow, IsWindow, SetForegroundWindow},
            };

            let previous = HWND(state.previous_window as *mut _);

            if window
                .hwnd()
                .is_ok_and(|hwnd| hwnd == GetForegroundWindow())
                && IsWindow(Some(previous)).as_bool()
            {
                let _ = SetForegroundWindow(previous);
            }
        }
    }

    state.focused = focused;

    FOCUSED.store(focused, Ordering::Release);

    if let Err(error) = window.emit("focus", focused) {
        log::error!("Cannot notify overlay focus: {error}");
    }

    Ok(())
}
