use tui::style::Style;
use tui::widgets::{Block, Borders, List, ListItem};

pub fn create<'a>(block_style: Style, menu_style: Style) -> List<'a> {
    let list_items = prepare();
    List::new(list_items.clone())
        .block(
            Block::default()
                .title("Transactions")
                .borders(Borders::ALL)
                .style(block_style),
        )
        .style(menu_style)
}

fn prepare<'a>() -> [ListItem<'a>; 3] {
    [
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ]
}
