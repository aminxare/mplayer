use clap::Parser;
use std::path::PathBuf;

/// A terminal-based music player built with Rust and Ratatui
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to scan for music files
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub dir: PathBuf,

    /// Verbose mode
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
