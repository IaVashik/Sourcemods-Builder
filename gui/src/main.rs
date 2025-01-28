#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod enums;
mod ui;

use std::sync::{Arc, Mutex};
use app::BuilderGui;
use eframe::{self, egui};
use log::LevelFilter;

const WIN_SIZE_X: f32 = if cfg!(target_os = "linux") { 330.0 } else { 500.0 };
const WIN_SIZE_Y: f32 = if cfg!(target_os = "linux") { 450.0 } else { 675.0 };

fn main() {
    sourcemods_builder::utils::setup_logger(LevelFilter::Info).unwrap();

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

    // Create BuilderGui and wrap it in Arc<Mutex<>>
    let gui_state = Arc::new(Mutex::new(BuilderGui::new()));
    let gui_clone = gui_state.clone();

    eframe::run_native(
        "Sourcemods Builder",
        options,
        Box::new(move |_cc| {
            // Use a move closure to capture Arc
            Ok(Box::new(GuiApp {
                gui_state: gui_clone, // Clone Arc for GuiApp
            }))
        }),
    ).expect("Failed to run GUI app");
    
    let _ = gui_state.lock().unwrap()
        .save_config();
}

// Create a wrapper structure for eframe::App that owns Arc<Mutex<BuilderGui>>
struct GuiApp {
    gui_state: Arc<Mutex<BuilderGui>>,
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Get MutexGuard to access BuilderGui
        ui::build_ui(ctx, self); // Pass a mutable reference to BuilderGui
    }
}
