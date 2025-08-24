use brotli::BrotliDecompress;
use generated_code::{MEGA_BLOCK, UNCOMPRESSED_MEGA_BLOCK_SIZE};
use wasm_bindgen::prelude::wasm_bindgen;

pub trait ComptimeCache {
    fn as_str(&self) -> &str;
}

static mut CACHE: *const str = "";

macro_rules! cache_slice {
    () => {
        CACHE.as_ref().unwrap_unchecked()
    };
}

#[wasm_bindgen]
pub fn cache_ptr() -> *const u8 {
    unsafe { cache_slice!().as_ptr() }
}

#[wasm_bindgen]
pub fn cache_len() -> usize {
    unsafe { cache_slice!().len() }
}

impl ComptimeCache for (usize, usize) {
    #[inline]
    fn as_str(&self) -> &'static str {
        unsafe { cache_slice!().get_unchecked(self.0..self.1) }
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
