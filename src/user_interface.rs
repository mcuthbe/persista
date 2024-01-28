use std::{io::Empty, mem};

use crate::{
    clipboard::{get_clip, set_clip},
    data_access::{delete_clip, open_database, retrieve_clip, save_clip, search_clips},
    errors::PersistaError,
    structs::Clip,
};
use eframe::egui::Context;
use eframe::Frame;
use egui::{ImageSource, Rect, Response, Ui, Window};
use epi::egui::text;
use winapi::shared::windef::HWND__;

pub struct PersistaApp {
    pub search_query: String,
    pub message: String,
    pub new_clip_name: String,
    pub clips: Vec<Clip>,
    pub should_focus: bool,
    pub should_refersh: bool,
    pub foreground_window: *mut HWND__,
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
            ui.set_width(100.0);
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            ui.horizontal(|ui| {
                ui.label("Search:");
                let text_edit_response = ui.text_edit_singleline(&mut self.search_query);

                ui.add_space(ui.available_width() - 250.0);

                ui.horizontal(|ui| {
                    ui.set_max_width(150.0);
                    ui.text_edit_singleline(&mut self.new_clip_name);

                    if ui.button("Save current clip").clicked() {
                        match get_clip() {
                            Ok(clip_item) => {
                                let persy = open_database("target/data.persy").unwrap();

                                let clip = Clip {
                                    name: self.new_clip_name.clone(),
                                    value: clip_item,
                                };

                                match save_clip(&persy, &clip) {
                                    Ok(_) => {
                                        self.message = "Successfully saved clip".to_string();
                                        self.should_refersh = true;
                                    }
                                    Err(e) => {
                                        self.message =
                                            "Failed to save clip".to_string() + &e.to_string()
                                    }
                                }
                            }
                            Err(e) => {
                                self.message = "Failed to save clip".to_string() + &e.to_string();
                            }
                        };
                    };
                });

                if text_edit_response.changed() {
                    self.should_refersh = true
                }
                if self.should_focus {
                    self.should_focus = false;
                    ui.memory_mut(|memory| {
                        memory.request_focus(text_edit_response.id);
                    })
                }
            });

            if !self.message.is_empty() {
                ui.label(&self.message);
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for clip in &self.clips {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if ui.link(&clip.name).clicked() {
                                match set_clip(clip.value.as_str()) {
                                    Ok(_) => {}
                                    Err(e) => {}
                                }
                                if !self.foreground_window.is_null() {
                                    unsafe {
                                        winapi::um::winuser::SetForegroundWindow(
                                            self.foreground_window,
                                        );
                                    }
                                } else {
                                    self.message =
                                        "No previous window found to paste to".to_string();
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
                                    self.message =
                                        "Failed to delete clip".to_owned() + &e.to_string()
                                }
                            }
                        }
                    });
                    ui.separator();
                }
            });

            if self.should_refersh {
                self.refresh_clips();
                self.should_refersh = false
            }
        });
    }
}
