use crate::app::BuilderGui as App;
use crate::ui::themes::Themes;
use eframe::egui::{self, menu};

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    let mut theme_changed = false; // Variable to track changes

    menu::bar(ui, |ui| {
        ui.menu_button("Theme", |ui| {
            egui::ScrollArea::vertical()
                .max_height(152.)
                .show(ui, |ui| {
                    let theme: &mut Themes = &mut app.config.theme;
                    theme_changed |= ui.selectable_value(theme, Themes::DefaultDark, Themes::DefaultDark.as_str()).changed();
                    theme_changed |= ui.selectable_value(theme, Themes::DefaultLight, Themes::DefaultLight.as_str()).changed();
                    ui.separator();
                    theme_changed |= ui.selectable_value(theme, Themes::Latte, Themes::Latte.as_str()).changed();
                    theme_changed |= ui.selectable_value(theme, Themes::Frappe, Themes::Frappe.as_str()).changed();
                    theme_changed |= ui.selectable_value(theme, Themes::Macchiato, Themes::Macchiato.as_str()).changed();
                    theme_changed |= ui.selectable_value(theme, Themes::Mocha, Themes::Mocha.as_str()).changed();
                    ui.separator();
                    theme_changed |= ui.selectable_value(theme, Themes::BluePortal, Themes::BluePortal.as_str()).changed();
                    theme_changed |= ui.selectable_value(theme, Themes::OrangePortal, Themes::OrangePortal.as_str()).changed();
                    theme_changed |= ui.selectable_value(theme, Themes::ChamberRust, Themes::ChamberRust.as_str()).changed();
                });
            #[cfg(debug_assertions)]
            {
                let cb = ui.checkbox(&mut app.debug_hover, "Enable Debug");
                if cb.changed() {
                    ui.ctx().set_debug_on_hover(app.debug_hover);
                }
            }
        });
        if ui.button("About").clicked() {
            app.about_window_open = true;
        }
    });

    app.internal.theme_was_changed = theme_changed;
}
