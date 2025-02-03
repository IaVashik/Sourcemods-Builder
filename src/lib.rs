//! Core library for sourcemods-builder application.

use log::error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

pub mod asset_processor;
pub mod parsers;
pub mod utils;

pub use asset_processor::UniqueAssets;

#[derive(Error, Debug)] // Derive Error trait for BuilderError
pub enum BuilderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("VMF parsing error: {0}")]
    VmfParseError(#[from] vmf_forge::VmfError),
    // #[error("BSP parsing error: {0}")] // todo Add this later when BSP parser is implemented
    // BspParseError(#[from] bsp_parser::error::BspError),
    #[error("Generic error: {0}")]
    GenericError(String), // For other types of errors
}

type BuilderResult<T> = Result<T, BuilderError>;

/// Checks if required directories exist and creates output directory if necessary.
pub fn check_directories(game_dir: &Path, map_dir: &Path, output_dir: &Path) -> Result<(), String> {
    // Game Dir
    if game_dir.to_str().unwrap_or_default().is_empty() {
        return Err("Game Dir is empty.".to_string());
    }
    if !game_dir.exists() {
        return Err(format!(
            "Game Dir \"{}\" doesn't exist.",
            game_dir.display()
        ));
    }

    // Map Dir
    if map_dir.to_str().unwrap_or_default().is_empty() {
        return Err("Map Dir is empty.".to_string());
    }
    if !map_dir.exists() {
        return Err(format!("Map Dir \"{}\" doesn't exist.", map_dir.display()));
    }

    // Output
    if output_dir.to_str().unwrap_or_default().is_empty() {
        return Err("Output is empty.".to_string());
    }
    if let Some(parent) = output_dir.parent() {
        if !parent.exists() {
            return Err("Output parent doesn't exist.".to_string());
        }
    }
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).map_err(|e| format!("Wrong output_dir: {}", e))?;
    }

    Ok(())
}

/// Finds asset directories within the game directory.
pub fn find_asset_directories(game_dir: &Path) -> (Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>) {
    let mut models_dirs: Vec<PathBuf> = Vec::new();
    let mut materials_dirs: Vec<PathBuf> = Vec::new();
    let mut sounds_dirs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(game_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        match entry.file_name().to_str().unwrap_or_default() {
            "models" => models_dirs.push(entry.path().parent().unwrap().to_path_buf()), // Parent of "models" dir
            "materials" => materials_dirs.push(entry.into_path()),
            "sound" => sounds_dirs.push(entry.into_path()),
            _ => {}
        }
    }

    (models_dirs, materials_dirs, sounds_dirs)
}
