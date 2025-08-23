use brotli::BrotliDecompress;
use generated_code::{MEGA_BLOCK, UNCOMPRESSED_MEGA_BLOCK_SIZE};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Uint8Array;

pub trait ComptimeCache {
    fn as_str(&self) -> &str;
}

static mut CACHE: *const str = "";

#[wasm_bindgen]
pub fn get_cache_slice() -> Uint8Array {
    unsafe { Uint8Array::view(CACHE.as_ref().unwrap_unchecked().as_bytes()) }
}

impl ComptimeCache for (usize, usize) {
    #[inline]
    fn as_str(&self) -> &'static str {
        unsafe {
            CACHE
                .as_ref()
                .unwrap_unchecked()
                .get_unchecked(self.0..self.1)
        }
    }
}

#[cold]
pub fn init_cache() {
    unsafe {
        let mut output = Vec::with_capacity(UNCOMPRESSED_MEGA_BLOCK_SIZE);
        let _ = BrotliDecompress(&mut (&MEGA_BLOCK as &[u8]), &mut output);
        let unchecked_string = String::from_utf8_unchecked(output);
        CACHE = Box::leak(unchecked_string.into_boxed_str());
    }
}
