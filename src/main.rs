mod clipboard;
mod data_access;
mod enums;
mod errors;
mod structs;
mod user_interface;
mod util;

use clipboard::clip_get;
use data_access::{open_database, retrieve_clip, save_clip};
use eframe::egui;
use enums::ClipboardItem;
use epi::App;
use structs::Clip;
use user_interface::{show_popup, PersistaApp};

use crate::clipboard::clip_set;

fn main() {
    let options = eframe::NativeOptions::default();

    let app = PersistaApp {
        search_query: "".to_string(),
        clips: vec![],
    };

    eframe::run_native("Persista", options, Box::new(|cc| Box::new(app)));
}
