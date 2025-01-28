use crate::app::BuilderGui as App;
use eframe::egui::{self, Ui, Vec2};
use rfd::FileDialog;

pub fn build(ui: &mut Ui, app: &mut App) {
    let button_width = 50.0;
    let spacing = ui.spacing().item_spacing.x;
    ui.vertical(|ui| {
        #[allow(deprecated)]
        ui.set_enabled(!app.processing);
        // Game Dir
        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), ui.spacing().interact_size.y),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let name_label = ui.add_sized(
                    Vec2::new(66.0, ui.spacing().interact_size.y),
                    egui::Label::new("Game Dir:"),
                );
                ui.add_sized(
                    Vec2::new(
                        ui.available_width() - button_width - spacing,
                        ui.spacing().interact_size.y,
                    ),
                    egui::TextEdit::singleline(&mut app.config.game_dir),
                )
                .labelled_by(name_label.id);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        app.config.game_dir = path.display().to_string();
                    }
                }
            },
        );

        // Output Dir
        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), ui.spacing().interact_size.y),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let name_label = ui.label("Output Dir: ");
                ui.add_sized(
                    Vec2::new(
                        ui.available_width() - button_width - spacing,
                        ui.spacing().interact_size.y,
                    ),
                    egui::TextEdit::singleline(&mut app.config.output_dir),
                )
                .labelled_by(name_label.id);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        app.config.output_dir = path.display().to_string();
                    }
                }
            },
        );
    });
}
