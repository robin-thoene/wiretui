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
                                AdapterConnectionDeactivationError::Infrastructure(_inner) => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                                AdapterConnectionDeactivationError::ActiveConnectionsRetrieval => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                                AdapterConnectionDeactivationError::ConnectionNotFound(_inner) => {
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
            None => Err(ConnectionDeactivationError::NotFound),
        }
    }
}

#[cfg(test)]
mod deactivate_connection_usecase_tests {
    use super::*;
    use crate::testing::*;

    /// Ensures that a currently active connection can be deactivated
    #[test]
    fn success_deactivate_active_connection() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::init_with_one_active_connection();
        let id_to_deactivate = "some-id-4";
        assert!(repo_mock.get_all_connections().iter().len() > 1);
        assert_eq!(repo_mock.get_active_connections().len(), 1);
        // Act
        let result = DeactivateConnectionUsecase::new(&repo_mock)
            .deactivate(&WireGuardConnection::new(id_to_deactivate.into(), true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(repo_mock.get_active_connections().iter().len(), 0);
    }

    /// Ensures that a currently active connection can be deactivated when multiple ones are
    /// currently active
    #[test]
    fn success_deactivate_one_of_mulitple_active_connections() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::init_with_multiple_active_connection();
        let active_conn_count = repo_mock.get_active_connections().len();
        let id_to_deactivate = "some-id-4";
        assert!(repo_mock.get_all_connections().iter().len() > 1);
        assert!(active_conn_count > 1);
        // Act
        let result = DeactivateConnectionUsecase::new(&repo_mock)
            .deactivate(&WireGuardConnection::new(id_to_deactivate.into(), true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(
            repo_mock.get_active_connections().iter().len(),
            active_conn_count - 1
        );
        // Ensure that the previously active connection is not in the list of current active
        // connections anymore
        assert_eq!(
            repo_mock
                .get_active_connections()
                .iter()
                .filter(|x| x.get_id() == id_to_deactivate)
                .collect::<Vec<_>>()
                .len(),
            0
        );
    }

    /// Ensure that the deactivation fails with the expected error if the connection to deactivate
    /// does not exist
    #[test]
    fn error_connection_to_deactivate_does_not_exist() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::init_with_one_active_connection();
        let active_conn_count = repo_mock.get_active_connections().len();
        let id_to_deactivate = "some-id-does-not-exist";
        assert!(repo_mock.get_all_connections().iter().len() > 1);
        assert_eq!(active_conn_count, 1);
        // Act
        let result = DeactivateConnectionUsecase::new(&repo_mock)
            .deactivate(&WireGuardConnection::new(id_to_deactivate.into(), true));
        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("expecting an error"),
            ConnectionDeactivationError::NotFound
        );
        assert_eq!(
            repo_mock.get_active_connections().iter().len(),
            active_conn_count
        );
    }

    /// Ensure that the deactivation fails with the expected error if the connection to deactivate
    /// is not currently active
    #[test]
    fn error_connection_to_deactivate_is_not_active() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::init_with_one_active_connection();
        let active_conn_count = repo_mock.get_active_connections().len();
        let id_to_deactivate = "some-id-1";
        assert!(repo_mock.get_all_connections().iter().len() > 1);
        assert_eq!(active_conn_count, 1);
        // Act
        let result = DeactivateConnectionUsecase::new(&repo_mock)
            .deactivate(&WireGuardConnection::new(id_to_deactivate.into(), true));
        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("expecting an error"),
            ConnectionDeactivationError::NotActive
        );
        assert_eq!(
            repo_mock.get_active_connections().iter().len(),
            active_conn_count
        );
    }
}
