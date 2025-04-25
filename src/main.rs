//! Main entry point for the sourcemod-builder-cli application.

use std::path::PathBuf;
use std::process::exit;

use log::{error, info, warn};

mod config;
use sourcemods_builder::utils;
use sourcemods_builder::{
    UniqueAssets, asset_processor, check_directories, find_asset_directories,
};

fn main() {
    let args = config::get_args();
    config::setup_logger(&args);

    let game_dir = PathBuf::from(&args.game_dir);
    let map_dir = PathBuf::from(&args.maps_dir);
    let output_dir = PathBuf::from(&args.output_dir);

    if let Err(err) = check_directories(&game_dir, &map_dir, &output_dir) {
        error!("{}", err);
        exit(1);
    }

    let u_assets_result = UniqueAssets::parse_dir(&map_dir, !args.ignore_vmf, !args.ignore_bsp);
    let mut u_assets = match u_assets_result {
        Ok(assets) => assets,
        Err(err) => {
            error!("Error processing maps: {}", err);
            exit(1);
        }
    };

    if u_assets.is_empty() {
        warn!("No available maps found in this folder.");
        exit(0)
    }

    let (models_dirs, materials_dirs, sounds_dirs) = find_asset_directories(&game_dir);

    // Processing assets
    let models_paths = asset_processor::models::process(&mut u_assets, &models_dirs);
    let materials_paths = asset_processor::materials::process(&u_assets, &materials_dirs);
    let sounds_paths = asset_processor::sounds::process(&u_assets, &sounds_dirs);

    let copied = models_paths.len() + materials_paths.len() + sounds_paths.len();
    if copied == 0 {
        error!("Nothing copied; no assets found.");
        exit(1);
    }

    if models_paths.is_empty() {
        warn!("No models for copying.");
    }
    if materials_paths.is_empty() {
        warn!("No materials for copying.");
    }
    if sounds_paths.is_empty() {
        warn!("No sounds for copying.");
    }

    // Copying assets to output directory
    utils::copy_files(&models_paths, &output_dir, "models").expect("Failed to copy models");
    utils::copy_files(&materials_paths, &output_dir, "materials")
        .expect("Failed to copy materials");
    utils::copy_files(&sounds_paths, &output_dir, "sound").expect("Failed to copy sounds");

    info!("Success! {} assets copied.", copied);
}

// TODO: particles
