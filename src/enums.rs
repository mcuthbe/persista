use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum ClipboardItem {
    Text(String),
    Image(Vec<u8>),
    // File(Vec<String>), //Probably disallow this because if the user deletes a file, this won't be permanent
    Html(String),
    Rtf(String),
    Custom(Vec<u8>),
}
