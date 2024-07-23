use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn create(size: Rect) -> (Vec<Rect>, Vec<Rect>){
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

    (main_chunks, middle_chunks)
}