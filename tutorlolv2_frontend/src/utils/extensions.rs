pub trait StringExt {
    fn padding_chars(&self) -> String;
    fn concat_char(&self, c: char) -> String;
    fn to_sized_slice<const N: usize>(&self) -> [u8; N];
    fn first_char(&self) -> char;
}

impl StringExt for str {
    fn first_char(&self) -> char {
        self.chars().next().unwrap_or_default()
    }
    fn concat_char(&self, c: char) -> String {
        let mut s = String::with_capacity(self.len() + 1);
        s.push_str(self);
        s.push(c);
        s
    }
    fn padding_chars(&self) -> String {
        let mut out = [0u8; 3];
        let mut i = 0;
        let mut written = 0;
        for b in self.bytes() {
            if b != b'_' {
                if i >= 1 && written < 3 {
                    out[written] = b;
                    written += 1;
                }
                i += 1;
            }
        }
        String::from_utf8_lossy(&out[..written]).into_owned()
    }
    fn to_sized_slice<const N: usize>(&self) -> [u8; N] {
        let mut out = [0u8; N];
        let bytes = self.as_bytes();
        let len = bytes.len().min(N);
        out[..len].copy_from_slice(&bytes[..len]);
        out
    }
}

pub trait BytesExt {
    fn as_str_unchecked(&self) -> &str;
}

impl<const N: usize> BytesExt for [u8; N] {
    fn as_str_unchecked(&self) -> &str {
        let len = self.iter().position(|&b| b == 0).unwrap_or(N);
        unsafe { std::str::from_utf8_unchecked(&self[..len]) }
    }
}
