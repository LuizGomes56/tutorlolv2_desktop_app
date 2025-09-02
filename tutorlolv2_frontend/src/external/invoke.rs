use crate::models::realtime::Realtime;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen(module = "/public/invoke.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invoke_checkup)]
    pub fn invoke_checkup() -> bool;

    #[wasm_bindgen(js_name = invoke_get_live_game, catch)]
    pub async fn invoke_get_live_game() -> Result<(), JsValue>;
}

static mut DATA_PTR: *mut Realtime = core::ptr::null_mut();

#[wasm_bindgen]
pub fn alloc_live_game_buffer(len: usize) -> u32 {
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr as u32
}

#[wasm_bindgen]
pub fn parse_live_game(ptr: u32, len: usize) {
    let buf = unsafe { Vec::from_raw_parts(ptr as *mut u8, len, len) };
    match bincode::decode_from_slice(&buf, bincode::config::standard()) {
        Ok((realtime, _)) => unsafe {
            if !DATA_PTR.is_null() {
                drop(Box::from_raw(DATA_PTR));
                DATA_PTR = core::ptr::null_mut();
            }
            DATA_PTR = Box::into_raw(Box::new(realtime));
        },
        Err(e) => web_sys::console::log_1(&format!("{:#?}", e).into()),
    };
}

pub fn take_live_game() -> Option<Realtime> {
    unsafe {
        if DATA_PTR.is_null() {
            let _ = invoke_get_live_game();
            None
        } else {
            let b = Box::from_raw(DATA_PTR);
            DATA_PTR = core::ptr::null_mut();
            Some(*b)
        }
    }
}
