mod clipboard;
mod data_access;
mod enums;
mod errors;
mod structs;
mod user_interface;
mod util;

use clipboard::{get_clip, set_clip};
use data_access::{open_database, retrieve_clip, save_clip, search_clips};
use eframe::egui;
use egui::ViewportBuilder;

use enums::ClipboardItem;
use structs::Clip;
use user_interface::PersistaApp;

fn main() {
    get_clip();
    let clip = get_clip().unwrap();
    let test_clip = Clip {
        name: "image".to_owned(),
        value: clip,
    };
    // clip_set("Test").unwrap();

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
        message: "".to_string(),
        should_refersh: true,
        new_clip_name: "".to_string(),
    };

    eframe::run_native("Persista", options, Box::new(|cc| Box::new(app)));
}
