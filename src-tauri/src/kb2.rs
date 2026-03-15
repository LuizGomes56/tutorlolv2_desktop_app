#![allow(static_mut_refs)]
#![cfg(target_os = "windows")]

use std::sync::{
    OnceLock,
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
};

use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{VK_CONTROL, VK_LCONTROL, VK_RCONTROL},
        WindowsAndMessaging::{
            CallNextHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, SetWindowsHookExW,
            UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

#[derive(Clone, Copy, Debug)]
pub enum KbEvent {
    ToggleOverlay,
    Change,
}

static mut HOOK_HANDLE: HHOOK = HHOOK(0 as _);
static HOTKEY_TX: OnceLock<Sender<KbEvent>> = OnceLock::new();

static CTRL_DOWN: AtomicBool = AtomicBool::new(false);
static TOGGLE_IS_DOWN: AtomicBool = AtomicBool::new(false);
static CHANGE_IS_DOWN: AtomicBool = AtomicBool::new(false);

const VK_K_CODE: u32 = b'K' as u32;
const VK_L_CODE: u32 = b'L' as u32;

pub fn set_hotkey_sender(tx: Sender<KbEvent>) {
    let _ = HOTKEY_TX.set(tx);
}

unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        if code == HC_ACTION as i32 {
            let kb = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let msg = wparam.0 as u32;

            let is_key_down = matches!(msg, WM_KEYDOWN | WM_SYSKEYDOWN);
            let is_key_up = matches!(msg, WM_KEYUP | WM_SYSKEYUP);

            let vk = kb.vkCode;

            if is_key_down {
                if vk == VK_CONTROL.0 as u32
                    || vk == VK_LCONTROL.0 as u32
                    || vk == VK_RCONTROL.0 as u32
                {
                    CTRL_DOWN.store(true, Ordering::SeqCst);
                }

                let ctrl_down = CTRL_DOWN.load(Ordering::SeqCst);
                let is_k = vk == VK_K_CODE;
                let is_l = vk == VK_L_CODE;

                if is_k && ctrl_down {
                    if !TOGGLE_IS_DOWN.swap(true, Ordering::SeqCst)
                        && let Some(tx) = HOTKEY_TX.get()
                    {
                        let _ = tx.send(KbEvent::ToggleOverlay);
                    }
                } else if is_l
                    && ctrl_down
                    && !CHANGE_IS_DOWN.swap(true, Ordering::SeqCst)
                    && let Some(tx) = HOTKEY_TX.get()
                {
                    let _ = tx.send(KbEvent::Change);
                }
            }

            if is_key_up {
                if vk == VK_CONTROL.0 as u32
                    || vk == VK_LCONTROL.0 as u32
                    || vk == VK_RCONTROL.0 as u32
                {
                    CTRL_DOWN.store(false, Ordering::SeqCst);
                }

                if vk == VK_K_CODE {
                    TOGGLE_IS_DOWN.store(false, Ordering::SeqCst);
                }

                if vk == VK_L_CODE {
                    CHANGE_IS_DOWN.store(false, Ordering::SeqCst);
                }
            }
        }

        CallNextHookEx(Some(HOOK_HANDLE), code, wparam, lparam)
    }
}

pub unsafe fn install_hook() {
    unsafe {
        if !HOOK_HANDLE.is_invalid() {
            return;
        }

        let hinstance = GetModuleHandleW(None).unwrap_unchecked();

        HOOK_HANDLE = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            Some(hinstance.into()),
            0,
        )
        .unwrap_unchecked();
    }
}

pub unsafe fn uninstall_hook() {
    unsafe {
        if !HOOK_HANDLE.is_invalid() {
            let _ = UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = HHOOK(0 as _);
        }
    }
}
