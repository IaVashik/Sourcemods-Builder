use crate::app::BuilderGui as App;
use eframe::egui;
use egui::{CentralPanel, Context};

mod about_window;
mod buttons_panel;
mod ext;
mod footer;
mod map_list_panel;
mod menu_bar;
mod settings_panel;
pub mod themes;

pub use ext::UiExt;

pub fn build_ui(ctx: &Context, app: &mut App) {
    ctx.set_pixels_per_point(1.5);
    if app.internal.theme_was_changed {
        app.config.theme.apply(ctx); 
        app.internal.theme_was_changed = false;
    }

    ctx.input(|i| {
        if !app.processing && !i.raw.dropped_files.is_empty() {
            app.handle_dropped_files(&i.raw.dropped_files);
        }
    });

    // Process additional/immediate windows
    if app.about_window_open {
        ext::show_viewport_immediate(ctx, "About", [340., 380.], |ctx, _| {
            if about_window::show_about_window(ctx) {
                app.about_window_open = false;
            }
        })
    }

    CentralPanel::default().show(ctx, |ui| {
        menu_bar::build(ui, app);
        ui.add_enabled_ui(!app.processing, |ui| settings_panel::build(ui, app));
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
