use crate::app::BuilderGui as App;
use eframe::egui::{self};
use rfd::FileDialog;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        build_left_ui(ui, app);
        build_right_ui(ui, app);
    });
}

fn build_left_ui(ui: &mut egui::Ui, app: &mut App) {
    if app.processing {
        ui.horizontal(|ui| {
            ui.label(app.process_status.to_string())
                .on_hover_cursor(egui::CursorIcon::Wait);
            match app.process_status {
                crate::enums::ProcessingStatus::ScanMap(idx) => {
                    ui.label(&format!("{idx}/{total}", total = app.config.maps.len()));
                }
                _ => {}
            }
        });
        return;
    }

    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        if ui.button("Add").clicked() {
            let dialog = FileDialog::new()
                .add_filter("Source Maps", &["vmf", "bsp"])
                .pick_files();
            if let Some(paths) = dialog {
                let _ = paths.iter().map(|path| app.add_map(path));
            }
        }
        if ui.button("Clear").clicked() {
            app.clear_maps();
        }
    });
}

fn build_right_ui(ui: &mut egui::Ui, app: &mut App) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if app.processing {
            if ui.button("Abort").clicked() {
                app.cancel_compile()
            }
            ui.add(egui::widgets::Spinner::new())
                .on_hover_cursor(egui::CursorIcon::Progress);
        } else if ui.button("Start Process").clicked() {
            app.start_processing();
        }
    });
}
