mod app;
mod audio;
mod ui;
mod utils;

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{events::handle_events, state::AppState};
use audio::AudioPlayer;

use crate::utils::helper;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    // Skip binrary path
    args.next();
    let override_map = helper::handle_args(&mut args);

    // Initialize logging
    env_logger::init();

    let ui = ui::UI::new();
    let mut state = AppState::new();
    let player = match AudioPlayer::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error occured on creating AudioPlayer: {e}");
            panic!()
        }
    };

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    run_app(&mut terminal, &mut state, ui, player)?;

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
    player: AudioPlayer,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui.render_ui(f, state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if handle_events(Event::Key(key), state)? {
                    break; // if pressed 'q' then exit
                }

                if key.code == crossterm::event::KeyCode::Char('p') {
                    if state.is_playing {
                        if let Some(path) = &state.song_path {
                            player.play(&path)?;
                        }
                    } else {
                        player.pause();
                    }
                }
            }
        }
    }
    Ok(())
}
