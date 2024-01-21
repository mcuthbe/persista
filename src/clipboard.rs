use std::mem::MaybeUninit;

use clipboard_win::{
    formats, get_clipboard, raw, set_clipboard, types::c_uint, Clipboard, EnumFormats, ErrorCode,
};

use crate::{enums::ClipboardItem, errors::PersistaError};

pub fn get_clip() -> Result<ClipboardItem, PersistaError> {
    if raw::is_format_avail(2) {
        let mut out: Vec<u8> = Vec::new();
        raw::get_bitmap(&mut out)?;
        Ok(ClipboardItem::Image(out))
    } else {
        let text = get_clipboard(formats::Unicode)?;
        Ok(ClipboardItem::Text(text))
    }
}

pub fn set_clip(text: &str) -> Result<(), ErrorCode> {
    set_clipboard(formats::Unicode, text)
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_set_text() {
        let text = "test";
        set_clip(text).unwrap();
        let text2 = get_clip().unwrap();

        if let ClipboardItem::Text(value) = text2 {
            assert_eq!(value, text);
        } else {
            panic!("Expected ClipboardItem::Text, found {:?}", text2);
        }
    }
}
