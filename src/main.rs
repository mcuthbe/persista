mod clipboard;
mod database;

use clipboard::set;
use database::save_clip;

fn main() {
    set("Test");
    save_clip("Test").expect("Save clip");
}
