mod clipboard;
mod data_access;
mod enums;
mod errors;
mod structs;
mod user_interface;
mod util;

use eframe::egui;
use egui::ViewportBuilder;

use user_interface::PersistaApp;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.persist_window = false;
    options.viewport = ViewportBuilder::default()
        .with_inner_size(egui::vec2(250.0, 400.0))
        .with_decorations(false);

    let app = PersistaApp {
        search_query: "".to_string(),
        clips: vec![],
    };

    eframe::run_native("Persista", options, Box::new(|cc| Box::new(app)));
}
