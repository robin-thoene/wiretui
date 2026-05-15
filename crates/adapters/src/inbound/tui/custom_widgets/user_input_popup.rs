use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Widget},
};
use ratatui_textarea::TextArea;

/// Popup to get a text input from the user
#[derive(Debug, Default)]
pub struct UserInputPopup {}

impl Widget for UserInputPopup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let area = popup_area(area);
        let mut textarea = TextArea::default();
        textarea.set_block(Block::default().borders(Borders::ALL));
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Enter the path to your config file to import ...");
        textarea.render(area, buf);
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
