use crate::inbound::tui::custom_widgets::popup::Popup;
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

/// Popup that displays all available keymaps for the entire application
#[derive(Default, Debug)]
pub struct KeymapsPopup {}

impl Widget for &mut KeymapsPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let keymaps = vec![
            Line::from("  ?          Open this help menu"),
            Line::from(" ESC         Close popup"),
            Line::from("  j          Scroll up"),
            Line::from("  k          Scroll up"),
            Line::from("SPACE        Toggle connection"),
            Line::from("  i          Import a new connection"),
            Line::from("Ctrl+d       Delete the selected connection"),
            Line::from("  /          Search the connection list"),
        ];
        let popup = Popup::new("Help", &keymaps);
        popup.render(area, buf);
    }
}
