use super::{utils, PathBuf, UniqueAssets};
use log::debug;

/// Processes sound assets, finding sound files.
pub fn process(u_assets: &UniqueAssets, sounds_dirs: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut sounds_paths: Vec<PathBuf> = Vec::new();

    for dir in sounds_dirs {
        for sound in &u_assets.sounds_name {
            let mut path = dir.join(sound);
            if !utils::ensure_correct_path(&mut path) {
                continue;
            }

            sounds_paths.push(path);
        }
    }

    debug!(
        "Sound processing finished. Found {} sound paths.",
        sounds_paths.len()
    );
    sounds_paths
}
