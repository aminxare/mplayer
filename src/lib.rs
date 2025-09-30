pub mod app;
pub mod audio;
pub mod errors;
pub mod ui;
pub mod utils;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
};

/// Runs the application lifecycle
pub fn run_app(dir: &str) -> Result<()> {
    let mut app = App::new(dir.to_string()).map_err(|e| {
        error!("Failed to create app: {}", e);
        e
    })?;

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    if let Err(e) = app.run(&mut terminal) {
        info!("App exited with error: {}", e);
    }

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
