use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Widget},
};
use ratatui_textarea::TextArea;

/// Popup to get a text input from the user
#[derive(Debug, Default)]
pub struct UserInputPopup<'a> {
    textarea: TextArea<'a>,
}

impl<'a> UserInputPopup<'a> {
    /// Handles user key events
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        log::debug!(
            "handling key event '{}' in user input popup",
            key_event.code
        );
        self.textarea.input(key_event);
    }

    /// Clears the current value within the text area
    pub fn clear(&mut self) {
        self.textarea.clear();
    }

    /// Returns the current text value within the text area (the first row)
    pub fn get_text(&self) -> Option<&String> {
        self.textarea.lines().first()
    }
}

impl<'a> Widget for &mut UserInputPopup<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let area = popup_area(area);
        self.textarea
            .set_block(Block::default().borders(Borders::ALL));
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea
            .set_placeholder_text("Enter the path to your config file to import ...");
        self.textarea.render(area, buf);
    }
}

/// Helper method to draw a new area on top of the given area
///
/// # Arguments
///
/// * `area` - The area that will partially be covered by the popup
fn popup_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
