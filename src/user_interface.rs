use std::{io::Empty, mem};

use crate::{
    clipboard::clip_set,
    data_access::{delete_clip, open_database, retrieve_clip, search_clips},
    errors::PersistaError,
    structs::Clip,
};
use eframe::egui::Context;
use eframe::Frame;
use egui::{ImageSource, Rect, Response, Ui, Window};
use epi::egui::text;

#[derive(Default)]
pub struct PersistaApp {
    pub search_query: String,
    pub clips: Vec<Clip>,
    pub should_focus: bool,
    pub message: String,
    pub should_refersh: bool,
}

impl PersistaApp {
    fn refresh_clips(&mut self) {
        let persy = open_database("target/data.persy").unwrap();

        self.clips = search_clips(&persy, &mut self.search_query).unwrap()
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
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if ui.link(&clip.name).clicked() {
                            match clip_set(clip.value.as_str()) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                        match &clip.value {
                            crate::enums::ClipboardItem::Image(image) => {
                                let uri = "bytes://my_image.png"; // Replace with your actual URI

                                let image_source = egui::ImageSource::Bytes {
                                    uri: std::borrow::Cow::Borrowed(uri),
                                    bytes: image.clone().into(),
                                };
                                ui.add(
                                    egui::Image::new(image_source).rounding(5.0), // .tint(egui::Color32::LIGHT_BLUE) // Optional tinting
                                )
                            }
                            _ => ui.label(clip.value.as_str()),
                        };
                    });

                    ui.add_space(ui.available_width() - 30.0);

                    if ui.button("x").clicked() {
                        let persy = open_database("target/data.persy").unwrap();

                        match delete_clip(&persy, &clip.name) {
                            Ok(_) => {
                                self.should_refersh = true;
                                self.message = "Successfully deleted clip".to_string();
                            }
                            Err(e) => {
                                self.message = "Failed to delete clip".to_owned() + &e.to_string()
                            }
                        }
                    }
                });
                ui.separator();
            }

            if text_edit_response.changed() {
                self.should_refersh = true
            }

            if self.should_refersh {
                self.refresh_clips();
                self.should_refersh = false
            }

            if self.should_focus {
                self.should_focus = false;
                ui.memory_mut(|memory| {
                    memory.request_focus(text_edit_response.id);
                })
            }

            if !self.message.is_empty() {
                ui.label(&self.message);
            }
        });
    }
}
