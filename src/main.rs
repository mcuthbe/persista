mod clipboard;
mod data_access;
mod enums;
mod errors;
mod structs;

use data_access::{get_clip, save_clip};
use enums::ClipboardItem;
use structs::Clip;

use crate::clipboard::clip_set;

fn main() {
    clip_set("Test");
    let test_clip = Clip {
        name: "Test".to_string(),
        value: ClipboardItem::Text("Test".to_string()),
    };
    let _ = save_clip(&test_clip);
    let values = get_clip(&"Test".to_string()).unwrap();
    match values {
        Some(value) => println!("Value: {}", value),
        None => println!("No value found"),
    }
}
