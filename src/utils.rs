//! Utility functions module for sourcemods-builder.

use colored::*;
use fern::Dispatch;
use regex::Regex;
use std::{
    fs, io,
    path::{Path, PathBuf},
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

/// Ensures the correctness of a file path (case-insensitive).
pub fn ensure_correct_path(path: &mut PathBuf) -> bool {
    if path.exists() {
        return true;
    }
    if path.exists_no_case().unwrap_or(false) {
        if let Some(corrected_path) = path.find_case_insensitive().unwrap_or(None) {
            *path = corrected_path; // Update path with corrected case
        } else {
            return false;
        }
        true
    } else {
        false
    }
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

/// Trait extension for PathBuf for case-insensitive operations.
pub trait PathBufCaseExt {
    fn exists_no_case(&self) -> io::Result<bool>;
    fn find_case_insensitive(&self) -> io::Result<Option<PathBuf>>;
}

impl PathBufCaseExt for PathBuf {
    fn exists_no_case(&self) -> io::Result<bool> {
        Ok(self.find_case_insensitive()?.is_some())
    }

    fn find_case_insensitive(&self) -> io::Result<Option<PathBuf>> {
        let file_name = match self.file_name().and_then(|name| name.to_str()) {
            Some(name) => name.to_lowercase(),
            None => return Ok(None), // If no filename, return None.
        };

        let parent_dir = match self.parent() {
            Some(dir) => dir,
            None => return Ok(None), // If no parent directory, return None.
        };

        for entry in fs::read_dir(parent_dir)? {
            let entry = entry?;
            let entry_name = entry.file_name().to_string_lossy().to_lowercase();
            if entry_name == file_name {
                return Ok(Some(entry.path())); // Return path with correct casing
            }
        }

        Ok(None) // File not found
    }
}

pub fn iter_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
}
