use crate::app::BuilderGui as App;
use eframe::egui::{Ui, Vec2};
use super::UiExt;
use rfd::FileDialog;

const BUTTON_WIDTH: f32 = 50.0;

pub fn build(ui: &mut Ui, app: &mut App) {
    let spacing = ui.spacing().item_spacing.x * 2.;

    // Game Dir
    ui.horizontal(|ui| {
        ui.label("Game Dir:");
        ui.allocate_space(Vec2::default()); // Small hack avoid text size differences between Game Dir & Output Dir
        ui.singleline_on_screen(&mut app.config.game_dir, BUTTON_WIDTH + spacing);
        if ui.button("Browse").clicked() {
            if let Some(path) = FileDialog::new().pick_folder() {
                app.config.game_dir = path.display().to_string();
            }
        }
    });

    // Output Dir
    ui.horizontal(|ui| {                
        ui.label("Output Dir:");
        ui.singleline_on_screen(&mut app.config.output_dir, BUTTON_WIDTH + spacing);
        if ui.button("Browse").clicked() {
            if let Some(path) = FileDialog::new().pick_folder() {
                app.config.output_dir = path.display().to_string();
            }
        }
    });
}
