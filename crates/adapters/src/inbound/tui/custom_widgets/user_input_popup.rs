use crate::inbound::tui::custom_widgets::popup::Popup;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};
use ratatui_textarea::TextArea;

/// Popup to get a text input from the user
#[derive(Debug, Default)]
pub struct UserInputPopup<'a> {
    textarea: TextArea<'a>,
    title: &'a str,
    placeholder: &'a str,
}

impl<'a> UserInputPopup<'a> {
    /// Create a new popup with custom title and placeholder
    pub fn new(title: &'a str, placeholder: &'a str) -> Self {
        Self {
            textarea: TextArea::default(),
            title,
            placeholder,
        }
    }

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
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_placeholder_text(self.placeholder);
        let popup = Popup::new(self.title, &self.textarea);
        popup.render(area, buf);
    }
}
