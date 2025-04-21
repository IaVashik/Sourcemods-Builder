use crate::app::BuilderGui as App;
use eframe::egui;
use egui::{CentralPanel, Context};

mod buttons_panel;
mod footer;
mod map_list_panel;
mod menu_bar;
mod settings_panel;
mod ext;
pub mod themes;

pub use ext::UiExt;

pub fn build_ui(ctx: &Context, app: &mut App) {
    ctx.set_pixels_per_point(1.5);
    app.config.theme.apply(ctx); // todo do it only if changed!

    ctx.input(|i| {
        if !app.processing && !i.raw.dropped_files.is_empty() {
            app.handle_dropped_files(&i.raw.dropped_files);
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        menu_bar::build(ui, app);
        settings_panel::build(ui, app);
        ui.separator();

        // Small hint
        ui.label_size_centered(
            "Automatically collect assets used in your Source Engine maps.",
            10.0,
        );
        ui.label_size_centered("1. Set 'Game Dir' and 'Output Dir'.", 10.0);
        ui.label_size_centered("2. Drag & drop maps or use 'Add' button.", 10.0);
        ui.add_space(20.0);
        ui.separator();

        buttons_panel::build(ui, app);
        ui.separator();

        map_list_panel::build(ui, app);
        footer::build(ui, app);
    });
}
