//! Command-line argument parsing helpers.

use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments accepted by the music player.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to scan for music files.
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub dir: PathBuf,

    /// Enables verbose logging output.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

/// Parses command-line arguments into the application configuration.
pub fn parse_args() -> Args {
    Args::parse()
}
