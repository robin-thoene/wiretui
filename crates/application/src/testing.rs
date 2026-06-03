use domain::models::WireGuardConnection;
use ports::outbound::wireguard_port::{
    ConnectionActivationError as AdapterConnectionActivationError,
    ConnectionDeactivationError as AdapterConnectionDeactivationError,
    ConnectionImportError as AdapterConnectionConnectionImportError,
    ConnectionImportError as AdapterConnectionImportError,
    ConnectionRemovalError as AdapterConnectionRemovalError,
    GetConnectionsError as AdapterGetConnectionsError, WireGuardPort,
};
use std::{cell::RefCell, path::Path};

/// Mock for the WireGuardNmRepo
#[derive(Default)]
pub struct WireGuardNmRepoMock {
    /// Internal state of mocked imported WireGuard connections to use for testing
    available_connections: RefCell<Vec<WireGuardConnection>>,
    /// The error type the mock shall return when the fn to list all connections is invoked
    get_imported_connections_error: Option<AdapterGetConnectionsError>,
    /// The error type the mock shall return when the fn to deactivate a connection is invoked
    deactivate_connection_error: Option<AdapterConnectionDeactivationError>,
    /// The error type the mock shall return when the fn to import a connection is invoked
    import_connection_error: Option<AdapterConnectionConnectionImportError>,
    /// The error type the mock shall return when the fn to remove a connection is invoked
    remove_connection_error: Option<AdapterConnectionRemovalError>,
}

/// Mock implementation of the WireGuardNmRepo
impl WireGuardNmRepoMock {
    /// Create a new mock instance with a given initial state of available connections
    pub fn new(initial_connections: Vec<WireGuardConnection>) -> Self {
        Self {
            available_connections: RefCell::new(initial_connections),
            get_imported_connections_error: None,
            deactivate_connection_error: None,
            import_connection_error: None,
            remove_connection_error: None,
        }
    }

    /// Create a new mock instance with a given initial state of available connections and
    /// expected errors that shall be returned as part of the mock
    pub fn new_with_error(
        initial_connections: Vec<WireGuardConnection>,
        get_imported_connections_error: Option<AdapterGetConnectionsError>,
        deactivate_connection_error: Option<AdapterConnectionDeactivationError>,
        import_connection_error: Option<AdapterConnectionConnectionImportError>,
        remove_connection_error: Option<AdapterConnectionRemovalError>,
    ) -> Self {
        Self {
            available_connections: RefCell::new(initial_connections),
            get_imported_connections_error,
            deactivate_connection_error,
            import_connection_error,
            remove_connection_error,
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
            .filter(|x| *x.get_is_active())
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
    pub fn add_connection_internal(&self, conn: WireGuardConnection) {
        self.available_connections.borrow_mut().push(conn);
    }

    /// Internally remove a connection to the mock state
    pub fn remove_connection_internal(&self, id: &str) {
        let idx = self
            .available_connections
            .borrow_mut()
            .iter()
            .position(|x| x.get_id() == id);
        if let Some(idx) = idx {
            self.available_connections.borrow_mut().remove(idx);
        }
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
        if let Some(expected_error) = self.deactivate_connection_error {
            return Err(expected_error);
        }
        self.set_internal_conn_state(id, false);
        Ok(())
    }

    fn import_from_file(
        &self,
        config_file_path: &Path,
    ) -> Result<String, AdapterConnectionImportError> {
        if let Some(expected_error) = self.import_connection_error {
            return Err(expected_error);
        }
        let id = config_file_path
            .iter()
            .next_back()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();
        self.add_connection_internal(WireGuardConnection::new(id.to_string(), true));
        Ok(id.to_string())
    }

    fn remove_connection(&self, id: &str) -> Result<(), AdapterConnectionRemovalError> {
        if let Some(err) = self.remove_connection_error {
            return Err(err);
        }
        self.remove_connection_internal(id);
        Ok(())
    }
}
