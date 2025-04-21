use sourcemods_builder::find_asset_directories;
use sourcemods_builder::UniqueAssets;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use crate::enums::{Map, MapStatus, ProcessingStatus, WarningReason};

use super::BuilderGui;

/// Messages sent from the processing thread to the GUI thread.
pub enum ProcessingMessage {
    /// Update the overall processing status (e.g., ScanMaps, SearchAssets, CopyAssets).
    SetProcessingStatus(ProcessingStatus),
    /// Update a specific map's status.
    MapStatus { index: usize, status: MapStatus },
    /// Update the count of unique assets.
    UniqueAssetsCount(u32),
    /// Update the count of found assets.
    AssetsFoundCount(u32),
    /// Report an error and stop processing.
    Error(String),
    /// Indicate that processing has completed successfully.
    Complete,
}

// Helper function to change map status in GUI
fn change_map_status(tx: &Sender<ProcessingMessage>, index: usize, status: MapStatus) {
    let _ = tx.send(ProcessingMessage::MapStatus { index, status });
}

fn extract_panic_message(payload: Box<dyn std::any::Any + Send + 'static>) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        s.to_string()
    }
    else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone() 
    }
    else {
        "Unknown.".to_string()
    }
}

// Helper function to process assets and send the count to the GUI.
fn process_and_send<T, F>(process_fn: F, tx: &Sender<ProcessingMessage>) -> Vec<T>
where
    F: FnOnce() -> Vec<T>,
{
    // Execute the processing function
    let paths = process_fn();
    // Send the count of found assets
    let count = paths.len() as u32;
    let _ = tx.send(ProcessingMessage::AssetsFoundCount(count));
    paths
}

impl BuilderGui {
    pub fn process_maps(&mut self, cancel_flag: Arc<AtomicBool>) -> Result<(), String> {
        let game_path = Path::new(&self.config.game_dir).to_path_buf();
        let output_path = Path::new(&self.config.output_dir).to_path_buf();
        // We don't have map_dir, using game_path as a workaround
        sourcemods_builder::check_directories(&game_path, &game_path, &output_path)?;

        if self.config.maps.is_empty() {
            return Err("No maps to process.".to_string());
        }

        // Create a channel for inter-thread communication
        let (tx, rx): (Sender<ProcessingMessage>, Receiver<ProcessingMessage>) = mpsc::channel();
        // Store the receiver in the GUI state for later processing in the update loop
        self.processing_rx = Some(rx);

        self.processing = true;

        self.internal.assets_found = 0;
        self.internal.unique_assets = 0;
        self.internal.assets_found_ui = 0;
        self.internal.unique_assets_ui = 0;

        let maps_clone = self.config.maps.clone(); // It's not the best idea, but it works for now

        std::thread::spawn(move || {
            if let Err(err) = std::panic::catch_unwind(|| {
                BuilderGui::_process_maps(&tx, maps_clone, game_path, output_path, cancel_flag);
            }) {
                let err = extract_panic_message(err);
                let msg = format!(
                    "A critical error occurred during map processing! \
                    This is likely a bug. Please report it to the repository with the following details:\n\n\
                    Panic information: {err:?}\n\n\
                    Steps to reproduce (if known):\n\
                    1. Describe what you were doing when the error occurred.\n\
                    2. Provide the map files or configuration that caused the issue.\n\n\
                    Thank you for helping us improve the application! ;>"
                );
                let _ = tx.send(ProcessingMessage::Error(msg));
            }
        });

        Ok(())
    }

