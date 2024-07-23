use crossterm::event::{poll, read, DisableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use tui::{backend::CrosstermBackend, Terminal};

use application::MenuPointer;
use application::Painter;
use std::time::Duration;

pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut menu_pointer = MenuPointer::new();

    let mut key_pressed = false;

    loop {
        // Draw the UI
        Painter.draw(terminal, &mut menu_pointer)?;

        // Handle input from keyboard
        if poll(Duration::from_millis(200))? {
            if let Event::Key(key) = read()? {
                if !key_pressed {
                    match key.code {
                        KeyCode::Up => menu_pointer.up(),
                        KeyCode::Down => menu_pointer.down(),
                        KeyCode::Left => menu_pointer.left(),
                        KeyCode::Right => menu_pointer.right(),
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                    key_pressed = true;
                }
            }
        } else {
            key_pressed = false;
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
