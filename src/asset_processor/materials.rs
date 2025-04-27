use super::{HashSet, PathBuf, UniqueAssets, utils};
use log::{debug, info};
use regex::Regex;
use std::sync::OnceLock;

// Static regex for extracting texture names from VMT files.
static RE: OnceLock<Regex> = OnceLock::new();

/// Returns a static Regex instance for parsing VMT texture paths.
fn get_regex() -> &'static Regex {
    RE.get_or_init(|| {
        Regex::new(r#"\$[^\s]+?"?\s+"?(([A-Z]|[a-z])[^\["\n\r.]+)"#).expect("Invalid regex pattern")
    })
}

/// Processes material assets to find VMT and VTF files.
///
/// Searches for VMT files based on unique material names and extracts VTF texture names.
/// Then, searches for VTF files based on the extracted texture names.
pub fn process(u_assets: &UniqueAssets, materials_dirs: &Vec<PathBuf>) -> Vec<PathBuf> {
    let re = get_regex();
    let mut materials_paths: Vec<PathBuf> = Vec::new();
    let mut textures_name: HashSet<String> = HashSet::with_capacity(512);

    // Search for VMT files based on unique material names.
    for dir in materials_dirs {
        for vmt in &u_assets.materials_name {
            let mut path = dir.join(vmt).with_extension("vmt");
            if !utils::ensure_correct_path(&mut path) {
                continue;
            }

            #[cfg(not(unix))]
            let path = dir.join(vmt).with_extension("vmt");
            #[cfg(unix)] // Source engine is not case-sensitive, unlike unix-like filesystems
            let path = match utils::find_asset_case_insensitive(dir, &vmt.clone().with_extension("vmt")) {
                Ok(Some(correct_path)) => correct_path,
                Ok(None) => {
                    log::debug!("Asset not found: {} in {}", vmt.display(), dir.display());
                    continue
                },
                Err(e) => {
                    log::warn!(
                        "Error searching for asset {} in {}: {}",
                        vmt.display(),
                        dir.display(),
                        e
                    );
                    continue;
                }
            };

            if !path.exists() { continue; }
            
            // Extract VTF texture names from VMT file content.
            info!("Extracting VTF texture names from VMT: {}", path.display());
            if let Ok(matches) = utils::find_all_groups_in_file(&path, re) {
                info!("  Found VTF textures in VMT: {:?}", matches);
                textures_name.extend(matches);
            } else {
                info!("  No VTF textures found in VMT or error reading file.");
            }
            materials_paths.push(path);
        }
    }

    // Search for VTF files based on extracted texture names.
    info!("Searching for VTF files based on extracted texture names...");
    for dir in materials_dirs {
        for vmt in &textures_name {
            #[cfg(not(unix))]
            let path = dir.join(vmt).with_extension("vtf");
            // Source engine is not case-sensitive, unlike unix-like filesystems
            #[cfg(unix)] 
            let path = match utils::find_asset_case_insensitive(dir, &PathBuf::from(vmt).with_extension("vtf")) {
                Ok(Some(correct_path)) => correct_path,
                Ok(None) => continue,
                Err(e) => {
                    log::warn!(
                        "Error searching for asset {} in {}: {}",
                        vmt,
                        dir.display(),
                        e
                    );
                    continue;
                }
            };
            if path.exists() {
                materials_paths.push(path);
            }
        }
    }

    info!(
        "Material processing finished. Found {} material paths.",
        materials_paths.len()
    );

    materials_paths
}
