use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use std::io;

mod views;
mod components;

pub struct UI {}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> anyhow::Result<()> {
        // Set up terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the app
        self.draw(&mut terminal)?;

        // Clean up terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
        loop {
            // Draw the UI
            terminal.draw(|f| {
                views::views(f);
            })?;

            // Handle input events
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
