use crate::inbound::tui::styles::{HIGHLIGHT_STYLE, SELECTED_STYLE};
use domain::models::WireGuardConnection;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget,
    },
};
use std::fmt::Debug;

/// Interactive list of available connections
pub struct ConnectionList<'a> {
    pub highlight: bool,
    pub connections: &'a Vec<WireGuardConnection>,
}

#[derive(Debug, Default)]
pub struct ConnectionListState {
    pub list_state: ListState,
}

impl<'a> StatefulWidget for ConnectionList<'a> {
    type State = ConnectionListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
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
            .map(|item| {
                let active_indicator = if *item.get_is_active() { "*" } else { "" };
                ListItem::from(format!("{} {}", item.get_id(), active_indicator))
            })
            .collect();
        let config_values_list = List::new(config_values_list_items)
            .block(config_values_block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(config_values_list, area, buf, &mut state.list_state);
    }
}
