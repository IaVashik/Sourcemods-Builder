use crate::app::BuilderGui as App;
use eframe::egui::{Ui, Vec2};
use super::UiExt;
use rfd::FileDialog;

const BUTTON_WIDTH: f32 = 50.0;

pub fn build(ui: &mut Ui, app: &mut App) {
    let spacing = ui.spacing().item_spacing.x;
    ui.vertical(|ui| {
        #[allow(deprecated)]
        ui.set_enabled(!app.processing);
        // Game Dir
        ui.horizontal(|ui| {
            ui.label("Game Dir:");
            ui.allocate_space(Vec2::default()); // Small hack avoid text size differences between Game Dir & Output Dir
            ui.singleline_on_screen(&mut app.game_dir, BUTTON_WIDTH + spacing, 0.0);
            if ui.button("Browse").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    app.game_dir = path.display().to_string();
                }
            }
        });

        // Output Dir
        ui.horizontal(|ui| {                
            ui.label("Output Dir:");
            ui.singleline_on_screen(&mut app.output_dir, BUTTON_WIDTH + spacing, 0.0);
            if ui.button("Browse").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    app.output_dir = path.display().to_string();
                }
            }
        });
    });
}
