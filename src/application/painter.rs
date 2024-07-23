use super::widgets::{bar, chunks, content, menu, tabs};
use super::MenuPointer;
use tui::style::{Color, Modifier, Style};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

pub struct Painter;

impl Painter {
    pub fn draw(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        menu_pointer: &MenuPointer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| {
            let size = frame.size();

            // Selected theme
            let (tabs_select_style, content_select_style, menu_select_style, bar_select_style) =
                self.selected_style(menu_pointer);

            // Widgets theme
            let tabs_style = Style::default().fg(Color::White);
            let content_style = Style::default().fg(Color::White);
            let menu_style = Style::default().fg(Color::White);
            let bar_style = Style::default().fg(Color::White);

            // Placeholders for the blocks
            let tabs = tabs::create(tabs_select_style, tabs_style);
            let content = content::create(content_select_style, content_style);
            let menu = menu::create(menu_select_style, menu_style);
            let bar = bar::create(bar_select_style, bar_style);
            
            // Creating chunks
            let (main_chunks, middle_chunks) = chunks::create(size);
            let tabs_chunk = main_chunks[0];
            let bar_chunk = main_chunks[2];
            let content_chunk = middle_chunks[0];
            let menu_chunk = middle_chunks[1];   

            // Render the widgets
            frame.render_widget(tabs, tabs_chunk);
            frame.render_widget(content, content_chunk);
            frame.render_widget(menu, menu_chunk);
            frame.render_widget(bar, bar_chunk);
        })?;
        Ok(())
    }

    fn selected_style(&self, pointer: &MenuPointer) -> (Style, Style, Style, Style) {
        let mut tabs_style = Style::default();
        let mut content_style = Style::default();
        let mut menu_style = Style::default();
        let mut bar_style = Style::default();

        if pointer.is_tabs_selected() {
            tabs_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
        } else if pointer.is_content_selected() {
            content_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
        } else if pointer.is_menu_selected() {
            menu_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
        } else if pointer.is_bar_selected() {
            bar_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
        }

        (tabs_style, content_style, menu_style, bar_style)
    }
}
