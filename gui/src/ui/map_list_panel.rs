use crate::app::BuilderGui as App;
use crate::enums::MapStatus;
use eframe::egui::{self, Layout, ScrollArea};

use super::UiExt;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(ui.available_height().max(22.) - 20.0);
        ui.set_width(ui.available_width());

        if app.config.maps.is_empty() {
            ui.add_space(10.0);
            ui.label_size_centered(
                "Drag-and-drop maps onto the window!",
                10.0
            );
            return;
        }

        let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);
        let index_to_scroll_to = match app.process_status {
            crate::enums::ProcessingStatus::ScanMap(index) => Some(index),
            _ => None,
        };        

        scroll_area.show(ui, |ui| {
            let mut indices_to_remove = Vec::new();

            for (index, map) in app.config.maps.iter().enumerate() {
                let item_response = ui.horizontal(|ui| {
                    match &map.status {
                        MapStatus::Processing => {
                            ui.add(egui::widgets::Spinner::new().size(12.0))
                                .on_hover_text("Processing...")
                                .on_hover_cursor(egui::CursorIcon::Progress);
                        }
                        MapStatus::Error(info) => {
                            ui.label(egui::RichText::new("❌").size(8.).color(egui::Color32::LIGHT_RED))
                                .on_hover_text(info)
                                .on_hover_cursor(egui::CursorIcon::ContextMenu);
                        }
                        _ => {
                            ui.label_sized(map.status.to_string(), 8.0)
                                .on_hover_text(map.status.get_hover_text())
                                .on_hover_cursor(egui::CursorIcon::Help);
                        }
                    };
                    ui.label(&map.name).on_hover_text(map.path.as_os_str().to_str().unwrap()); 

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_enabled(!app.processing, egui::Button::new("🗑").small()).clicked() {
                            indices_to_remove.push(index);
                        };
                    });
                });

                // If the current index matches the target, scroll to its rectangle in the UI.
                if Some(index) == index_to_scroll_to {
                    ui.scroll_to_rect(item_response.response.rect, Some(egui::Align::Center));
                }
            }


            // Removing elements after iteration
            for index in indices_to_remove.iter().rev() {
                app.remove_map(*index);
            }
        });
    });
}
