use ratatui::style::{Color, Modifier, Style};

pub const SELECTED_STYLE: Style = Style::new()
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);
pub const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Green);
pub const INFO_STYLE: Style = Style::new().fg(Color::Blue);
pub const WARN_STYLE: Style = Style::new().fg(Color::Yellow);
pub const ERR_STYLE: Style = Style::new().fg(Color::Red);
