use crate::{
    clipboard::clip_set,
    data_access::{open_database, retrieve_clip, search_clips},
    errors::PersistaError,
    structs::Clip,
};
use eframe::egui::Context;
use eframe::Frame;
use egui::Window;

#[derive(Default)]
pub struct PersistaApp {
    pub search_query: String,
    pub clips: Vec<Clip>,
}

impl PersistaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for PersistaApp {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        let _ = show_popup(context, &mut self.search_query);
    }
}

pub fn show_popup(context: &Context, search_query: &mut String) -> Result<(), PersistaError> {
    Window::new("Search clips")
        .open(&mut true)
        .show(context, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(search_query)
            });

            ui.separator();

            let persy = open_database("target/data.persy").unwrap();

            let clips = search_clips(&persy, &search_query).unwrap();

            for clip in clips {
                if ui.button(&clip.name).clicked() {
                    match clip_set(clip.value.as_str()) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        });

    Ok(())
}
