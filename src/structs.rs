use serde::{Deserialize, Serialize};

use crate::enums::ClipboardItem;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Clip {
    pub name: String,
    pub value: ClipboardItem,
}
