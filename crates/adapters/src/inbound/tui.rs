mod custom_widgets;
mod styles;

use crate::inbound::tui::custom_widgets::{
    connection_list::{ConnectionList, ConnectionListState},
    keymaps_popup::KeymapsPopup,
    status_bar::StatusBar,
    user_input_popup::UserInputPopup,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use domain::models::WireGuardConnection;
use ports::inbound::{
    activate_connection_port::ActivateConnectionPort,
    deactivate_connection_port::DeactivateConnectionPort,
    import_connection_port::ImportConnectionPort, list_connections_port::ListConnectionsPort,
    remove_connection_port::RemoveConnectionPort,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};
use std::io;

#[derive(Debug)]
pub struct App<'a, L, A, D, I, R>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
    D: DeactivateConnectionPort,
    I: ImportConnectionPort,
    R: RemoveConnectionPort,
{
    show_help: bool,
    show_import_popup: bool,
    exit: bool,
    connections: Connections,
    import_popup: UserInputPopup<'a>,
    list_connections_port: L,
    activate_connection_port: A,
    deactivate_connection_port: D,
    import_connection_port: I,
    remove_connection_port: R,
}

#[derive(Debug, Default)]
struct Connections {
    value: Vec<WireGuardConnection>,
    connection_list_state: ConnectionListState,
}

impl<'a, L, A, D, I, R> App<'a, L, A, D, I, R>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
    D: DeactivateConnectionPort,
    I: ImportConnectionPort,
    R: RemoveConnectionPort,
{
    pub fn new(
        list_connections_port: L,
        activate_connection_port: A,
        deactivate_connection_port: D,
        import_connection_port: I,
        remove_connection_port: R,
    ) -> Self {
        Self {
            show_help: bool::default(),
            show_import_popup: bool::default(),
            exit: bool::default(),
            connections: Connections::default(),
            import_popup: UserInputPopup::default(),
            list_connections_port,
            activate_connection_port,
            deactivate_connection_port,
            import_connection_port,
            remove_connection_port,
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
        let connections_result = self.list_connections_port.get();
        if let Ok(conn) = connections_result {
            self.connections.value = conn;
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
        } else {
            // TODO: display error in UI
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
        log::debug!("user pressed '{}'", key_event.code);
        if self.show_import_popup {
            match key_event.code {
                KeyCode::Esc => self.close_import_popup(),
                KeyCode::Enter => self.import_new_connection(),
                _ => {
                    // Let the popup handle the key event
                    self.import_popup.handle_key_event(key_event);
                }
            }
            return;
        }
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
            KeyCode::Char('i') if !self.show_import_popup => self.open_import_popup(),
            KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.remove_selected_connection();
            }
            _ => {}
        }
    }

    /// Close the application
    fn exit(&mut self) {
        log::info!("triggering application shutdown");
        self.exit = true;
    }

    /// Open the help menu i.e. the keymaps popup
    fn open_help(&mut self) {
        log::info!("showing the help menu");
        self.show_help = true;
    }

    /// Close the help menu i.e. the keymaps popup
    fn close_help(&mut self) {
        log::info!("closing the help menu");
        self.show_help = false
    }

    /// Open the popup to get the user input for importing new connections
    fn open_import_popup(&mut self) {
        log::info!("showing the import popup");
        self.show_import_popup = true;
    }

    /// Close the popup to get the user input for importing new connections
    fn close_import_popup(&mut self) {
        log::info!("closing the import popup");
        self.show_import_popup = false;
        self.import_popup.clear();
    }

    /// Refresh the list of all connection and it's states
    fn refresh_connection_list(&mut self) {
        // TODO: use a more optimal way to mark successful activated conn as active
        let connections_result = self.list_connections_port.get();
        if let Ok(conn) = connections_result {
            log::debug!("refreshing connection list with the new value: {:?}", conn);
            self.connections.value = conn;
        } else {
            // TODO: display error in UI
        }
    }

    /// Import a new connection
    fn import_new_connection(&mut self) {
        // Get the current input from the user
        let user_input = self.import_popup.get_text();
        if let Some(user_input) = user_input {
            if user_input.trim().is_empty() {
                log::warn!("user entered empty or whitespace, skipping");
            } else {
                log::debug!(
                    "user input for importing a connection from config file: {}",
                    user_input
                );
                // Try to import a new connection from the given file path
                let result = self.import_connection_port.import_from_file(user_input);
                match result {
                    Ok(_) => self.refresh_connection_list(),
                    Err(err) => {
                        log::error!("error occurred while importing the connection: {}", err)
                    }
                }
            }
        } else {
            log::warn!(
                "user did not input a value for the path to a config file to import a new connection"
            )
        }
        self.close_import_popup();
    }

    /// Toggle the selected connection. If it is active, deactivate it and vice versa
    fn toggle_selected_connection(&mut self) {
        let idx = self.connections.connection_list_state.list_state.selected();
        if let Some(idx) = idx {
            let selected = self.connections.value.get(idx);
            if let Some(conn) = selected {
                if *conn.get_is_active() {
                    // Attempt to deactivate the selected connection if it is already active
                    let res = self.deactivate_connection_port.deactivate(conn);
                    if let Err(error) = res {
                        log::error!("{}", error);
                        // TODO: display error in UI
                    }
                } else {
                    // Attempt to activate the selected connection if it is not yet active
                    let res = self.activate_connection_port.activate(conn);
                    if let Err(error) = res {
                        log::error!("{}", error);
                        // TODO: display error in UI
                    }
                }
                self.refresh_connection_list();
            }
        }
    }

    /// Remove the currently selected connection
    fn remove_selected_connection(&mut self) {
        let idx = self.connections.connection_list_state.list_state.selected();
        if let Some(idx) = idx {
            let selected = self.connections.value.get(idx);
            if let Some(conn) = selected {
                let result = self.remove_connection_port.remove(conn);
                if let Err(error) = result {
                    log::error!("{}", error);
                    // TODO: display error in UI
                } else {
                    self.refresh_connection_list();
                }
            }
        }
    }
}

impl<'a, L, A, D, I, R> Widget for &mut App<'a, L, A, D, I, R>
where
    L: ListConnectionsPort,
    A: ActivateConnectionPort,
    D: DeactivateConnectionPort,
    I: ImportConnectionPort,
    R: RemoveConnectionPort,
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

        let status_bar = StatusBar::default();
        status_bar.render(main_layout[1], buf);

        if self.show_help {
            let help_popup = KeymapsPopup::default();
            help_popup.render(area, buf);
        }

        if self.show_import_popup {
            self.import_popup.render(area, buf);
        }
    }
}