    /// The processing function running in a background thread.
    #[rustfmt::skip]
    pub fn _process_maps(
        tx: &Sender<ProcessingMessage>,
        maps_clone: Vec<Map>,
        game_dir: PathBuf,
        output_dir: PathBuf,
        is_cancelled: Arc<AtomicBool>
    ) {
        log::info!("Start processing {} maps.", maps_clone.len());
        let mut u_assets = UniqueAssets::default();
        let mut unique_count: u32 = 0;

        for (idx, map) in maps_clone.iter().enumerate() {
            // is should canceled?
            if is_cancelled.load(Ordering::SeqCst) { return }
            // is already processed?
            if matches!(map.status, MapStatus::Completed) { continue }

            // Notify GUI that this map processing has started
            change_map_status(&tx, idx, MapStatus::Processing);
            let _ = tx.send(ProcessingMessage::SetProcessingStatus(
                ProcessingStatus::ScanMap(idx),
            ));
            log::info!("Processing map {}", map.name);

            let parse_result: Result<(), String> = if map.is_vmf {
                // Extract unique assets from the VMF file
                u_assets.parse_vmf(&map.path)
                    .map_err(|err| err.to_string()) 
            } else {
                // Extract unique assets from the BSP file
                u_assets.parse_bsp(&map.path)
                    .map_err(|err| err.to_string())
            };

            if let Err(err_string) = parse_result {
                change_map_status(&tx, idx, MapStatus::Error(err_string));
                continue;
            }

            // Update unique assets count in GUI
            let len = u_assets.len() as u32;
            // If no unique assets found, send warning status
            if len - unique_count == 0 {
                change_map_status(&tx, idx, MapStatus::Warning(WarningReason::NotFoundAssets));
                continue;
            }
            unique_count = len;
            let _ = tx.send(ProcessingMessage::UniqueAssetsCount(unique_count));

            // Mark map as completed
            change_map_status(&tx, idx, MapStatus::Completed);
        }

        // Notify GUI that asset search is starting
        let _ = tx.send(ProcessingMessage::SetProcessingStatus(
            ProcessingStatus::SearchAssets,
        ));

        // Locate asset directories
        let (models_dirs, materials_dirs, sounds_dirs) = find_asset_directories(&game_dir);
        if is_cancelled.load(Ordering::SeqCst) { return }

        //-- region: processing paths
        // Process models using the helper function
        let models_paths = process_and_send(
            || sourcemods_builder::asset_processor::models::process(&mut u_assets, &models_dirs),
            &tx,
        );
        // Process materials using the helper function
        let materials_paths = process_and_send(
            || sourcemods_builder::asset_processor::materials::process(&u_assets, &materials_dirs),
            &tx,
        );
        // Process sounds using the helper function
        let sounds_paths = process_and_send(
            || sourcemods_builder::asset_processor::sounds::process(&u_assets, &sounds_dirs),
            &tx,
        );

        // If new unique assets were found during the processing, we update count in GUI
        let _ = tx.send(ProcessingMessage::UniqueAssetsCount(u_assets.len() as u32));
        if is_cancelled.load(Ordering::SeqCst) { return }
        //-- Endregion

        // Notify GUI that asset copying is starting
        let _ = tx.send(ProcessingMessage::SetProcessingStatus(
            ProcessingStatus::CopyAssets,
        ));

        // Copy model files
        if let Err(err) = sourcemods_builder::utils::copy_files(&models_paths, &output_dir, "models") {
            let _ = tx.send(ProcessingMessage::Error(format!("Failed to copy models: {}", err)));
            return;
        }
        // Copy material files
        if let Err(err) = sourcemods_builder::utils::copy_files(&materials_paths, &output_dir, "materials") {
            let _ = tx.send(ProcessingMessage::Error(format!("Failed to copy materials: {}", err)));
            return;
        }
        // Copy sound files
        if let Err(err) = sourcemods_builder::utils::copy_files(&sounds_paths, &output_dir, "sound") {
            let _ = tx.send(ProcessingMessage::Error(format!("Failed to copy sounds: {}", err)));
            return;
        }

        // Notify GUI that processing is complete
        let _ = tx.send(ProcessingMessage::Complete);
    }



    /// GUI update method to process messages from the processing thread.
    /// This function should be called regularly in your UI update loop.
    pub fn poll_processing_events(&mut self) {
        let mut is_cancelled = false;

        // Process messages if the receiver exists
        if let Some(rx) = &self.processing_rx {
            // Process all available messages without blocking
            for msg in rx.try_iter() {
                match msg {
                    ProcessingMessage::SetProcessingStatus(status) => {
                        // Update overall processing status
                        self.process_status = status;
                    }
                    ProcessingMessage::MapStatus { index, status } => {
                        // Update the status of a specific map if the index is valid
                        if let Some(map) = self.config.maps.get_mut(index) {
                            map.status = status;
                        }
                    }
                    ProcessingMessage::UniqueAssetsCount(count) => {
                        // Update the unique assets count in the internal state
                        self.internal.unique_assets = count;
                    }
                    ProcessingMessage::AssetsFoundCount(count) => {
                        // Update the assets found count in the internal state (accumulate count)
                        self.internal.assets_found += count;
                    }
                    ProcessingMessage::Error(err) => {
                        // Drop error message, set error state and stop processing
                        rfd::MessageDialog::new()
                            .set_description(&err)
                            .set_level(rfd::MessageLevel::Error)
                            .set_title("Processing Error!")
                            .show();
                        self.process_status = ProcessingStatus::ProcessingError(err);
                        is_cancelled = true;
                    }
                    ProcessingMessage::Complete => {
                        // Mark processing as completed
                        self.processing = false;
                        self.process_status = ProcessingStatus::Completed;
                    }
                }
            }
        }

        if is_cancelled {
            self.cancel_compile();
        }
    }
}
