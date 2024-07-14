use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen, SetTitle,
};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{BarChart, Block, Borders, List, ListItem, Tabs},
    Terminal,
};

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            let size = frame.size();

            // Main chunks, 3 rows
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(15),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(size);

            // Split middle row into 2 columns
            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(main_chunks[1]);

            let tabs_chunk = main_chunks[0];
            let bar_chunk = main_chunks[2];
            let content_chunk = middle_chunks[0];
            let menu_chunk = middle_chunks[1];

            // Placeholders for the blocks
            let titles = ["Tab1", "Tab2", "Tab3", "Tab4"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();
            let tabs =
                Tabs::new(titles).block(Block::default().title("Tabs").borders(Borders::ALL));

            let list_items = [
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ];
            let content = List::new(list_items.clone())
                .block(Block::default().title("Transactions").borders(Borders::ALL));

            let menu = List::new(list_items.clone())
                .block(Block::default().title("Menu").borders(Borders::ALL));

            let bar = BarChart::default()
                .block(Block::default().title("BarChart").borders(Borders::ALL))
                .bar_width(3)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
                .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .label_style(Style::default().fg(Color::White))
                .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
                .max(4);

            frame.render_widget(tabs, tabs_chunk);
            frame.render_widget(content, content_chunk);
            frame.render_widget(menu, menu_chunk);
            frame.render_widget(bar, bar_chunk);
        })?;

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
