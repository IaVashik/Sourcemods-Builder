use crate::app::BuilderGui as App;
use eframe::egui::{self, menu};
use crate::ui::themes::Themes;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    menu::bar(ui, |ui| {
        ui.menu_button("Theme", |ui| {
            egui::ScrollArea::vertical()
                .max_height(152.)
                .show(ui, |ui| {
                    let theme: &mut Themes = &mut app.theme;
                    ui.selectable_value(theme, Themes::DefaultDark, Themes::DefaultDark.as_str());
                    ui.selectable_value(theme, Themes::DefaultLight, Themes::DefaultLight.as_str());
                    ui.separator();
                    ui.selectable_value(theme, Themes::Latte, Themes::Latte.as_str());
                    ui.selectable_value(theme, Themes::Frappe, Themes::Frappe.as_str());
                    ui.selectable_value(theme, Themes::Macchiato, Themes::Macchiato.as_str());
                    ui.selectable_value(theme, Themes::Mocha, Themes::Mocha.as_str());
                    ui.separator();
                    ui.selectable_value(theme, Themes::BluePortal, Themes::BluePortal.as_str());
                    ui.selectable_value(theme, Themes::OrangePortal, Themes::OrangePortal.as_str());
                    ui.selectable_value(theme, Themes::ChamberRust, Themes::ChamberRust.as_str());
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

        }
    });
}
