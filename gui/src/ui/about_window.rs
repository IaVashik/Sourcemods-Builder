use eframe::egui::{self, Align, Frame, Layout, Margin, RichText};

use super::UiExt;

// Helper for icon + text label
fn icon_label(ui: &mut egui::Ui, icon: char, text: &str) {
    ui.horizontal(|ui| {
        let icon_color = ui.visuals().hyperlink_color;
        ui.colored_label(icon_color, RichText::new(icon.to_string()).size(14.0));
        ui.label(RichText::new(text).size(14.0).strong());
    });
}

pub fn show_about_window(ctx: &egui::Context) -> bool {
    let mut should_closed = false; 
    let section_frame = Frame::group(ctx.style().as_ref())
        .inner_margin(Margin::same(10)) 
        .fill(ctx.style().visuals.faint_bg_color); 

    egui::CentralPanel::default()
        .frame(Frame::central_panel(&ctx.style()).inner_margin(Margin::same(15)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Sourcemods Builder")
                        .size(28.0)
                        .strong()
                        .color(ui.visuals().strong_text_color()),
                );
                ui.label(
                    RichText::new(format!("Version: {}", env!("CARGO_PKG_VERSION")))
                        .color(ui.visuals().weak_text_color())
                        .size(11.0),
                );
                ui.label(
                    RichText::new("Asset Gathering Made Easy For Source Engine Mods")
                        .italics()
                        .size(8.0),
                );
            });

            // Use standard separator
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // Description
            ui.label(
                RichText::new(
                    "A utility to streamline Source Engine modding by automatically finding and \
                  organizing all required assets (.mdl, .vmt, .vtf, .wav, etc.) \
                  from your VMF and BSP map files into a clean output directory.",
                )
                .size(10.0),
            );
            ui.add_space(8.0);

            // Key Features
            section_frame.show(ui, |ui: &mut egui::Ui| {
                icon_label(ui, '✨', "Key Features");
                ui.add_space(5.0);
                ui.columns(3, |columns| {
                    columns[0].label_sized("• VMF/BSP Parsing", 8.);
                    columns[0].label_sized("• Model Collection", 8.);
                    columns[1].label_sized("• Material/Texture Collection", 8.);
                    columns[1].label_sized("• Sound Collection", 8.);
                    columns[2].label_sized("• Case-Insensitive Handling", 8.);
                    columns[2].label_sized("• GUI & CLI Available", 8.);
                });
            });
            ui.add_space(8.0);

            // Development & Credits
            section_frame.show(ui, |ui| {
                ui.columns(2, |columns| {
                    let ui = &mut columns[0];
                    icon_label(ui, '💻', "Development");
                    ui.horizontal(|ui| {
                        ui.label_sized("Developed by:", 10.);
                        ui.add(egui::Hyperlink::from_label_and_url(
                            RichText::new("laVashik")
                                .size(10.)
                                .color(ui.visuals().hyperlink_color),
                            "https://github.com/IaVashik",
                        ));
                    });
                    ui.label_sized("Yo guys, I'm coocked!", 8.);

                    let ui = &mut columns[1];
                    icon_label(ui, '🔗', "Core Dependencies");
                    egui::ScrollArea::vertical()
                        .max_height(80.0)
                        .show(ui, |ui| {
                            egui::CollapsingHeader::new(RichText::new("UI Framework").small())
                                .show(ui, |ui| {
                                    ui.hyperlink_to(
                                        RichText::new("egui / eframe").size(7.),
                                        "https://github.com/emilk/egui",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("catppuccin-egui").size(7.),
                                        "https://github.com/catppuccin/egui",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("rfd").size(7.),
                                        "https://crates.io/crates/rfd",
                                    );
                                });
                            egui::CollapsingHeader::new(RichText::new("File Parsers").small())
                                .show(ui, |ui| {
                                    ui.hyperlink_to(
                                        RichText::new("vmf-forge").size(7.),
                                        "https://crates.io/crates/vmf-forge",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("vbsp").size(7.),
                                        "https://crates.io/crates/vbsp",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("vmdl").size(7.),
                                        "https://crates.io/crates/vmdl",
                                    );
                                });
                            egui::CollapsingHeader::new(RichText::new("Core & Utilities").small())
                                .show(ui, |ui| {
                                    ui.hyperlink_to(
                                        RichText::new("clap (CLI args)").size(7.),
                                        "https://crates.io/crates/clap",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("log / fern (Logging)").size(7.),
                                        "https://crates.io/crates/log",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("confy (Config)").size(7.),
                                        "https://crates.io/crates/confy",
                                    );
                                    ui.hyperlink_to(
                                        RichText::new("thiserror (Errors)").size(7.),
                                        "https://crates.io/crates/thiserror",
                                    );
                                });
                        });
                });
            });
            ui.add_space(5.0);

            // Links Row
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let github_link = ui.add(egui::Hyperlink::from_label_and_url(
                        RichText::new(" GitHub Repo").color(ui.visuals().hyperlink_color), // special_emojis::GITHUB
                        "https://github.com/IaVashik/Sourcemods-Builder",
                    ));
                    if github_link.hovered() {
                        github_link.highlight();
                    }
                });
                
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let license_link = ui.add(egui::Hyperlink::from_label_and_url(
                        RichText::new("📜 License (MIT)").color(ui.visuals().hyperlink_color),
                        "https://github.com/IaVashik/Sourcemods-Builder/blob/main/LICENSE",
                    ));
                    if license_link.hovered() {
                        license_link.highlight();
                    }
                });
            });
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // Copyright (frfr)
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Copyright © 2025 laVashik")
                        .small()
                        .color(ui.visuals().weak_text_color()),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let close_button = ui.add(egui::Button::new(RichText::new("Close").strong()));
                    if close_button.clicked() {
                        should_closed = true
                    }
                    if close_button.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand); // todo
                    }
                });
            });
        });

    // Return true if close was requested by button OR window manager
    if should_closed || ctx.input(|i| i.viewport().close_requested()) {
        return true;
    }
    false
}
