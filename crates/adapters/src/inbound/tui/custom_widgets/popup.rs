use crate::inbound::tui::styles::HIGHLIGHT_STYLE;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};
use ratatui_textarea::TextArea;

/// Custom trait to support rendering different content types
pub trait PopupContent {
    /// Rendering the content as ratatui widget
    fn render(&self, area: Rect, buf: &mut Buffer);

    /// Report the desired content height
    fn desired_height(&self) -> u16;
}

impl<'a> PopupContent for Vec<Line<'a>> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let content = Paragraph::new(self.clone());
        content.render(area, buf);
    }

    fn desired_height(&self) -> u16 {
        self.len() as u16
    }
}

impl<'a> PopupContent for TextArea<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(self, area, buf);
    }

    fn desired_height(&self) -> u16 {
        1
    }
}

/// Reusable popup to display content to the user
#[derive(Debug)]
pub struct Popup<'a, C>
where
    C: PopupContent,
{
    title: &'a str,
    content: &'a C,
    custom_border_style: Option<Style>,
}

impl<'a, C> Popup<'a, C>
where
    C: PopupContent,
{
    /// Creates a new popup are to display the given content
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display
    /// * `content` - The actual content as a type that is supported to be rendered
    pub fn new(title: &'a str, content: &'a C, custom_border_style: Option<Style>) -> Self {
        Self {
            title,
            content,
            custom_border_style,
        }
    }

    /// Helper method to draw a new area on top of the given area to use for the popup content
    ///
    /// # Arguments
    ///
    /// * `area` - The area that will partially be covered by the popup
    fn build_popup_area(&self, area: Rect) -> Rect {
        let height = self.content.desired_height() + 2;
        let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(60)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}

impl<'a, C> Widget for Popup<'a, C>
where
    C: PopupContent,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = self.build_popup_area(area);
        Clear.render(area, buf);
        let title_block = Block::bordered()
            .border_style(self.custom_border_style.unwrap_or(HIGHLIGHT_STYLE))
            .border_type(BorderType::Thick)
            .title(format!(" {} ", self.title))
            .title_alignment(Alignment::Center);
        let content_area = title_block.inner(area);
        title_block.render(area, buf);
        self.content.render(content_area, buf);
    }
}
