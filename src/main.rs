mod app;
mod audio;
mod errors;
mod ui;
mod utils;

use anyhow::Result;
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{events::handle_events, state::AppState};

use crate::utils::helper;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    // Skip binrary path
    args.next();
    let override_map =
        helper::handle_args(&mut args).expect("Something went wrong with reading args...");
    let dir = override_map
        .get("dir")
        .expect("Error occured: No directiory was setted.");

    // Initialize logging
    env_logger::init();

    let ui = ui::UI::new();
    let mut state = AppState::new(dir.clone())?;

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    run_app(&mut terminal, &mut state, ui)?;

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut AppState,
    ui: ui::UI,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui.render_ui(f, state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let e = event::read()?;
            if handle_events(e, state)? {
                break; // if pressed 'q' then exit
            }
        }
    }
    Ok(())
}
