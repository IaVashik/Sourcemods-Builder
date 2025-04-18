#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use std::{
    borrow::Cow, fmt::Display, path::{Path, PathBuf}
};

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum ProcessingStatus {
    #[default]
    Idle,
    ScanMap(usize),
    SearchAssets,
    CopyAssets,
    CopyError(String),
    Completed,
}

impl Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            ProcessingStatus::ScanMap(_) => "Scanning Maps...",
            ProcessingStatus::SearchAssets => "Searching Assets...",
            ProcessingStatus::CopyAssets => "Copying Assets...",
            ProcessingStatus::CopyError(info) => info,
            _ => ""
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MapStatus {
    Pending,
    Processing,
    Warning(WarningReason),
    Error(String),
    Completed,
}

impl Default for MapStatus {
    fn default() -> Self {
        MapStatus::Pending
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WarningReason {
    NotFoundAssets,
    Unknown, // never created? lmao
}

impl Display for MapStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            MapStatus::Pending => "⚫",
            MapStatus::Warning(_) => "⚠",
            MapStatus::Error(_) => "❌",
            MapStatus::Completed => "✅",
            _ => "",
        };
        write!(f, "{}", status_str)
    }
}

impl MapStatus {
    pub fn get_hover_text<'a>(&'a self) -> Cow<'a, str> {
        match self {
            MapStatus::Pending => "Pending".into(),
            MapStatus::Warning(reason) => match reason {
                WarningReason::NotFoundAssets => "New unique Assets not found in this map".into(),
                WarningReason::Unknown => "Unknown warning".into(),
            },
            MapStatus::Error(err_msg) => Cow::Borrowed(err_msg), // never use btw
            MapStatus::Completed => "Completed".into(),
            _ => "".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
    pub path: PathBuf,
    pub name: String,
    #[serde(skip)]
    pub status: MapStatus,
    pub is_vmf: bool,
}

impl Map {
    pub fn new(path: &Path, is_vmf: bool) -> Self {
        Self {
            path: path.to_path_buf(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            status: MapStatus::Pending,
            is_vmf,
        }
    }
}
