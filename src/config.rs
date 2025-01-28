use clap::Parser;
use log::LevelFilter;

use crate::utils;

/// Command-line arguments structure.
#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "laVashik",
    about = "sourcemods-builder is a utility that gathers map assets into a single location for streamlined SourceMod build compilation."
)]
pub struct Args {
    /// Path to the maps directory.
    pub maps_dir: String,
    /// Path to the game directory (e.g., csgo).
    pub game_dir: String,
    /// Path to the output directory where assets will be copied.
    pub output_dir: String,

    /// Enable verbose output (debug level logging).
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// Process only VMF files, skip BSP.
    #[arg(long, default_value_t = false)]
    pub ignore_vmf: bool,
    /// Process only BSP files, skip VMF.
    #[arg(long, default_value_t = false)]
    pub ignore_bsp: bool,
}

/// Parses command-line arguments.
pub fn get_args() -> Args {
    Args::parse()
}

/// Sets up the logger based on command-line arguments.
pub fn setup_logger(args: &Args) {
    let level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    utils::setup_logger(level).expect("Failed to initialize logger");
}
