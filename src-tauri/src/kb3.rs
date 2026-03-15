#![cfg(target_os = "windows")]

use std::sync::{
    OnceLock,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Sender},
};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::GetAsyncKeyState,
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG,
            SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN,
            WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

#[derive(Clone, Copy, Debug)]
enum HotkeyAction {
    Focus,
    Unfocus,
    Change,
}

static HOTKEY_TX: OnceLock<Sender<HotkeyAction>> = OnceLock::new();

static K_DOWN: AtomicBool = AtomicBool::new(false);
static L_DOWN: AtomicBool = AtomicBool::new(false);
static O_DOWN: AtomicBool = AtomicBool::new(false);

const VK_CONTROL_I32: i32 = 0x11;
const VK_K_U32: u32 = b'K' as u32;
const VK_L_U32: u32 = b'L' as u32;
const VK_O_U32: u32 = b'O' as u32;

pub fn install_low_level_shortcuts(app: &AppHandle) -> tauri::Result<()> {
    let (tx, rx) = mpsc::channel::<HotkeyAction>();
    let _ = HOTKEY_TX.set(tx);

    let app_for_actions = app.clone();
    thread::spawn(move || {
        while let Ok(action) = rx.recv() {
            if let Some(overlay) = app_for_actions.get_webview_window("overlay") {
                match action {
                    HotkeyAction::Focus => {
                        println!("Focusing");
                        let _ = overlay.emit("focus", true);
                        let _ = overlay.set_ignore_cursor_events(false);
                    }
                    HotkeyAction::Unfocus => {
                        println!("Unfocusing");
                        let _ = overlay.emit("focus", false);
                        let _ = overlay.set_ignore_cursor_events(true);
                    }
                    HotkeyAction::Change => {
                        println!("Change");
                        let _ = overlay.emit("change", ());
                    }
                }
            }
        }
    });

    thread::spawn(move || unsafe {
        let module = GetModuleHandleW(None).unwrap_or_default();

        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), Some(module.into()), 0)
            .expect("failed to install WH_KEYBOARD_LL");

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = UnhookWindowsHookEx(hook);
    });

    Ok(())
}

fn is_keydown(msg: u32) -> bool {
    msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN
}

fn is_keyup(msg: u32) -> bool {
    msg == WM_KEYUP || msg == WM_SYSKEYUP
}

fn ctrl_is_down() -> bool {
    (unsafe { GetAsyncKeyState(VK_CONTROL_I32) } as u16 & 0x8000) != 0
}

fn fire_once(flag: &AtomicBool, action: HotkeyAction) {
    if !flag.swap(true, Ordering::SeqCst)
        && let Some(tx) = HOTKEY_TX.get()
    {
        let _ = tx.send(action);
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if code < 0 {
            return CallNextHookEx(Some(HHOOK::default()), code, wparam, lparam);
        }

        if code == HC_ACTION as i32 {
            let msg = wparam.0 as u32;
            let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
            let vk = kb.vkCode;

            if is_keydown(msg) && ctrl_is_down() {
                match vk {
                    VK_K_U32 => fire_once(&K_DOWN, HotkeyAction::Focus),
                    VK_L_U32 => fire_once(&L_DOWN, HotkeyAction::Unfocus),
                    VK_O_U32 => fire_once(&O_DOWN, HotkeyAction::Change),
                    _ => {}
                }
            }

            if is_keyup(msg) {
                match vk {
                    VK_K_U32 => K_DOWN.store(false, Ordering::SeqCst),
                    VK_L_U32 => L_DOWN.store(false, Ordering::SeqCst),
                    VK_O_U32 => O_DOWN.store(false, Ordering::SeqCst),
                    _ => {}
                }
            }
        }

        CallNextHookEx(Some(HHOOK::default()), code, wparam, lparam)
    }
}
