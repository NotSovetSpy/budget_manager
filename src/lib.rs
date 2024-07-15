use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode,
    LeaveAlternateScreen,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use application::painter::Painter;

pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Draw the UI
        Painter.draw(terminal)?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Ending program
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
