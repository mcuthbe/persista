use clipboard_win::{formats, get_clipboard, set_clipboard};

fn get() -> String {
    let text = get_clipboard(formats::Unicode).expect("Get clipboard contents");
    text
}

fn set(text: &str) {
    set_clipboard(formats::Unicode, text).expect("Set clipboard contents");
}

fn main() {}
