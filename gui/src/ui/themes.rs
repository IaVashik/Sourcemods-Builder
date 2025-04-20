use catppuccin_egui::Theme;
use eframe::egui;
use egui::Color32;
use serde::{Deserialize, Serialize};

/// Enumeration of available themes for the user interface.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Themes {
    // Custom themes from catppuccin_egui.
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    // Custom themes inspired by the Portal universe.
    BluePortal,
    OrangePortal,
    ChamberRust,
    // Default egui themes.
    #[default]
    DefaultDark,
    DefaultLight,
}

impl Themes {
    /// Applies the selected theme to the egui context
    pub fn apply(self, ctx: &egui::Context) {
        match self {
            Themes::Latte => catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE),
            Themes::Frappe => catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE),
            Themes::Macchiato => catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO),
            Themes::Mocha => catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA),
            Themes::BluePortal => catppuccin_egui::set_theme(ctx, PORTAL_BLUE),
            Themes::OrangePortal => catppuccin_egui::set_theme(ctx, PORTAL_ORANGE),
            Themes::ChamberRust => catppuccin_egui::set_theme(ctx, PORTAL_CHAMBER_RUST),
            Themes::DefaultLight => ctx.set_visuals(egui::Visuals::light()),
            Themes::DefaultDark => ctx.set_visuals(egui::Visuals::dark()),
        }
    }

    /// Returns a string representation for display in the UI
    pub fn as_str(self) -> &'static str {
        match self {
            Themes::Latte => "Latte (Light)",
            Themes::Frappe => "Frappe",
            Themes::Macchiato => "Macchiato",
            Themes::Mocha => "Mocha",
            Themes::DefaultDark => "Default (Dark)",
            Themes::DefaultLight => "Default (Light)",
            Themes::BluePortal => "Blue Portal",
            Themes::OrangePortal => "Orange Portal",
            Themes::ChamberRust => "Chamber Rust",
        }
    }
}

/// Custom Portal theme (Blue).
/// This is a custom theme inspired by Portal.
pub const PORTAL_BLUE: Theme = Theme {
    rosewater: Color32::from_rgb(120, 165, 210),
    flamingo: Color32::from_rgb(112, 154, 192),
    pink: Color32::from_rgb(96, 143, 178),
    mauve: Color32::from_rgb(60, 130, 170),
    red: Color32::from_rgb(30, 120, 160),
    maroon: Color32::from_rgb(20, 105, 145),
    peach: Color32::from_rgb(18, 95, 130),
    yellow: Color32::from_rgb(21, 89, 120),
    green: Color32::from_rgb(30, 95, 120),
    teal: Color32::from_rgb(40, 105, 140),
    sky: Color32::from_rgb(50, 117, 155),
    sapphire: Color32::from_rgb(35, 130, 180),
    blue: Color32::from_rgb(50, 155, 210),
    lavender: Color32::from_rgb(65, 140, 190),
    text: Color32::from_rgb(200, 215, 235),
    subtext1: Color32::from_rgb(180, 195, 215),
    subtext0: Color32::from_rgb(160, 175, 195),
    overlay2: Color32::from_rgb(140, 155, 175),
    overlay1: Color32::from_rgb(125, 135, 155),
    overlay0: Color32::from_rgb(110, 120, 140),
    surface2: Color32::from_rgb(85, 95, 110),
    surface1: Color32::from_rgb(70, 80, 95),
    surface0: Color32::from_rgb(55, 65, 80),
    base: Color32::from_rgb(25, 30, 40),
    mantle: Color32::from_rgb(18, 22, 30),
    crust: Color32::from_rgb(14, 17, 24),
};

/// Custom Portal theme (Orange).
/// This is a custom theme inspired by Portal.
pub const PORTAL_ORANGE: Theme = Theme {
    rosewater: Color32::from_rgb(235, 160, 90),
    flamingo: Color32::from_rgb(220, 130, 70),
    pink: Color32::from_rgb(210, 110, 55),
    mauve: Color32::from_rgb(195, 98, 48),
    red: Color32::from_rgb(185, 85, 42),
    maroon: Color32::from_rgb(170, 72, 35),
    peach: Color32::from_rgb(160, 70, 30),
    yellow: Color32::from_rgb(145, 65, 35),
    green: Color32::from_rgb(125, 60, 36),
    teal: Color32::from_rgb(110, 58, 40),
    sky: Color32::from_rgb(190, 105, 50),
    sapphire: Color32::from_rgb(200, 120, 60),
    blue: Color32::from_rgb(235, 135, 55),
    lavender: Color32::from_rgb(215, 125, 60),
    text: Color32::from_rgb(235, 220, 210),
    subtext1: Color32::from_rgb(215, 200, 190),
    subtext0: Color32::from_rgb(185, 170, 160),
    overlay2: Color32::from_rgb(155, 140, 130),
    overlay1: Color32::from_rgb(135, 120, 110),
    overlay0: Color32::from_rgb(115, 100, 90),
    surface2: Color32::from_rgb(95, 85, 80),
    surface1: Color32::from_rgb(75, 68, 60),
    surface0: Color32::from_rgb(60, 50, 45),
    base: Color32::from_rgb(30, 24, 20),
    mantle: Color32::from_rgb(24, 18, 14),
    crust: Color32::from_rgb(18, 13, 10),
};

/// Custom Portal theme (Chamber Rust).
/// This is a custom theme inspired by Portal.
pub const PORTAL_CHAMBER_RUST: Theme = Theme {
    rosewater: Color32::from_rgb(160, 130, 100),
    flamingo: Color32::from_rgb(170, 140, 110),
    pink: Color32::from_rgb(180, 150, 120),
    mauve: Color32::from_rgb(150, 120, 90),
    red: Color32::from_rgb(200, 80, 50),
    maroon: Color32::from_rgb(180, 70, 40),
    peach: Color32::from_rgb(240, 120, 40),
    yellow: Color32::from_rgb(210, 170, 60),
    green: Color32::from_rgb(80, 110, 70),
    teal: Color32::from_rgb(60, 100, 90),
    sky: Color32::from_rgb(120, 140, 100),
    sapphire: Color32::from_rgb(90, 110, 80),
    blue: Color32::from_rgb(70, 90, 110),
    lavender: Color32::from_rgb(130, 110, 100),
    text: Color32::from_rgb(235, 220, 210),
    subtext1: Color32::from_rgb(205, 190, 180),
    subtext0: Color32::from_rgb(185, 170, 160),
    overlay2: Color32::from_rgb(165, 150, 140),
    overlay1: Color32::from_rgb(145, 130, 120),
    overlay0: Color32::from_rgb(125, 110, 100),
    surface2: Color32::from_rgb(105, 95, 85),
    surface1: Color32::from_rgb(85, 75, 65),
    surface0: Color32::from_rgb(75, 65, 55),
    base: Color32::from_rgb(30, 30, 35),
    mantle: Color32::from_rgb(25, 25, 30),
    crust: Color32::from_rgb(20, 20, 25),
};
