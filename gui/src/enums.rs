#![allow(dead_code)]

use std::{
    borrow::Cow, fmt::Display, path::{Path, PathBuf}
};

use sourcemods_builder::parsers::vmf::VmfError;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum ProcessingStatus {
    #[default]
    ScanMaps,
    SearchAssets,
    CopyAssets,
}

impl Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            ProcessingStatus::ScanMaps => "Scanning Maps...",
            ProcessingStatus::SearchAssets => "Searching Assets...",
            ProcessingStatus::CopyAssets => "Copying Assets...",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MapStatus {
    Pending,
    Processing,
    Warning(WarningReason),
    Error(ErrorReason),
    Completed,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WarningReason {
    BspNotSupportNow, // temp
    NotFoundAssets,
    Unknown,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ErrorReason {
    IoError(std::io::ErrorKind),
    VmfError(VmfError),
    Unknown,
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
    pub fn get_hover_text(&self) -> Cow<'static, str> {
        match self {
            MapStatus::Pending => "Pending".into(),
            MapStatus::Warning(reason) => match reason {
                WarningReason::BspNotSupportNow => ".bsp format not supported now".into(),
                WarningReason::NotFoundAssets => "New unique Assets not found in this map".into(),
                WarningReason::Unknown => "Unknown warning".into(),
            },
            MapStatus::Error(reason) => match reason {
                ErrorReason::IoError(error_kind) => error_kind.to_string().into(),
                ErrorReason::VmfError(vmf_error) => vmf_error.to_string().into(),
                ErrorReason::Unknown => "Unknown error".into(),
            },
            MapStatus::Completed => "Completed".into(),
            _ => "".into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Map {
    pub path: PathBuf,
    pub name: String,
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
