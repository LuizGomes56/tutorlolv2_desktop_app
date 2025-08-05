use brotli::BrotliDecompress;
use std::{
    io::Cursor,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

pub trait ComptimeCache {
    fn as_str(&self) -> &str;
}

static CACHE_PTR: AtomicPtr<u8> = AtomicPtr::new(null_mut());
static CACHE_LEN: AtomicPtr<usize> = AtomicPtr::new(null_mut());

impl ComptimeCache for (usize, usize) {
    fn as_str(&self) -> &str {
        let cache_ptr = CACHE_PTR.load(Ordering::Relaxed);
        let len_ptr = CACHE_LEN.load(Ordering::Relaxed);

        if cache_ptr.is_null() || len_ptr.is_null() {
            return "";
        }

        unsafe {
            let cache_len = *len_ptr;
            let cache_slice = std::slice::from_raw_parts(cache_ptr, cache_len);
            let cache_str = std::str::from_utf8_unchecked(cache_slice);
            if self.1 <= cache_len && self.0 <= self.1 {
                cache_str.get_unchecked(self.0..self.1)
            } else {
                ""
            }
        }
    }
}

pub fn init_cache() {
    if CACHE_PTR.load(Ordering::Relaxed).is_null() {
        let mut output = Vec::with_capacity(generated_code::UNCOMPRESSED_MEGA_BLOCK_SIZE);
        let _ = BrotliDecompress(&mut Cursor::new(&generated_code::MEGA_BLOCK), &mut output);
        let unchecked_string = unsafe { String::from_utf8_unchecked(output) };
        let leaked_str: &'static str = Box::leak(unchecked_string.into_boxed_str());
        let str_ptr = leaked_str.as_ptr() as *mut u8;
        let len_ptr = Box::leak(Box::new(leaked_str.len())) as *mut usize;
        CACHE_PTR.store(str_ptr, Ordering::Release);
        CACHE_LEN.store(len_ptr, Ordering::Release);
    }
}
