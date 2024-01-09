use crate::enums::ClipboardItem;

pub fn clipboard_item_to_u8_vec(item: ClipboardItem) -> Vec<u8> {
    match item {
        ClipboardItem::Text(text) => text.into_bytes(),
        ClipboardItem::Image(image) => image.clone(),
        ClipboardItem::Html(html) => html.into_bytes(),
        ClipboardItem::Rtf(rtf) => rtf.into_bytes(),
        ClipboardItem::Custom(custom) => custom.clone(),
    }
}

pub fn u8_vec_to_clipboard_item(item: Vec<u8>) -> ClipboardItem {
    if let Ok(text) = String::from_utf8(item.clone()) {
        return ClipboardItem::Text(text);
    }
    ClipboardItem::Custom(item)
}
