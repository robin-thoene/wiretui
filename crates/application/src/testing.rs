use domain::models::WireGuardConnection;
use ports::outbound::wireguard_port::{
    ConnectionActivationError as AdapterConnectionActivationError,
    ConnectionDeactivationError as AdapterConnectionDeactivationError,
    GetConnectionsError as AdapterGetConnectionsError, WireGuardPort,
};
use std::{cell::RefCell, path::PathBuf};

/// Mock for the WireGuardNmRepo
#[derive(Default)]
pub struct WireGuardNmRepoMock {
    /// Internal state of mocked imported WireGuard connections to use for testing
    available_connections: RefCell<Vec<WireGuardConnection>>,
    /// The error type the mock shall return when the `get_imported_connections` fn is invoked
    get_imported_connections_error: Option<AdapterGetConnectionsError>,
}

/// Mock implementation of the WireGuardNmRepo
impl WireGuardNmRepoMock {
    /// Create a new mock instance with a given initial state of available connections
    pub fn new(initial_connections: Vec<WireGuardConnection>) -> Self {
        Self {
            available_connections: RefCell::new(initial_connections),
            get_imported_connections_error: None,
        }
    }

    /// Create a new mock instance with a given initial state of available connections and
    /// expected errors that shall be returned as part of the mock
    pub fn new_with_error(
        initial_connections: Vec<WireGuardConnection>,
        get_imported_connections_error: AdapterGetConnectionsError,
    ) -> Self {
        Self {
            available_connections: RefCell::new(initial_connections),
            get_imported_connections_error: Some(get_imported_connections_error),
        }
    }

    /// Get the internal mock state of all connections
    pub fn get_all_connections(&self) -> Vec<WireGuardConnection> {
        let conn: Vec<WireGuardConnection> = self
            .available_connections
            .borrow()
            .iter()
            .map(|x| WireGuardConnection::new(x.get_id().into(), *x.get_is_active()))
            .collect();
        conn
    }

    /// Get the internal mock state of connections that are marked as active
    pub fn get_active_connections(&self) -> Vec<WireGuardConnection> {
        let conn: Vec<WireGuardConnection> = self
            .available_connections
            .borrow()
            .iter()
            .filter(|x| x.get_is_active() == &true)
            .map(|x| WireGuardConnection::new(x.get_id().into(), *x.get_is_active()))
            .collect();
        conn
    }

    /// Internally mark the connection as activated/deactivated if it exists in the mock state
    pub fn set_internal_conn_state(&self, id: &str, is_active: bool) {
        let mut mutb = self.available_connections.borrow_mut();
        let idx = mutb.iter().position(|x| x.get_id() == id);
        if let Some(idx) = idx {
            mutb[idx] = WireGuardConnection::new(id.into(), is_active);
        }
    }

    /// Internally add a new connection to the mock state
    pub fn add_connection(&self, conn: WireGuardConnection) {
        self.available_connections.borrow_mut().push(conn);
    }
}

/// Custom helper to initialize the mock repository with test data that is explicitly working
/// for testing different scenarios
impl WireGuardNmRepoMock {
    /// Helper to initialize the mock with a list of non active connections
    pub fn init_with_no_active_connection() -> Self {
        WireGuardNmRepoMock::new(vec![
            WireGuardConnection::new("some-id-1".into(), false),
            WireGuardConnection::new("some-id-2".into(), false),
            WireGuardConnection::new("some-id-3".into(), false),
            WireGuardConnection::new("some-id-4".into(), false),
        ])
    }

    /// Helper to initialize the mock with a list of non active connections explicitly not ordered
    pub fn init_with_no_active_connection_unordered() -> Self {
        WireGuardNmRepoMock::new(vec![
            WireGuardConnection::new("some-id-2".into(), false),
            WireGuardConnection::new("some-id-4".into(), false),
            WireGuardConnection::new("some-id-3".into(), false),
            WireGuardConnection::new("some-id-1".into(), false),
        ])
    }

    /// Helper to initialize the mock with a list of non active connections, but one active
    pub fn init_with_one_active_connection() -> Self {
        WireGuardNmRepoMock::new(vec![
            WireGuardConnection::new("some-id-1".into(), false),
            WireGuardConnection::new("some-id-2".into(), false),
            WireGuardConnection::new("some-id-3".into(), false),
            WireGuardConnection::new("some-id-4".into(), true),
        ])
    }

    /// Helper to initialize the mock with a list of non active connections, but
    /// multiple active ones
    pub fn init_with_multiple_active_connection() -> Self {
        WireGuardNmRepoMock::new(vec![
            WireGuardConnection::new("some-id-1".into(), false),
            WireGuardConnection::new("some-id-2".into(), false),
            WireGuardConnection::new("some-id-3".into(), true),
            WireGuardConnection::new("some-id-4".into(), true),
        ])
    }
}

/// Actual implementation of the trait for the WireGuardNmRepoMock
impl WireGuardPort for WireGuardNmRepoMock {
    fn get_imported_connections(
        &self,
    ) -> Result<Vec<WireGuardConnection>, AdapterGetConnectionsError> {
        if let Some(expected_error) = self.get_imported_connections_error {
            return Err(expected_error);
        }
        let conns: Vec<WireGuardConnection> = self.get_all_connections();
        Ok(conns)
    }

    fn activate_connection(&self, id: &str) -> Result<(), AdapterConnectionActivationError> {
        self.set_internal_conn_state(id, true);
        Ok(())
    }

    fn deactivate_connection(&self, id: &str) -> Result<(), AdapterConnectionDeactivationError> {
        self.set_internal_conn_state(id, false);
        Ok(())
    }

    fn import_from_file(
        &self,
        config_file_path: PathBuf,
    ) -> Result<String, ports::outbound::wireguard_port::ConnectionImportError> {
        let id = config_file_path
            .iter()
            .next_back()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();
        self.add_connection(WireGuardConnection::new(id.to_string(), true));
        Ok(id.to_string())
    }
}
