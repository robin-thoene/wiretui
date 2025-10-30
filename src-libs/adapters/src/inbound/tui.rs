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
    activate_connection_port::ActivateConnectionPort, list_connections_port::ListConnectionsPort,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};
use std::{env, io};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct App<L, A>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
{
    show_help: bool,
    exit: bool,
    connections: Connections,
    list_connections_port: L,
    _activate_connection_port: A,
}

#[derive(Debug, Default)]
struct Connections {
    value: Vec<WireGuardConnection>,
    connection_list_state: ConnectionListState,
}

impl<L, A> App<L, A>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
{
    pub fn new(list_connections_port: L, activate_connection_port: A) -> Self {
        Self {
            show_help: bool::default(),
            exit: bool::default(),
            connections: Connections::default(),
            list_connections_port,
            _activate_connection_port: activate_connection_port,
        }
    }
    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        // Load all the available VPN config files only once at app startup
        self.init_connection_list();
        // Main loop
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
        self.connections.value = self.list_connections_port.get();
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

    /// Get the file locations of all available VPN config files
    /// from the configured directory
    fn _get_wireguard_config_files(&self) -> Vec<String> {
        let mut res = vec![];
        if let Some(mut dir) = env::home_dir() {
            dir.push("wireguard");
            let walker = WalkDir::new(dir).min_depth(1).into_iter();
            for entry in walker.filter_entry(|e| e.file_type().is_file()).flatten() {
                if let Some(s) = entry.path().to_str() {
                    res.push(s.to_string());
                }
            }
        }
        res
    }
}

impl<L, A> Widget for &mut App<L, A>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
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
