use crate::{app::events::handle_events, audio::AudioPlayer, ui::UI};
use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use state::AppState;

mod events;
pub mod state;

pub struct App {
    ui: UI,
    state: AppState,
    player: AudioPlayer,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
            state: AppState::new(),
            player: AudioPlayer::new().unwrap(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Set up terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the app
        self.run_app(&mut terminal)?;

        // Clean up terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn run_app(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui.render_ui(f, &self.state))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if handle_events(Event::Key(key), &mut self.state)? {
                        break; // if pressed 'q' then exit
                    }
                    
                    if key.code == crossterm::event::KeyCode::Char('p') {
                        if self.state.is_playing {
                            if let Some(path) = &self.state.song_path {
                                self.player.play(path)?;
                            }
                        } else {
                            self.player.pause();
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
