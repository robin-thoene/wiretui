use crate::inbound::tui::styles::HIGHLIGHT_STYLE;
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

/// Popup that displays all available keymaps for the entire application
#[derive(Default)]
pub struct KeymapsPopup {}

impl Widget for KeymapsPopup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let area = popup_area(area);
        let keymaps = vec![
            Line::from("  ?          Open this help menu"),
            Line::from(" ESC         Close popup"),
            Line::from("  j          Scroll up"),
            Line::from("  k          Scroll up"),
            Line::from("SPACE        Toggle connection"),
            Line::from("  i          Import a new connection"),
            Line::from("Ctrl+d       Delete the selected connection"),
        ];
        let help_popup_content = Paragraph::new(keymaps).block(
            Block::bordered()
                .border_style(HIGHLIGHT_STYLE)
                .border_type(BorderType::Thick)
                .title(" Help ")
                .title_alignment(Alignment::Center),
        );
        help_popup_content.render(area, buf);
    }
}

/// Helper method to draw a new area on top of the given area
///
/// # Arguments
///
/// * `area` - The area that will partially be covered by the popup
fn popup_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(9)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(60)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
