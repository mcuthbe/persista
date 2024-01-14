use clipboard_win::{formats, get_clipboard, set_clipboard, ErrorCode};

use crate::errors::PersistaError;

pub fn clip_get() -> Result<String, PersistaError> {
    let text = get_clipboard(formats::Unicode)?;
    Ok(text)
}

pub fn clip_set(text: &str) -> Result<(), ErrorCode> {
    set_clipboard(formats::Unicode, text)
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_set() {
        let text = "test";
        clip_set(text).unwrap();
        let text2 = clip_get().unwrap();
        assert_eq!(text, text2);
    }
}
