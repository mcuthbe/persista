use clipboard_win::{formats, get_clipboard, set_clipboard, ErrorCode};

use crate::errors::PersistaError;

pub fn clip_get() -> Result<String, PersistaError> {
    let text = get_clipboard(formats::Unicode)?;
    Ok(text)
}

pub fn clip_set(text: &str) -> Result<(), ErrorCode> {
    set_clipboard(formats::Unicode, text)
}
