use crate::GuiApp as AppWrapper;
use eframe::egui;
use egui::{CentralPanel, Context, RichText};

mod buttons_panel;
mod footer;
mod map_list_panel;
mod menu_bar;
mod settings_panel;

trait UiExt {
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> egui::Response;
}

impl UiExt for egui::Ui {
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> egui::Response {
        self.label(RichText::new(text.into()).size(size))
    }
}

pub fn build_ui(ctx: &Context, app_wrapper: &mut AppWrapper) {
    ctx.set_pixels_per_point(1.5);
    // Get MutexGuard to access BuilderGui
    let mut app = app_wrapper.gui_state.lock().unwrap();

    ctx.input(|i| {
        if !app.processing && !i.raw.dropped_files.is_empty() {
            app.handle_dropped_files(&i.raw.dropped_files);
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        menu_bar::build(ui, &mut app);
        settings_panel::build(ui, &mut app);
        ui.separator();

        // Small hint
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label_sized(
                "Automatically collect assets used in your Source Engine maps.",
                10.0,
            );
            ui.label_sized("1. Set 'Game Dir' and 'Output Dir'.", 10.0);
            ui.label_sized("2. Drag & drop maps or use 'Add' button.", 10.0);
            ui.add_space(20.0);
            ui.separator();
        });

        buttons_panel::build(ui, &mut app, app_wrapper.gui_state.clone());
        ui.separator();

        map_list_panel::build(ui, &mut app);
        footer::build(ui, &mut app);
    });
}
