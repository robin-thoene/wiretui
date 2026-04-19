use domain::models::WireGuardConnection;
use ports::{
    inbound::activate_connection_port::{ActivateConnectionPort, ConnectionActivationError},
    outbound::wireguard_port::{
        ConnectionActivationError as AdapterConnectionActivationError, WireGuardPort,
    },
};

/// Use case for activating an available connection
pub struct ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> ActivateConnectionPort for ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), ConnectionActivationError> {
        let connections = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ConnectionActivationError::Infra)?;
        let conn = connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        match conn {
            Some(conn) => match conn.get_is_active() {
                true => {
                    log::warn!("{}", "connection is already active");
                    Err(ConnectionActivationError::AlreadyActive)
                }
                false => {
                    // Ensure that previously active connections are deactivated
                    for conn in connections.iter().filter(|x| *x.get_is_active()) {
                        let deactivate_res =
                            self.wireguard_port.deactivate_connection(conn.get_id());
                        if let Err(_err) = deactivate_res {
                            return Err(ConnectionActivationError::Infra);
                        }
                    }
                    // Activate the connection
                    let activation_result = self.wireguard_port.activate_connection(conn.get_id());
                    match activation_result {
                        Ok(()) => Ok(()),
                        Err(err) => {
                            log::error!(
                                "error occurred while attempting to activate connection {}: {}",
                                conn.get_id(),
                                err
                            );
                            match err {
                                AdapterConnectionActivationError::Infrastructure(_i) => {
                                    Err(ConnectionActivationError::Infra)
                                }
                                AdapterConnectionActivationError::ConnectionAlreadyActive => {
                                    Err(ConnectionActivationError::AlreadyActive)
                                }
                                AdapterConnectionActivationError::ImportedConnectionsRetrieval => {
                                    Err(ConnectionActivationError::Infra)
                                }
                                AdapterConnectionActivationError::ConnectionNotFound(_i) => {
                                    Err(ConnectionActivationError::NotFound)
                                }
                                AdapterConnectionActivationError::CouldNotActivate => {
                                    Err(ConnectionActivationError::Infra)
                                }
                            }
                        }
                    }
                }
            },
            None => Err(ConnectionActivationError::NotFound),
        }
    }
}

#[cfg(test)]
mod activate_connection_usecase_tests {
    use super::*;
    use ports::outbound::wireguard_port::{
        ConnectionActivationError as AdapterConnectionActivationError,
        ConnectionDeactivationError as AdapterConnectionDeactivationError, GetConnectionsError,
    };
    use std::cell::RefCell;

    struct WireGuardDbusRepoMock {
        available_connections: RefCell<Vec<WireGuardConnection>>,
    }

    impl WireGuardDbusRepoMock {
        /// Initialize the mock implementation with a single non-active connection
        pub fn init_single_conn() -> Self {
            Self {
                available_connections: RefCell::new(vec![WireGuardConnection::new(
                    "some-id".into(),
                    false,
                )]),
            }
        }

        /// Initialize the mock implementation with a bunch of non-active connections
        pub fn init_conn_list() -> Self {
            Self {
                available_connections: RefCell::new(vec![
                    WireGuardConnection::new("some-id-1".into(), false),
                    WireGuardConnection::new("some-id-2".into(), false),
                    WireGuardConnection::new("some-id-3".into(), false),
                    WireGuardConnection::new("some-id-4".into(), false),
                ]),
            }
        }

        /// Initialize the mock implementation with a bunch of non-active connections
        /// and one that is currently marked as active
        pub fn init_conn_list_with_active() -> Self {
            Self {
                available_connections: RefCell::new(vec![
                    WireGuardConnection::new("some-id-1".into(), false),
                    WireGuardConnection::new("some-id-2".into(), false),
                    WireGuardConnection::new("some-id-3".into(), false),
                    WireGuardConnection::new("some-id-4".into(), true),
                ]),
            }
        }

        /// Initialize the mock implementation with a bunch of non-active connections
        /// and multiple that are currently marked as active
        pub fn init_conn_list_with_multi_active() -> Self {
            Self {
                available_connections: RefCell::new(vec![
                    WireGuardConnection::new("some-id-1".into(), false),
                    WireGuardConnection::new("some-id-2".into(), false),
                    WireGuardConnection::new("some-id-3".into(), true),
                    WireGuardConnection::new("some-id-4".into(), true),
                ]),
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
    }
    impl WireGuardPort for WireGuardDbusRepoMock {
        fn get_imported_connections(
            &self,
        ) -> Result<Vec<WireGuardConnection>, GetConnectionsError> {
            let conns: Vec<WireGuardConnection> = self.get_all_connections();
            Ok(conns)
        }

        fn activate_connection(&self, id: &str) -> Result<(), AdapterConnectionActivationError> {
            self.set_internal_conn_state(id, true);
            Ok(())
        }

        fn deactivate_connection(
            &self,
            id: &str,
        ) -> Result<(), AdapterConnectionDeactivationError> {
            self.set_internal_conn_state(id, false);
            Ok(())
        }
    }

    /// Ensures that activating a single imported connection that not yet is active works
    #[test]
    fn success_single_available_connection() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_single_conn();
        let id_to_activate = "some-id";
        // Act
        let result = ActivateConnectionUsecase::new(&dbus_repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(dbus_repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_id(),
            id_to_activate
        );
    }

    /// Ensures that activating a single imported connection that not yet is active works for
    /// an available list of not yet activated connections
    #[test]
    fn success_multiple_available_connection() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_conn_list();
        let id_to_activate = "some-id-4";
        // Act
        let result = ActivateConnectionUsecase::new(&dbus_repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(dbus_repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_id(),
            id_to_activate
        );
    }

    /// Ensures that activating one connection automatically deactivates another that is currently
    /// active
    #[test]
    fn success_currently_active_connection() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_conn_list_with_active();
        let id_to_activate = "some-id-2";
        // Act
        let result = ActivateConnectionUsecase::new(&dbus_repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(dbus_repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_id(),
            id_to_activate
        );
    }

    /// Ensures that activating one connection automatically deactivates all others that are
    /// currently active
    #[test]
    fn success_currently_multiple_active_connections() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_conn_list_with_multi_active();
        let id_to_activate = "some-id-2";
        // Act
        let result = ActivateConnectionUsecase::new(&dbus_repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(dbus_repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_id(),
            id_to_activate
        );
    }

    /// Ensures that activating one connection that is already active results in the expected error
    #[test]
    fn error_connection_already_active() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_conn_list_with_active();
        let id_to_activate = "some-id-4";
        // Act
        let result = ActivateConnectionUsecase::new(&dbus_repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("expect to be an error"),
            ConnectionActivationError::AlreadyActive
        );
        // The connections was already active and is expected to stay active even after this
        // error
        assert_eq!(
            dbus_repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_id(),
            id_to_activate
        );
    }

    /// Ensures that trying to activate a connection that does not exist results in the expected
    /// error
    #[test]
    fn error_connection_to_activate_does_not_exist() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_conn_list();
        let id_to_activate = "some-id-does-not-exist";
        // Act
        let result = ActivateConnectionUsecase::new(&dbus_repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("expect to be an error"),
            ConnectionActivationError::NotFound
        );
    }
}
