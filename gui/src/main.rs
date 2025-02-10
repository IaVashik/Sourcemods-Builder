#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod enums;
mod ui;

use app::BuilderGui;
use eframe::{self, egui};
use log::LevelFilter;

#[rustfmt::skip]
const WIN_SIZE_X: f32 = if cfg!(target_os = "linux") { 330.0 } else { 500.0 };
#[rustfmt::skip]
const WIN_SIZE_Y: f32 = if cfg!(target_os = "linux") { 450.0 } else { 675.0 };

fn main() {
    sourcemods_builder::utils::setup_logger(LevelFilter::Debug).unwrap();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([WIN_SIZE_X, WIN_SIZE_Y])
            .with_inner_size([WIN_SIZE_X, WIN_SIZE_Y])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../../media/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Sourcemods Builder",
        options,
        Box::new(move |_cc| Ok(Box::new(BuilderGui::new()))),
    )
    .expect("Failed to run GUI app");
}
