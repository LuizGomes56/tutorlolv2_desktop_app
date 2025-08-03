use brotli::BrotliDecompress;
use rustc_hash::FxHashMap;
use std::{cell::RefCell, io::Cursor};

pub trait FromBrotliBytes {
    fn as_str(&self) -> &str;
}

thread_local! {
    static CACHE: RefCell<FxHashMap<*const u8, &'static str>> =
        RefCell::new(FxHashMap::default());
}

impl FromBrotliBytes for &[u8] {
    fn as_str(&self) -> &str {
        let ptr = self.as_ptr();

        CACHE.with(|cell| {
            let mut cache = cell.borrow_mut();
            if let Some(&s) = cache.get(&ptr) {
                return s;
            }
            let mut output = Vec::with_capacity(self.len() * 4);
            if BrotliDecompress(&mut Cursor::new(*self), &mut output).is_err() {
                return "Failed to decompress data";
            }
            let leaked = unsafe {
                let s = String::from_utf8_unchecked(output);
                Box::leak(s.into_boxed_str())
            };
            cache.insert(ptr, leaked);
            leaked
        })
    }
}

impl<'b> FromBrotliBytes for &&'b [u8] {
    fn as_str(&self) -> &str {
        <&[u8] as FromBrotliBytes>::as_str(*self)
    }
}
