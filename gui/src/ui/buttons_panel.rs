use crate::app::BuilderGui as App;
use eframe::egui::{self};
use rfd::FileDialog;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        if !app.processing {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                if ui.button("Add").clicked() {
                    if let Some(paths) = FileDialog::new()
                        .add_filter("Source Maps", &["vmf", "bsp"])
                        .pick_files()
                    {
                        for path in paths {
                            app.add_map(&path);
                        }
                    }
                }
                if ui.button("Clear").clicked() {
                    app.clear_maps();
                }
            });
        } else {
            ui.label(app.process_status.to_string());
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if app.processing {
                let button = egui::Button::new("Processing...");
                ui.add_enabled(false, button);
                ui.add(egui::widgets::Spinner::new());
            } else if ui.button("Start Process").clicked() {
                app.start_processing();
            }
        });
    });
}
