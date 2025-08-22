use crate::inbound::tui::styles::{HIGHLIGHT_STYLE, SELECTED_STYLE};
use ratatui::{
    style::Style,
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget,
    },
};

/// Interactive list of available connections
/// Can be used to activate/deactivate VPN connections
pub struct ConnectionList<'a> {
    pub highlight: bool,
    pub connections: &'a Vec<String>,
}

#[derive(Debug, Default)]
pub struct ConnectionListState {
    pub list_state: ListState,
}

impl<'a> StatefulWidget for ConnectionList<'a> {
    type State = ConnectionListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let config_values_block = Block::new()
            .title(Line::raw(" Available configs ").left_aligned())
            .borders(Borders::all())
            .border_style(if self.highlight {
                HIGHLIGHT_STYLE
            } else {
                Style::default()
            })
            .border_type(if self.highlight {
                BorderType::Thick
            } else {
                BorderType::default()
            });
        let config_values_list_items: Vec<ListItem> = self
            .connections
            .iter()
            .map(|item| ListItem::from(item.as_str()))
            .collect();
        let config_values_list = List::new(config_values_list_items)
            .block(config_values_block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(config_values_list, area, buf, &mut state.list_state);
    }
}
