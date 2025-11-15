use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

pub struct StatusBar {}

impl Widget for StatusBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let help_display = Line::from(vec![" Help ".into(), "<?> ".bold()]);
        Block::default()
            .title_bottom(help_display.right_aligned())
            .render(area, buf);
    }
}
