use ratatui::style::{Color, Modifier, Style};

pub const SELECTED_STYLE: Style = Style::new()
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);
pub const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Green);
