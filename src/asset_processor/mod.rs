//! This module handles the processing of game assets.
//!
//! It takes unique asset names identified by the parsers and uses them to
//! locate and process the actual asset files (models, materials, sounds)
//! within the game directory.

use log::info;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::BuilderResult;
use crate::parsers::{bsp, vmf};
use crate::utils;

pub mod materials;
pub mod models;
pub mod sounds;

/// Struct to hold unique asset names found in map files.
#[derive(Debug, Default)]
pub struct UniqueAssets {
    pub models_name: HashSet<PathBuf>,
    pub materials_name: HashSet<PathBuf>,
    pub sounds_name: HashSet<PathBuf>,
}

impl UniqueAssets {
    /// Parses a map directory to find unique assets (models, materials, etc.).
    pub fn parse_dir(mapdir: &Path, process_vmf: bool, process_bsp: bool) -> BuilderResult<Self> {
        let mut u_assets = UniqueAssets::default();

        for entry in utils::iter_files(mapdir) {
            let path = entry.path();

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                match ext {
                    "vmf" if process_vmf => vmf::get_uniques(path, &mut u_assets)?, // Parse VMF file
                    "bsp" if process_bsp => bsp::get_uniques(path, &mut u_assets)?, // Parse BSP file
                    _ => continue,
                }
            }

            info!("File \"{}\" processed.", path.display());
        }

        Ok(u_assets)
    }

    pub fn parse_vmf(vmf_path: &Path, u_assets: &mut UniqueAssets) -> BuilderResult<()> {
        Ok(vmf::get_uniques(vmf_path, u_assets)?)
    }

    pub fn parse_bsp(bsp_path: &Path, u_assets: &mut UniqueAssets) -> BuilderResult<()> {
        Ok(bsp::get_uniques(bsp_path, u_assets)?)
    }

    pub fn is_empty(&self) -> bool {
        self.models_name.is_empty() && self.materials_name.is_empty() && self.sounds_name.is_empty()
    }

    pub fn len(&self) -> usize {
        self.models_name.len() + self.materials_name.len() + self.sounds_name.len()
    }
}
