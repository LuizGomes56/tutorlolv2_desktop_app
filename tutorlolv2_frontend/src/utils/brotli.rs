use brotli::BrotliDecompress;
use generated_code::{MEGA_BLOCK, UNCOMPRESSED_MEGA_BLOCK_SIZE};
use wasm_bindgen::prelude::wasm_bindgen;

static mut CACHE_PTR: *const u8 = core::ptr::null();
static mut CACHE_LEN: usize = 0;

#[wasm_bindgen]
pub fn cache_ptr() -> *const u8 {
    unsafe { CACHE_PTR }
}

#[wasm_bindgen]
pub fn cache_len() -> usize {
    unsafe { CACHE_LEN }
}

#[cold]
pub fn init_cache() {
    let mut out = Vec::with_capacity(UNCOMPRESSED_MEGA_BLOCK_SIZE);
    let _ = BrotliDecompress(&mut (&MEGA_BLOCK as &[u8]), &mut out);
    let leaked = Box::leak(out.into_boxed_slice());
    unsafe {
        CACHE_PTR = leaked.as_ptr();
        CACHE_LEN = leaked.len();
    }
}

pub trait ComptimeCache {
    fn as_str(&self) -> &str;
}

impl ComptimeCache for (u32, u32) {
    fn as_str(&self) -> &'static str {
        unsafe {
            core::str::from_utf8_unchecked(
                core::slice::from_raw_parts(CACHE_PTR, CACHE_LEN)
                    .get_unchecked(self.0 as usize..self.1 as usize),
            )
        }
    }
}
