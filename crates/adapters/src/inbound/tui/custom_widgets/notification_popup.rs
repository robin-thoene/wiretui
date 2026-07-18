use crate::inbound::tui::{
    custom_widgets::popup::Popup,
    styles::{ERR_STYLE, INFO_STYLE, WARN_STYLE},
};
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget};

#[derive(Debug, Default)]
pub enum NotificationLevel {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Default)]
pub struct NotificationPopup {
    message: String,
    level: NotificationLevel,
}

impl NotificationPopup {
    /// Update the notification message with a new value
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    /// Update the severity of the notification
    pub fn set_level(&mut self, level: NotificationLevel) {
        self.level = level;
    }

    /// Get the notification popup title
    fn get_title(&self) -> &str {
        match self.level {
            NotificationLevel::Info => "Info",
            NotificationLevel::Warn => "Warning",
            NotificationLevel::Error => "Error",
        }
    }

    fn get_style(&self) -> Style {
        match self.level {
            NotificationLevel::Info => INFO_STYLE,
            NotificationLevel::Warn => WARN_STYLE,
            NotificationLevel::Error => ERR_STYLE,
        }
    }
}

impl Widget for &mut NotificationPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = vec![Line::from(self.message.to_string())];
        let popup = Popup::new(self.get_title(), &content, Some(self.get_style()));
        popup.render(area, buf);
    }
}
