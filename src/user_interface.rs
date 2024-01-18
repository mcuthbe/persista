use std::mem;

use crate::{
    clipboard::clip_set,
    data_access::{open_database, retrieve_clip, search_clips},
    errors::PersistaError,
    structs::Clip,
};
use eframe::egui::Context;
use eframe::Frame;
use egui::{Response, Ui, Window};
use epi::egui::text;

#[derive(Default)]
pub struct PersistaApp {
    pub search_query: String,
    pub clips: Vec<Clip>,
    pub should_focus: bool,
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
        egui::CentralPanel::default().show(context, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            ui.label("Search:");
            let text_edit_response = ui.text_edit_singleline(&mut self.search_query);

            ui.separator();

            for clip in &self.clips {
                ui.label(&clip.name);
                if ui.button(&clip.name).clicked() {
                    match clip_set(clip.value.as_str()) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }

            if text_edit_response.changed() {
                let persy = open_database("target/data.persy").unwrap();

                self.clips = search_clips(&persy, &mut self.search_query).unwrap();
            }

            if self.should_focus {
                self.should_focus = false;
                ui.memory_mut(|memory| {
                    memory.request_focus(text_edit_response.id);
                })
            }
        });
    }
}
