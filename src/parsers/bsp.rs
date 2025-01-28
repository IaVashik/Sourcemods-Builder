use crate::asset_processor::UniqueAssets;
use log::warn;
use std::path::Path;

/// Extracts unique assets from a BSP map file.
/// Currently, BSP parsing is not implemented.
pub fn get_uniques(_path: &Path, _uasset: &mut UniqueAssets) -> std::io::Result<()> {
    warn!("BSP support is not implemented.");
    Ok(())
}
