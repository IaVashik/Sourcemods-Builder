use processing::ProcessingMessage;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync;
use std::sync::mpsc::Receiver;

use crate::enums::{Map, MapStatus, ProcessingStatus};
use crate::ui;

mod processing;

#[derive(Default)]
pub struct BuilderGui {
    pub config: StorageSettings,
    pub process_status: ProcessingStatus,
    pub processing: bool,
    pub processing_rx: Option<Receiver<ProcessingMessage>>,
    pub processing_cancel_flag: Option<sync::Arc<sync::atomic::AtomicBool>>,

    pub internal: InternalData,

    #[cfg(debug_assertions)]
    pub debug_hover: bool,

    // additionals windows
    pub about_window_open: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct StorageSettings {
    pub game_dir: String,
    pub output_dir: String,
    pub maps: Vec<Map>,
    pub theme: ui::themes::Themes,
}

#[derive(Default)]
pub struct InternalData {
    pub unique_assets: u32,
    pub assets_found: u32,
    pub unique_assets_ui: u32,
    pub assets_found_ui: u32,
    pub theme_was_changed: bool,
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
        let config = confy::load("sourcemods_builder", "config").unwrap_or_default();
        let mut app = Self {
            config,
            ..Default::default()
        };
        app.internal.theme_was_changed = true; // a little hack
        app
    }

    pub fn save_config(&self) -> Result<(), confy::ConfyError> {
        log::info!("Saving data...");
        confy::store("sourcemods_builder", "config", &self.config)
    }

    #[rustfmt::skip]
    pub fn start_processing(&mut self) {
        let _ = self.save_config(); // autosave :p

        if self.config.maps.iter().all(|map| matches!(map.status, MapStatus::Completed)) {
            rfd::MessageDialog::new()
                .set_description("All maps already processed")
                .set_level(rfd::MessageLevel::Warning)
                .show();
            return;
        }

        let cancel_flag = sync::Arc::new(sync::atomic::AtomicBool::new(false));
        self.processing_cancel_flag = Some(cancel_flag.clone());

        if let Err(err) = self.process_maps(cancel_flag) {
            rfd::MessageDialog::new()
                .set_description(&err)
                .set_level(rfd::MessageLevel::Error)
                .set_title("Error")
                .show();
        }
    }

    pub fn cancel_compile(&mut self) {
        if let Some(cancel_flag) = &self.processing_cancel_flag {
            cancel_flag.store(true, sync::atomic::Ordering::SeqCst);
        }

        // Update GUI state to reflect cancellation
        self.processing = false;
        self.process_status = ProcessingStatus::Cancelled;

        // Clear backend receiver and cancel flag to reset compilation state.
        self.processing_rx = None;
        self.processing_cancel_flag = None;

        // Reset maps status to pending if they were processing
        for m in self.config.maps.iter_mut() {
            if let MapStatus::Processing = m.status {
                m.status = MapStatus::Pending;
            }
        }
    }

    pub fn add_map(&mut self, path: &Path) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if self.config.maps.iter().any(|map| map.path == path) {
                return;
            }

            if ext == "vmf" || ext == "bsp" {
                let map = Map::new(path, ext == "vmf");
                self.config.maps.push(map);
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
        self.config.maps.clear();
        self.internal.unique_assets = 0;
        self.internal.unique_assets_ui = 0;
    }

    pub fn remove_map(&mut self, index: usize) {
        self.config.maps.remove(index);
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
