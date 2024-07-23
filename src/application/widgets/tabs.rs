use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

pub fn create<'a>(block_style: Style, tabs_style: Style) -> Tabs<'a> {
    let titles = prepare(tabs_style);
    let active_tab_style = Style::default().fg(tui::style::Color::Yellow);

    Tabs::new(titles)
        .block(
            Block::default()
                .title("Budgets")
                .borders(Borders::ALL)
                .style(block_style),
        )
        .style(tabs_style)
        .highlight_style(active_tab_style)
}

fn prepare<'a>(tabs_style: Style) -> Vec<Spans<'a>> {
    // TODO: Get from database
    ["Budget 1", "Budget 2", "Budget 3", "Budget 4"]
        .iter()
        .cloned()
        .map(|title| Spans::from(Span::styled(title, tabs_style)))
        .collect()
}
