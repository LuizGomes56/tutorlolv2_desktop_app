#![allow(static_mut_refs)]
#![cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentThreadId},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_OEM_7},
        WindowsAndMessaging::{
            CallNextHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, PostThreadMessageW,
            SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_USER,
        },
    },
};

static mut HOOK_HANDLE: HHOOK = HHOOK(0 as _);

unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        if code == HC_ACTION as i32 {
            let kb_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            if wparam.0 as u32 == WM_KEYDOWN && kb_struct.vkCode == VK_OEM_7.0 as u32 {
                if GetAsyncKeyState(VK_CONTROL.0 as i32) & 0x8000u16 as i16 != 0 {
                    let _ =
                        PostThreadMessageW(GetCurrentThreadId(), WM_USER + 1, WPARAM(0), LPARAM(0));
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
        let hinstance = GetModuleHandleW(None).unwrap();
        HOOK_HANDLE = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            Some(hinstance.into()),
            0,
        )
        .unwrap();
    }
}

pub unsafe fn uninstall_hook() {
    unsafe {
        if HOOK_HANDLE.is_invalid() {
            let _ = UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = HHOOK(0 as _);
        }
    }
}
