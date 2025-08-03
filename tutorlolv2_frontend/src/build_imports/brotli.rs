use brotli::BrotliDecompress;
use std::io::Cursor;
use yew::Html;

pub trait FromBrotliBytes {
    fn to_string(&self) -> String;
    fn as_html(&self) -> Html {
        Html::from(self.to_string())
    }
}

impl FromBrotliBytes for &[u8] {
    fn to_string(&self) -> String {
        let mut output = Vec::with_capacity(self.len() * 4);
        if BrotliDecompress(&mut Cursor::new(*self), &mut output).is_err() {
            return String::new();
        }
        String::from_utf8(output).unwrap_or("Invalid UTF-8 byte sequence".to_string())
    }

    fn as_html(&self) -> Html {
        Html::from(self.to_string())
    }
}

impl<'b> FromBrotliBytes for &&'b [u8] {
    fn to_string(&self) -> String {
        <&[u8] as FromBrotliBytes>::to_string(*self)
    }

    fn as_html(&self) -> Html {
        <&[u8] as FromBrotliBytes>::as_html(*self)
    }
}
