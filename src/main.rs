//! Command-line entry point for the terminal music player.

use anyhow::Result;
use mplayer::{
    run_app,
    utils::{helper::parse_args, logger},
};

/// Starts the application from the command line using the parsed arguments.
fn main() -> Result<()> {
    let args = parse_args();
    let dir = args.dir.to_str().expect("Invalid directory path");

    logger::init();
    run_app(dir)?;
    Ok(())
}
