use crate::inbound::tui::styles::HIGHLIGHT_STYLE;
use crossterm::event::KeyEvent;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Widget},
};
use ratatui_textarea::TextArea;

#[derive(Default, Debug)]
pub struct StatusBar<'a> {
    show_search: bool,
    search_textarea: TextArea<'a>,
}

impl<'a> StatusBar<'a> {
    pub fn show_search(&mut self) {
        self.show_search = true;
    }

    pub fn hide_search(&mut self) {
        self.show_search = false;
    }

    /// Handles key events
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        log::debug!(
            "handling key event '{}' in status bar search textarea",
            key_event.code
        );
        self.search_textarea.input(key_event);
    }

    /// Clears the current value within the text area
    pub fn clear(&mut self) {
        self.search_textarea.clear();
    }

    /// Returns the current text value within the text area (the first row)
    pub fn get_text(&self) -> Option<&String> {
        self.search_textarea.lines().first()
    }
}

impl<'a> Widget for &mut StatusBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.show_search {
            self.search_textarea.set_cursor_line_style(Style::default());
            self.search_textarea
                .set_placeholder_text("Enter a search term ...");
            self.search_textarea.set_placeholder_style(HIGHLIGHT_STYLE);
            self.search_textarea.render(area, buf);
        }

        let help_display = Line::from(vec![" Help ".into(), "<?> ".bold()]);
        Block::default()
            .title(help_display.right_aligned())
            .render(area, buf);
    }
}
