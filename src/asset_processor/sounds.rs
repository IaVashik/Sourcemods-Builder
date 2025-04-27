use super::{PathBuf, UniqueAssets, utils};

/// Processes sound assets, finding sound files.
pub fn process(u_assets: &UniqueAssets, sounds_dirs: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut sounds_paths: Vec<PathBuf> = Vec::new();

    for dir in sounds_dirs {
        for sound in &u_assets.sounds_name {
            #[cfg(not(unix))]
            let path = dir.join(sound);
            #[cfg(unix)] // Source engine is not case-sensitive, unlike unix-like filesystems
            let path = match utils::find_asset_case_insensitive(dir, sound) {
                Ok(Some(correct_path)) => correct_path,
                Ok(None) => continue,
                Err(e) => {
                    log::warn!(
                        "Error searching for asset {} in {}: {}",
                        sound.display(),
                        dir.display(),
                        e
                    );
                    continue;
                }
            };

            if path.exists() {
                sounds_paths.push(path);
            }
        }
    }

    log::debug!(
        "Sound processing finished. Found {} sound paths.",
        sounds_paths.len()
    );
    sounds_paths
}
