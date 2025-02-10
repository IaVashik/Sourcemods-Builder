use crate::app::BuilderGui as App;
use crate::enums::MapStatus;
use eframe::egui::{self, Layout, ScrollArea};

use super::UiExt;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(ui.available_height() - 20.0);
        ui.set_width(ui.available_width());

        if app.maps.is_empty() {
            ui.add_space(40.0);
            ui.label_size_centered(
                "Drag-and-drop maps onto the window!",
                10.0
            );
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            let mut indices_to_remove = Vec::new();

            for (index, map) in app.maps.iter().enumerate() {
                ui.horizontal(|ui| {
                    match map.status {
                        MapStatus::Processing => {
                            ui.add(egui::widgets::Spinner::new().size(12.0))
                                .on_hover_text("Processing...")
                                .on_hover_cursor(egui::CursorIcon::Progress);
                        }
                        _ => {
                            ui.label_sized(map.status.to_string(), 8.0)
                                .on_hover_text(map.status.get_hover_text())
                                .on_hover_cursor(egui::CursorIcon::Help);
                        }
                    };
                    ui.label(&map.name);

                    if app.processing {
                        return;
                    }

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("🗑").clicked() {
                            indices_to_remove.push(index);
                        };
                    });
                });
            }

            // Removing elements after iteration
            for index in indices_to_remove.iter() {
                app.remove_map(*index);
            }
        });
    });
}
