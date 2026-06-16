use anyhow::Result;
use mplayer::{
    run_app,
    utils::{helper::parse_args, logger},
};

fn main() -> Result<()> {
    let args = parse_args();
    let dir = args.dir.to_str().expect("Invalid directory path");

    // Initialize logging
    logger::init();
    run_app(dir)?;
    Ok(())
}
