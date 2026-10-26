//! Windows keyboard hook: all input transitions run on its dedicated message thread.
#![cfg(target_os = "windows")]

use crate::{
    overlay,
    shortcut::{Action, KeyboardState, physical_key},
};
use std::{
    cell::RefCell,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicU32, AtomicU64, Ordering},
        mpsc::{self, SyncSender},
    },
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager};
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, MAPVK_VK_TO_VSC_EX, MapVirtualKeyW},
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, HC_ACTION, KBDLLHOOKSTRUCT,
            LLKHF_EXTENDED, MSG, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx,
            WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

static HOTKEY_TX: OnceLock<SyncSender<Action>> = OnceLock::new();
static BINDING: AtomicU32 = AtomicU32::new(0x24b);
static RECORDING_UNTIL: AtomicU64 = AtomicU64::new(0);
static RECORDING_OWNER: Mutex<Option<String>> = Mutex::new(None);
static CLOCK: OnceLock<Instant> = OnceLock::new();

thread_local! { static KEYBOARD: RefCell<KeyboardState> = RefCell::new(KeyboardState::new()); }

pub fn set_binding(binding: u32) {
    BINDING.store(binding, Ordering::Release);
}

pub fn set_recording(owner: &str, recording: bool) {
    if let Ok(mut current) = RECORDING_OWNER.lock() {
        if recording {
            *current = Some(owner.to_owned());
            RECORDING_UNTIL.store(
                CLOCK.get_or_init(Instant::now).elapsed().as_millis() as u64 + 30_000,
                Ordering::Release,
            );
        } else if current.as_deref() == Some(owner) {
            *current = None;
            RECORDING_UNTIL.store(0, Ordering::Release);
        }
    }
}

pub fn install_low_level_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    CLOCK.get_or_init(Instant::now);

    let (tx, rx) = mpsc::sync_channel::<Action>(32);

    HOTKEY_TX
        .set(tx)
        .map_err(|_| "Keyboard hook is already installed")?;

    let app_for_actions = app.clone();

    thread::Builder::new()
        .name("overlay-actions".into())
        .spawn(move || {
            while let Ok(action) = rx.recv() {
                let app = app_for_actions.clone();

                if let Err(error) = app_for_actions.run_on_main_thread(move || {
                    let result = match action {
                        Action::Toggle => overlay::toggle(&app).map(|_| ()),
                        Action::Close => overlay::set_focused(&app, false),
                        Action::Change => app
                            .get_webview_window("overlay")
                            .ok_or_else(|| "Overlay window not found".to_string())
                            .and_then(|window| {
                                window.emit("change", ()).map_err(|e| e.to_string())
                            }),
                    };

                    if let Err(error) = result {
                        log::error!("Overlay shortcut failed: {error}");
                    }
                }) {
                    log::error!("Cannot schedule overlay shortcut: {error}");
                }
            }
        })?;

    let (ready_tx, ready_rx) = mpsc::sync_channel(1);

    thread::Builder::new()
        .name("overlay-keyboard".into())
        .spawn(move || unsafe {
            // Seed held keys before installing the hook. Never query asynchronous key state in the callback.
            KEYBOARD.with(|state| {
                let mut state = state.borrow_mut();

                for vk in 0..256 {
                    if !matches!(vk, 0x10..=0x12)
                        && GetAsyncKeyState(vk as i32) as u16 & 0x8000 != 0
                    {
                        let scan = MapVirtualKeyW(vk, MAPVK_VK_TO_VSC_EX);
                        let key = physical_key(vk, scan & 0xff, scan & 0xff00 != 0);

                        state.process(key, true, 0, true, false);
                    }
                }
            });

            let hook = GetModuleHandleW(None).and_then(|module| {
                SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), Some(module.into()), 0)
            });

            let hook = match hook {
                Ok(hook) => {
                    let _ = ready_tx.send(Ok(()));
                    hook
                }
                Err(error) => {
                    let _ = ready_tx.send(Err(error.to_string()));
                    return;
                }
            };

            let mut msg = MSG::default();

            loop {
                let status = GetMessageW(&mut msg, None, 0, 0).0;

                if status <= 0 {
                    if status < 0 {
                        log::error!("Keyboard hook message loop failed");
                    }

                    break;
                }

                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = UnhookWindowsHookEx(hook);
        })?;

    ready_rx
        .recv_timeout(Duration::from_secs(5))?
        .map_err(|error| {
            std::io::Error::other(format!("Cannot install overlay keyboard hook: {error}"))
        })?;

    Ok(())
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if code == HC_ACTION as i32 {
            let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
            let msg = wparam.0 as u32;
            let down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            let up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

            // Remappers and accessibility keyboards can send injected events. This hook
            // never injects input itself, so accepting them cannot create a feedback loop.
            if (down || up) && kb.vkCode < 256 {
                let key = physical_key(kb.vkCode, kb.scanCode, kb.flags.contains(LLKHF_EXTENDED));

                let consume = KEYBOARD.with(|state| {
                    let mut state = state.borrow_mut();
                    let recording = CLOCK
                        .get()
                        .map(|start| {
                            (start.elapsed().as_millis() as u64)
                                < RECORDING_UNTIL.load(Ordering::Acquire)
                        })
                        .unwrap_or(false);

                    let decision = state.process(
                        key,
                        down,
                        BINDING.load(Ordering::Acquire),
                        recording,
                        overlay::is_focused(),
                    );

                    if let Some(action) = decision.action
                        && !HOTKEY_TX
                            .get()
                            .is_some_and(|tx| tx.try_send(action).is_ok())
                    {
                        state.allow_passthrough(key);

                        return false;
                    }

                    decision.consume
                });

                if consume {
                    return LRESULT(1);
                }
            }
        }

        CallNextHookEx(None, code, wparam, lparam)
    }
}
