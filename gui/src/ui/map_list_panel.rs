use crate::app::BuilderGui as App;
use crate::enums::MapStatus;
use eframe::egui::{self, Layout, RichText, ScrollArea};

use super::UiExt;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(ui.available_height() - 20.0);
        ui.set_width(ui.available_width());

        if app.maps.is_empty() {
            ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(40.0);
                ui.label(
                    RichText::new("Drag-and-drop maps onto the window!")
                        .monospace()
                        .small_raised(),
                );
            });
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            let mut indices_to_remove = Vec::new();

            for (index, map) in app.maps.iter().enumerate() {
                ui.horizontal(|ui| {
                    match map.status {
                        MapStatus::Processing => {
                            ui.add(egui::widgets::Spinner::new().size(12.0))
                                .on_hover_text("Processing...");
                        }
                        _ => {
                            ui.label_sized(map.status.to_string(), 8.0)
                                .on_hover_text(map.status.get_hover_text());
                        }
                    };
                    ui.label(&map.name);

                    if !app.processing {
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("🗑").clicked() {
                                indices_to_remove.push(index);
                            };
                        });
                    }
                });
            }

            // Removing elements after iteration
            for index in indices_to_remove.iter() {
                app.remove_map(*index);
            }
        });
    });
}
