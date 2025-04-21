use crate::app::BuilderGui as App;
use eframe::egui::{self, Layout, RichText, Ui};

use super::UiExt;

pub fn build(ui: &mut Ui, app: &mut App) {
    ui.horizontal(|ui| {
        let internal = &app.internal;
        let diff_unique = internal.unique_assets - internal.unique_assets_ui;
        let diff_found = internal.assets_found - internal.assets_found_ui;

        // Dynamic increment step. The greater the difference, the greater the step.
        if diff_unique > 0 {
            let increment_unique = (diff_unique / 10).max(1);
            app.internal.unique_assets_ui += increment_unique;
        }
        if diff_found > 0 {
            let increment_found = (diff_found / 10).max(1);
            app.internal.assets_found_ui += increment_found;
        }

        ui.label_sized(
            format!(
                "{maps} maps | {uassets} unique assets | {founded} assets found",
                maps = app.config.maps.len(), 
                uassets = app.internal.unique_assets_ui, 
                founded = app.internal.assets_found_ui
            ),
            8.0,
        );

        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            use egui::special_emojis::GITHUB;
            ui.hyperlink_to(
                RichText::new(format!("{GITHUB} GitHub repo")).size(8.0),
                "https://github.com/IaVashik/Sourcemods-Builder",
            );
        });
    });
}
