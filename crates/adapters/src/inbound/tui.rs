mod custom_widgets;
mod styles;

use crate::inbound::tui::custom_widgets::{
    connection_list::{ConnectionList, ConnectionListState},
    keymaps_popup::KeymapsPopup,
    status_bar::StatusBar,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use domain::models::WireGuardConnection;
use ports::inbound::{
    activate_connection_port::ActivateConnectionPort,
    deactivate_connection_port::DeactivateConnectionPort,
    list_connections_port::ListConnectionsPort,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};
use std::io;

#[derive(Debug)]
pub struct App<L, A, D>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
    D: DeactivateConnectionPort,
{
    show_help: bool,
    exit: bool,
    connections: Connections,
    list_connections_port: L,
    activate_connection_port: A,
    deactivate_connection_port: D,
}

#[derive(Debug, Default)]
struct Connections {
    value: Vec<WireGuardConnection>,
    connection_list_state: ConnectionListState,
}

impl<L, A, D> App<L, A, D>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
    D: DeactivateConnectionPort,
{
    pub fn new(
        list_connections_port: L,
        activate_connection_port: A,
        deactivate_connection_port: D,
    ) -> Self {
        Self {
            show_help: bool::default(),
            exit: bool::default(),
            connections: Connections::default(),
            list_connections_port,
            activate_connection_port,
            deactivate_connection_port,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        self.init_connection_list();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        ratatui::restore();

        Ok(())
    }

    /// Load the available VPN connections
    /// and import them to the app state
    fn init_connection_list(&mut self) {
        // TODO: error handling
        self.connections.value = self.list_connections_port.get().unwrap_or_default();
        if self
            .connections
            .connection_list_state
            .list_state
            .selected()
            .is_none()
            && !self.connections.value.is_empty()
        {
            // Ensure that at any time there is already a selected item
            self.connections
                .connection_list_state
                .list_state
                .select(Some(0));
        }
    }

    /// Renders the main application on the available frame
    ///
    /// # Arguments
    ///
    /// * `frame` - The main application frame to draw on
    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Central entry point for handling different kinds of events
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// Central entry point for handling key events
    ///
    /// # Arguments
    ///
    /// * `key_event` - The received key event
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // Make sure that quitting the application is always possible
        if key_event.code == KeyCode::Char('q') {
            self.exit();
            return;
        }
        if self.show_help {
            if key_event.code == KeyCode::Esc {
                self.close_help();
            }
            return;
        }
        match key_event.code {
            KeyCode::Char('?') if !self.show_help => self.open_help(),
            KeyCode::Char('j') => self
                .connections
                .connection_list_state
                .list_state
                .select_next(),
            KeyCode::Char('k') => self
                .connections
                .connection_list_state
                .list_state
                .select_previous(),
            KeyCode::Char(' ') => self.toggle_selected_connection(),
            _ => {}
        }
    }

    /// Close the application
    fn exit(&mut self) {
        self.exit = true;
    }

    /// Open the help menu i.e. the keymaps popup
    fn open_help(&mut self) {
        self.show_help = true;
    }

    /// Close the help menu i.e. the keymaps popup
    fn close_help(&mut self) {
        self.show_help = false
    }

    /// Toggles the selected connection. If it is active, deactivate it and vice versa
    fn toggle_selected_connection(&mut self) {
        // TODO: better handling of unexpected state and errors
        let idx = self.connections.connection_list_state.list_state.selected();
        if let Some(idx) = idx {
            let selected = self.connections.value.get(idx);
            if let Some(conn) = selected {
                if conn.get_is_active() == &true {
                    // Attempt to deactivate the selected connection if it is already active
                    let _ = self.deactivate_connection_port.deactivate(conn);
                } else {
                    // Attempt to activate the selected connection if it is not yet active
                    let _ = self.activate_connection_port.activate(conn);
                }
                // Refresh the connection list
                // TODO: use a more optimal way to mark successful activated conn as active
                // in ui
                self.connections.value = self.list_connections_port.get().unwrap_or_default();
            }
        }
    }
}

impl<L, A, D> Widget for &mut App<L, A, D>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
    D: DeactivateConnectionPort,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(area);

        let connection_list = ConnectionList {
            highlight: !self.show_help,
            connections: &self.connections.value,
        };
        connection_list.render(
            main_layout[0],
            buf,
            &mut self.connections.connection_list_state,
        );

        let status_bar = StatusBar {};
        status_bar.render(main_layout[1], buf);

        if self.show_help {
            let help_popup = KeymapsPopup {};
            help_popup.render(area, buf);
        }
    }
}
