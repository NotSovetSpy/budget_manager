use tui::style::{Color, Modifier, Style};
use tui::widgets::{BarChart, Block, Borders};

#[allow(unused_variables)]
pub fn create<'a>(block_style: Style, bar_style:Style) -> BarChart<'a> {
    BarChart::default()
        .block(
            Block::default()
                .title("BarChart")
                .borders(Borders::ALL)
                .style(block_style),
        )
        .bar_width(3)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
        .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::White))
        .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
        .max(4)
}