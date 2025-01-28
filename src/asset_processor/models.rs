use std::fs;

use super::{utils, PathBuf, UniqueAssets};
use log::{debug, warn};
use vmdl::Mdl; // Crate for parsing MDL files.

// List of model file extensions to check.
static EXTENSIONS: [&str; 7] = ["vtx", "dx90.vtx", "dx80.vtx", "sw.vtx", "vvd", "phy", "ani"];

/// Processes model assets, finding MDL and associated files.
/// Extracts material paths from MDL files.
pub fn process(u_assets: &mut UniqueAssets, models_dirs: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut models_paths: Vec<PathBuf> = Vec::new();

    for dir in models_dirs {
        for mdl in &u_assets.models_name {
            let mut path = dir.join(mdl);
            if !utils::ensure_correct_path(&mut path) {
                continue;
            }

            for ext in EXTENSIONS {
                let new_path = path.with_extension(ext);
                if new_path.exists() {
                    debug!("Found associated model file: {}", new_path.display());
                    models_paths.push(new_path);
                }
            }

            // Process materials from MDL file
            let data = fs::read(&path).unwrap_or_default();
            if let Ok(info) = Mdl::read(&data) {
                for info in info.textures {
                    for up in info.search_paths {
                        let relative_path = PathBuf::from(up).join(&info.name);
                        debug!(
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

    debug!(
        "Model processing finished. Found {} model paths.",
        models_paths.len()
    );
    models_paths
}
