use domain::models::WireGuardConnection;
use ports::{
    inbound::deactivate_connection_port::{ConnectionDeactivationError, DeactivateConnectionPort},
    outbound::wireguard_port::{
        ConnectionDeactivationError as AdapterConnectionDeactivationError, WireGuardPort,
    },
};

/// Use case for deactivating an available connection
pub struct DeactivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> DeactivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> DeactivateConnectionPort for DeactivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    fn deactivate(
        &self,
        connection: &WireGuardConnection,
    ) -> Result<(), ConnectionDeactivationError> {
        let connections = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ConnectionDeactivationError::Infra)?;
        let conn = connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        match conn {
            Some(conn) => match conn.get_is_active() {
                false => Err(ConnectionDeactivationError::NotActive),
                true => {
                    let deactivation_result =
                        self.wireguard_port.deactivate_connection(conn.get_id());
                    match deactivation_result {
                        Ok(()) => Ok(()),
                        Err(err) => {
                            log::error!(
                                "error occurred while attempting to deactivate connection {}: {}",
                                conn.get_id(),
                                err
                            );
                            match err {
                                AdapterConnectionDeactivationError::Infrastructure(_i) => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                                AdapterConnectionDeactivationError::ActiveConnectionsRetrieval => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                                AdapterConnectionDeactivationError::NotFound(_n) => {
                                    Err(ConnectionDeactivationError::NotFound)
                                }
                                AdapterConnectionDeactivationError::CouldNotDeactivate => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                            }
                        }
                    }
                }
            },
            None => Err(ConnectionDeactivationError::Infra),
        }
    }
}

#[cfg(test)]
mod deactivate_connection_usecase_tests {
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

    /// Ensures that a currently active connection can be deactivated
    #[test]
    fn success_deactivate_active_connection() {
        // Arrange
        let dbus_repo_mock = WireGuardDbusRepoMock::init_conn_list_with_active();
        let id_to_deactivate = "some-id-4";
        // Act
        let result = DeactivateConnectionUsecase::new(&dbus_repo_mock)
            .deactivate(&WireGuardConnection::new(id_to_deactivate.into(), true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(dbus_repo_mock.get_active_connections().iter().len(), 0);
    }
}
