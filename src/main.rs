mod clipboard;
mod data_access;
mod enums;
mod errors;
mod structs;
mod user_interface;
mod util;

use clipboard::{clip_get, clip_set};
use data_access::{open_database, retrieve_clip, save_clip};
use eframe::egui;
use egui::ViewportBuilder;

use enums::ClipboardItem;
use structs::Clip;
use user_interface::PersistaApp;

fn main() {
    clip_set("Test").unwrap();
    clip_get().unwrap();
    let test_clip = Clip {
        name: "Test".to_string(),
        value: ClipboardItem::Text("Test".to_string()),
    };

    // Inlined function to call and dispose of persy
    {
        let persy = open_database("target/data.persy").unwrap();
        // let _ = save_clip(&persy, &test_clip);
        let values = retrieve_clip(&persy, &"Test".to_string()).unwrap();
        match values {
            Some(clip) => println!("Value: {}", clip.as_str()),
            None => println!("No value found"),
        }
    }

    let mut options = eframe::NativeOptions::default();
    options.persist_window = false;
    options.viewport = ViewportBuilder::default()
    // .with_decorations(false)
    ;

    let app = PersistaApp {
        search_query: "".to_string(),
        clips: vec![],
        should_focus: true,
    };

    eframe::run_native("Persista", options, Box::new(|cc| Box::new(app)));
}
