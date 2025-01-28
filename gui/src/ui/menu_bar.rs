use crate::app::BuilderGui as App;
use eframe::egui::{self, menu};
use egui_theme_switch::global_theme_switch;

pub fn build(ui: &mut egui::Ui, _app: &mut App) {
    menu::bar(ui, |ui| {
        ui.menu_button("Theme", |ui| {
            global_theme_switch(ui);
            #[cfg(debug_assertions)]
            {
                let cb = ui.checkbox(&mut _app.debug_hover, "Enable Debug");
                if cb.changed() {
                    ui.ctx().set_debug_on_hover(_app.debug_hover);
                }
            }
        });
    });
}
