use serde::{Deserialize, Serialize};
use sourcemods_builder::{asset_processor, find_asset_directories, utils};
use sourcemods_builder::{parsers, UniqueAssets};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use crate::enums::{ErrorReason, Map, MapStatus, ProcessingStatus, WarningReason};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub game_dir: String,
    pub output_dir: String,
}

#[derive(Default)]
pub struct BuilderGui {
    pub config: Config,

    pub unique_assets: u32,
    pub assets_found: u32,
    pub unique_assets_ui: u32,
    pub assets_found_ui: u32,

    pub maps: Vec<Map>,
    pub process_status: ProcessingStatus,
    pub processing: bool,

    #[cfg(debug_assertions)]
    pub debug_hover: bool,
}

impl BuilderGui {
    pub fn new() -> Self {
        let config = confy::load("sourcemods_builder", "config").unwrap_or_default();
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn save_config(&self) -> Result<(), confy::ConfyError> {
        confy::store("sourcemods_builder", "config", &self.config)
    }

    pub fn add_map(&mut self, path: &Path) {
        // -> result?
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if self.maps.iter().any(|map| map.path == path) {
                return;
            }

            if ext == "vmf" {
                let map = Map::new(path, true);
                self.maps.push(map);
            }
            // TODO DEV CODE: because bsp is not supported at the moment.
            if ext == "bsp" {
                let mut map = Map::new(path, false);
                map.status = MapStatus::Warning(WarningReason::BspNotSupportNow);
                self.maps.push(map);
            }
        }
    }

    pub fn add_maps(&mut self, path: &Path) {
        for entry in sourcemods_builder::utils::iter_files(path) {
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

    pub fn process_maps(&mut self, app_mutex: Arc<Mutex<BuilderGui>>) -> Result<(), String> {
        let game_path = Path::new(&self.config.game_dir).to_path_buf();
        let output_path = Path::new(&self.config.output_dir).to_path_buf();
        // We don't have map_dir, using game_path as a workaround
        sourcemods_builder::check_directories(&game_path, &game_path, &output_path)?;

        if self.maps.is_empty() {
            return Err("No maps to process.".to_string());
        }

        if self
            .maps
            .iter()
            .all(|map| map.status == MapStatus::Completed)
        {
            return Err("All maps already processed.".to_string());
        }

        self.processing = true;
        self.process_status = ProcessingStatus::ScanMaps;

        self.assets_found = 0;
        self.unique_assets = 0;
        self.assets_found_ui = 0;
        self.unique_assets_ui = 0;

        let maps_clone = self.maps.clone();

        std::thread::spawn(move || {
            BuilderGui::_process_maps(app_mutex, maps_clone, game_path, output_path);
        });

        Ok(())
    }

    pub fn _process_maps(
        app_mutex: Arc<Mutex<BuilderGui>>,
        maps_clone: Vec<Map>,
        game_dir: PathBuf,
        output_dir: PathBuf,
    ) {
        log::info!("Start processing {} maps.", maps_clone.len());
        let mut u_assets = UniqueAssets::default();
        let mut last_processed: u32;

        for (idx, map) in maps_clone.iter().enumerate() {
            if !map.is_vmf || map.status == MapStatus::Completed {
                continue;
            }

            {
                let mut app = app_mutex.lock().unwrap();
                app.maps[idx].status = MapStatus::Processing;
            }
            log::info!("Start process {}", map.name);
            if let Err(err) = parsers::vmf::get_uniques(&map.path, &mut u_assets) {
                let mut app = app_mutex.lock().unwrap();
                app.maps[idx].status = MapStatus::Error(ErrorReason::VmfError(err));
                continue;
            }

            let mut app = app_mutex.lock().unwrap();
            let len = u_assets.len() as u32;
            last_processed = len - app.unique_assets;

            if last_processed == 0 {
                app.maps[idx].status = MapStatus::Warning(WarningReason::NotFoundAssets);
                continue;
            }

            app.unique_assets = len;
            app.maps[idx].status = MapStatus::Completed;
        }

        {
            let mut app = app_mutex.lock().unwrap();
            app.process_status = ProcessingStatus::SearchAssets;
        }
        let (models_dirs, materials_dirs, sounds_dirs) = find_asset_directories(&game_dir);

        // Processing assets
        let models_paths = asset_processor::models::process(&mut u_assets, &models_dirs);
        {
            let mut app = app_mutex.lock().unwrap();
            app.assets_found = models_paths.len() as u32;
            app.unique_assets = u_assets.len() as u32;
        }

        let materials_paths = asset_processor::materials::process(&u_assets, &materials_dirs);
        {
            let mut app = app_mutex.lock().unwrap();
            app.assets_found += materials_paths.len() as u32;
            app.unique_assets = u_assets.len() as u32;
        }

        let sounds_paths = asset_processor::sounds::process(&u_assets, &sounds_dirs);
        {
            let mut app = app_mutex.lock().unwrap();
            app.assets_found += sounds_paths.len() as u32;
            app.unique_assets = u_assets.len() as u32;
        }

        {
            let mut app = app_mutex.lock().unwrap();
            app.process_status = ProcessingStatus::CopyAssets;
        }

        if let Err(err) = utils::copy_files(&models_paths, &output_dir, "models") {
            BuilderGui::process_error(app_mutex, format!("Failed to copy models: {}", err));
            return;
        }
        if let Err(err) = utils::copy_files(&materials_paths, &output_dir, "materials") {
            BuilderGui::process_error(app_mutex, format!("Failed to copy materials: {}", err));
            return;
        }
        if let Err(err) = utils::copy_files(&sounds_paths, &output_dir, "sound") {
            BuilderGui::process_error(app_mutex, format!("Failed to copy sounds: {}", err));
            return;
        }

        let mut app = app_mutex.lock().unwrap();
        app.processing = false;
    }

    fn process_error(app_mutex: Arc<Mutex<BuilderGui>>, error_info: String) {
        log::error!("{}", error_info);
        {
            let mut app = app_mutex.lock().unwrap();
            app.process_status = ProcessingStatus::CopyError(error_info);
        }
        sleep(std::time::Duration::from_secs(4));

        let mut app = app_mutex.lock().unwrap();
        app.processing = false;
    }
}
