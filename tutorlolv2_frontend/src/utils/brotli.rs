#![allow(static_mut_refs)]
use brotli::BrotliDecompress;
use generated_code::{MEGA_BLOCK, UNCOMPRESSED_MEGA_BLOCK_SIZE};
use std::io::Write;
use wasm_bindgen::prelude::wasm_bindgen;

static mut CACHE: [u8; UNCOMPRESSED_MEGA_BLOCK_SIZE] = [0; UNCOMPRESSED_MEGA_BLOCK_SIZE];

pub trait ComptimeCache {
    fn as_str(&self) -> &str;
}

pub struct FixedBuffer<const N: usize> {
    buffer: &'static mut [u8; N],
    position: usize,
}

impl<const N: usize> Write for FixedBuffer<N> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            let destination_ptr = self.buffer.as_mut_ptr().add(self.position);
            let source_ptr = buf.as_ptr();
            core::ptr::copy_nonoverlapping(source_ptr, destination_ptr, buf.len());
            self.position += buf.len();
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl ComptimeCache for (u32, u32) {
    #[inline]
    fn as_str(&self) -> &'static str {
        unsafe {
            core::str::from_utf8_unchecked(CACHE.get_unchecked(self.0 as usize..self.1 as usize))
        }
    }
}

#[wasm_bindgen]
pub fn cache_ptr() -> *const u8 {
    unsafe { CACHE.as_ptr() }
}

#[wasm_bindgen]
pub fn cache_len() -> usize {
    UNCOMPRESSED_MEGA_BLOCK_SIZE
}

#[cold]
pub fn init_cache() {
    web_sys::console::time();
    unsafe {
        let _ = BrotliDecompress(
            &mut (&MEGA_BLOCK as &[u8]),
            &mut (&mut FixedBuffer {
                buffer: &mut CACHE,
                position: 0,
            }),
        );
    }
    web_sys::console::time_end();
}
