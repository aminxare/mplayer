use anyhow::Result;
use mplayer::{
    run_app,
    utils::{helper::handle_args, logger},
};

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next(); // Skip binrary path

    let override_map = handle_args(&mut args).expect("Failed to parse CLI arguments.");
    let dir = override_map
        .get("dir")
        .expect("Error: No directory was provided.");

    // Initialize logging
    logger::init();
    run_app(dir)?;
    Ok(())
}
