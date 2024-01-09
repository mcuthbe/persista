mod clipboard;
mod data_access;
mod enums;
mod transformations;

use data_access::{get_clip, save_clip};
use enums::ClipboardItem;

use crate::clipboard::clip_set;

fn main() {
    clip_set("Test");
    let testValue = "Test".to_string();
    let _ = save_clip("Test", ClipboardItem::Text(testValue));
    let values = get_clip(&"Test".to_string()).unwrap();
    for val in values {
        println!("{}", val);
    }
}
