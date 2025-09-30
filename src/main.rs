mod app;
mod audio;
mod errors;
mod ui;
mod utils;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
    utils::{helper, logger},
};

fn main() -> Result<()> {
    let mut args = std::env::args();
    // Skip binrary path
    args.next();
    let override_map =
        helper::handle_args(&mut args).expect("Something went wrong with reading args...");
    let dir = override_map
        .get("dir")
        .expect("Error occured: No directiory was setted.");

    // Initialize logging
    logger::init();

    let mut app = App::new(dir.to_string()).unwrap_or_else(|e| {
        error!("{}", e);
        panic!()
    });

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    app.run(&mut terminal).unwrap_or_else(|e| info!("{}", e));

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
