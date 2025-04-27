use std::fs;

use super::{PathBuf, UniqueAssets, utils};
use log::{info, warn};
use vmdl::Mdl; // Crate for parsing MDL files.

// List of model file extensions to check.
static EXTENSIONS: [&str; 7] = ["vtx", "dx90.vtx", "dx80.vtx", "sw.vtx", "vvd", "phy", "ani"];

/// Processes model assets, finding MDL and associated files.
/// Extracts material paths from MDL files.
pub fn process(u_assets: &mut UniqueAssets, models_dirs: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut models_paths: Vec<PathBuf> = Vec::new();

    for dir in models_dirs {
        for mdl in &u_assets.models_name {
            #[cfg(not(unix))]
            let path = dir.join(mdl);
            #[cfg(unix)] // Source engine is not case-sensitive, unlike unix-like filesystems
            let path = match utils::find_asset_case_insensitive(dir, mdl) {
                Ok(Some(correct_path)) => correct_path,
                Ok(None) => continue,
                Err(e) => {
                    warn!(
                        "Error searching for asset {} in {}: {}",
                        mdl.display(),
                        dir.display(),
                        e
                    );
                    continue;
                }
            };

            if !path.exists() { continue; }
            println!("valid path: {}", path.display());

            for ext in EXTENSIONS {
                let new_path = path.with_extension(ext);
                if new_path.exists() {
                    info!("Found associated model file: {}", new_path.display());
                    models_paths.push(new_path);
                }
            }

            // Process materials from MDL file
            let data = fs::read(&path).unwrap_or_default();
            if let Ok(info) = Mdl::read(&data) {
                // todo [panic]: not yet implemented: read animation from animation block
                for info in info.textures {
                    for up in info.search_paths {
                        let relative_path = PathBuf::from(up).join(&info.name);
                        info!(
                            "Extracted material path from MDL: {}",
                            relative_path.display()
                        );
                        u_assets.materials_name.insert(relative_path);
                    }
                }
            } else {
                warn!("Error parsing MDL file or no materials found.");
            }

            models_paths.push(path);
        }
    }

    info!(
        "Model processing finished. Found {} model paths.",
        models_paths.len()
    );
    models_paths
}
