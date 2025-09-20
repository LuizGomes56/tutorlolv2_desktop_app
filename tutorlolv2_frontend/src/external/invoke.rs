use crate::models::realtime::Realtime;
use std::{
    cell::{Cell, RefCell},
    mem::MaybeUninit,
};
use tutorlolv2_imports::ChampionId;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/public/invoke.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invoke_checkup)]
    pub fn invoke_checkup() -> bool;

    #[wasm_bindgen(js_name = invoke_get_live_game)]
    pub fn invoke_get_live_game();
}

thread_local! {
    pub static RT_ENEMY_IDS: RefCell<Option<Vec<ChampionId>>> = RefCell::new(None);
    static GAME_TIME: Cell<i32> = Cell::new(0);
}

static mut REALTIME_PTR: *mut Realtime = core::ptr::null_mut();

#[wasm_bindgen]
pub fn alloc_bytes(len: usize) -> u32 {
    let mut buf = Box::<[MaybeUninit<u8>]>::new_uninit_slice(len);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr as u32
}

#[wasm_bindgen]
pub fn parse_live_game(ptr: u32, len: usize) {
    unsafe {
        let buf = std::slice::from_raw_parts(ptr as *mut u8, len);
        match bincode::decode_from_slice::<Realtime, _>(buf, bincode::config::standard()) {
            Ok((realtime, _)) => {
                if !REALTIME_PTR.is_null() {
                    drop(Box::from_raw(REALTIME_PTR));
                    REALTIME_PTR = core::ptr::null_mut();
                }
                let game_time = realtime.game_information.game_time;
                if GAME_TIME.get() > game_time || RT_ENEMY_IDS.take().is_none() {
                    GAME_TIME.set(game_time);
                    RT_ENEMY_IDS.replace(Some(realtime.enemies.iter().map(|(x, _)| *x).collect()));
                }
                REALTIME_PTR = Box::into_raw(Box::new(realtime));
            }
            Err(e) => web_sys::console::log_1(&format!("Decode error: {:#?}", e).into()),
        };
    }
}

pub fn take_live_game() -> Option<Realtime> {
    unsafe {
        if REALTIME_PTR.is_null() {
            None
        } else {
            let data = Box::from_raw(REALTIME_PTR);
            REALTIME_PTR = core::ptr::null_mut();
            Some(*data)
        }
    }
}
