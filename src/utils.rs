//! Utility functions module for sourcemods-builder.

use colored::*;
use fern::Dispatch;
use regex::Regex;
use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

/// Sets up the global logger with specified level filter.
pub fn setup_logger(level: log::LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, _, record| {
            if record.level() == log::Level::Info {
                return out.finish(format_args!(
                    "{}",
                    format!("{}", record.args()).truecolor(200, 200, 250)
                ));
            };

            let level_color = match record.level() {
                log::Level::Error => "red",
                log::Level::Warn => "yellow",
                log::Level::Info => "NONE",
                log::Level::Debug => "blue",
                log::Level::Trace => "purple",
            };

            let colored_message =
                format!("{}: {}", record.level(), record.args()).color(level_color);
            out.finish(format_args!("{}", colored_message))
        })
        .level(level)
        .chain(io::stdout())
        .apply()?;

    Ok(())
}

/// Finds all captured groups from a regex in a file.
pub fn find_all_groups_in_file(file_path: &Path, re: &Regex) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    let matches: Vec<String> = re
        .captures_iter(&content)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().replace("\\", "/").to_lowercase())
        .collect();
    Ok(matches)
}

/// Constructs a destination path for copied files.
fn get_path(path: &Path, output_dir: &Path, base_folder: &str) -> io::Result<PathBuf> {
    let relative_path = path
        .iter()
        .skip_while(|part| part.to_str() != Some(base_folder))
        .collect::<PathBuf>();

    let destination = output_dir.join(&relative_path);

    if let Some(parent_dir) = destination.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    Ok(destination)
}

/// Copies multiple files to the output directory.
pub fn copy_files(paths: &Vec<PathBuf>, output_dir: &Path, base_folder: &str) -> io::Result<()> {
    for path in paths {
        let destination = get_path(path, output_dir, base_folder)?;
        fs::copy(path, &destination)?;
    }
    Ok(())
}

/// Attempts to locate an asset by its relative path within a given base directory,
/// performing a case-insensitive search for each path component.
///
/// This is useful for environments like Linux where the filesystem is case-sensitive,
/// but asset references might not match the exact casing.
///
/// # Arguments
///
/// * `base_dir` - The known-correct base directory where the search should start (e.g., `/path/to/game/materials`).
/// * `relative_asset_path` - The relative path of the asset to find (e.g., `Brick/Wall01.vtf`), potentially with incorrect casing.
///
/// # Returns
///
/// * `Ok(Some(PathBuf))` - If the asset is found, returns the full path with the correct casing.
/// * `Ok(None)` - If the asset or any intermediate directory component cannot be found case-insensitively.
/// * `Err(io::Error)` - If an I/O error occurs during directory traversal (e.g., permissions).
#[cfg(unix)]
pub fn find_asset_case_insensitive(
    base_dir: &Path,
    relative_asset_path: &Path,
) -> io::Result<Option<PathBuf>> {
    let mut current_path = base_dir.to_path_buf();

    // Iterate through each component of the relative path.
    for component in relative_asset_path.components() {
        let component_os_str = match component {
            Component::Normal(name) => name,
            // *Skip other component types like CurDir (.), ParentDir (..) for this relative search.
            _ => continue,
        };

        // Prepare the component name for case-insensitive comparison.
        let target_name_lower = match component_os_str.to_str() {
            Some(s) => s.to_lowercase(),
            None => {
                log::warn!(
                    "Path component contains non-UTF8 characters: {:?}",
                    component_os_str
                );
                return Ok(None); // todo: Or implement OsStr comparison logic?
            }
        };

        let mut found_match = false;
        // Read the contents of the current directory to find the next component.
        match fs::read_dir(&current_path) {
            Ok(entries) => {
                for entry_result in entries {
                    let entry = entry_result?;
                    let entry_os_name = entry.file_name();
                    let entry_name_str_lower = entry_os_name.to_string_lossy().to_lowercase();

                    if entry_name_str_lower == target_name_lower {
                        // Found a match. Update `current_path` to the actual path of the found entry.
                        current_path = entry.path();
                        found_match = true;
                        break;
                    }
                }
            }
            // Handle specific error kinds gracefully.
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(e) => {
                return Err(e); // Propagate the error.
            }
        }

        // If no matching entry was found in the directory for the current component.
        if !found_match {
            return Ok(None);
        }

        // Check if the found path is a directory before proceeding to the next component.
        // This prevents trying to `read_dir` on a file.
        let is_last_component = component == relative_asset_path.components().last().unwrap();
        if !is_last_component && !current_path.is_dir() {
            return Ok(None); // Cannot traverse further. :<
        }
    }

    Ok(Some(current_path))
}

pub fn iter_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
}
