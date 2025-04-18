use processing::ProcessingMessage;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc::Receiver;

use crate::enums::{Map, MapStatus, ProcessingStatus, WarningReason};
use crate::ui;

mod processing;

#[derive(Default, Serialize, Deserialize)]
pub struct BuilderGui {
    pub game_dir: String,
    pub output_dir: String,
    pub maps: Vec<Map>,

    #[serde(skip)]
    pub process_status: ProcessingStatus,
    #[serde(skip)]
    pub processing: bool,
    #[serde(skip)]
    pub processing_rx: Option<Receiver<ProcessingMessage>>,

    #[serde(skip)]
    pub internal: InternalData,

    #[cfg(debug_assertions)]
    #[serde(skip)]
    pub debug_hover: bool,
}

#[derive(Default)]
pub struct InternalData {
    pub unique_assets: u32,
    pub assets_found: u32,
    pub unique_assets_ui: u32,
    pub assets_found_ui: u32,
}

impl eframe::App for BuilderGui {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ui::build_ui(ctx, self);

        if ctx.input(|i| i.viewport().close_requested()) {
            self.save_config().expect("Data's will not saved."); // todo comm
        }

        if self.processing {
            // GUI update method to process messages from the processing thread
            self.poll_processing_events()
        }
    }
}

impl BuilderGui {
    pub fn new() -> Self {
        confy::load("sourcemods_builder", "config").unwrap_or_default()
    }

    pub fn save_config(&self) -> Result<(), confy::ConfyError> {
        log::info!("Saving data...");
        confy::store("sourcemods_builder", "config", &self)
    }

    #[rustfmt::skip]
    pub fn start_processing(&mut self) {
        let _ = self.save_config(); // autosave :p

        if self.maps.iter().all(|map| matches!(map.status, MapStatus::Completed)) {
            rfd::MessageDialog::new()
                .set_description("All maps already processed")
                .set_level(rfd::MessageLevel::Warning)
                .show();
            return;
        }

        if let Err(err) = self.process_maps() {
            rfd::MessageDialog::new()
                .set_description(&err)
                .set_level(rfd::MessageLevel::Error)
                .set_title("Error")
                .show();
        }
    }

    pub fn add_map(&mut self, path: &Path) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if self.maps.iter().any(|map| map.path == path) {
                return;
            }

            if ext == "vmf" || ext == "bsp" {
                let map = Map::new(path, ext == "vmf");
                self.maps.push(map);
            }
        }
    }

    pub fn add_maps(&mut self, path_dir: &Path) {
        for entry in sourcemods_builder::utils::iter_files(path_dir) {
            let path = entry.path();
            self.add_map(path);
        }
    }

    pub fn clear_maps(&mut self) {
        self.maps.clear();
    }

    pub fn remove_map(&mut self, index: usize) {
        self.maps.remove(index);
    }

    pub fn handle_dropped_files(&mut self, files: &[eframe::egui::DroppedFile]) {
        for file in files.iter().cloned() {
            if let Some(path) = &file.path {
                if path.is_dir() {
                    self.add_maps(path);
                } else {
                    self.add_map(path);
                }
            }
        }
    }
}
