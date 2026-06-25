//! Terminal music player library.
//!
//! This crate assembles the application state, audio playback backend,
//! terminal UI, and shared utilities into a single runnable experience.

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

use crate::app::App;

/// Starts the terminal application for the provided music directory.
///
/// The function initializes the app state, enables alternate-screen mode,
/// runs the main event loop, and restores the terminal when the user exits.
pub fn run_app(dir: &str) -> Result<()> {
    let mut app = App::new(dir.to_string()).map_err(|e| {
        error!("Failed to create app: {}", e);
        e
    })?;

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    if let Err(e) = app.run(&mut terminal) {
        info!("App exited with error: {}", e);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
