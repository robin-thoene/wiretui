use domain::models::WireGuardConnection;
use ports::{
    inbound::remove_connection_port::{ConnectionRemovalError, RemoveConnectionPort},
    outbound::wireguard_port::{
        ConnectionDeactivationError as AdapterConnectionDeactivationError,
        ConnectionRemovalError as AdapterConnectionRemovalError, WireGuardPort,
    },
};

/// Use case for removing a single imported connection
pub struct RemoveConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> RemoveConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> RemoveConnectionPort for RemoveConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    fn remove(&self, connection: &WireGuardConnection) -> Result<(), ConnectionRemovalError> {
        let connections = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ConnectionRemovalError::Infra)?;
        let conn = connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        match conn {
            Some(conn) => {
                if *conn.get_is_active() {
                    // If the connection is currently active, deactivate it first before attempting
                    // to remove it
                    let deactivation_result =
                        self.wireguard_port.deactivate_connection(conn.get_id());
                    if let Err(err) = deactivation_result {
                        log::error!(
                            "an error occurred while deactivation connection {}: {}",
                            conn.get_id(),
                            err
                        );
                        return match err {
                            AdapterConnectionDeactivationError::Infrastructure(_inner) => {
                                Err(ConnectionRemovalError::Infra)
                            }
                            AdapterConnectionDeactivationError::ActiveConnectionsRetrieval => {
                                Err(ConnectionRemovalError::Infra)
                            }
                            AdapterConnectionDeactivationError::ConnectionNotFound(_inner) => {
                                Err(ConnectionRemovalError::NotFound)
                            }
                            AdapterConnectionDeactivationError::CouldNotDeactivate => {
                                Err(ConnectionRemovalError::Infra)
                            }
                        };
                    }
                }
                let result = self.wireguard_port.remove_connection(conn.get_id());
                match result {
                    Ok(_) => Ok(()),
                    Err(err) => match err {
                        AdapterConnectionRemovalError::Infrastructure(inner) => {
                            log::error!(
                                "error removing the connection {}: {}",
                                conn.get_id(),
                                inner
                            );
                            Err(ConnectionRemovalError::Infra)
                        }
                        AdapterConnectionRemovalError::ImportedConnectionsRetrieval => {
                            Err(ConnectionRemovalError::Infra)
                        }
                        AdapterConnectionRemovalError::ConnectionNotFound(_inner) => {
                            Err(ConnectionRemovalError::NotFound)
                        }
                    },
                }
            }
            None => Err(ConnectionRemovalError::NotFound),
        }
    }
}

#[cfg(test)]
mod remove_connection_usecase_tests {
    use super::*;
    use crate::testing::*;
    use ports::outbound::wireguard_port::{InfrastructureError, NotFoundError};

    /// Test that ensures that a connection can be removed and if it was the only one, no more
    /// connections remain in state
    #[test]
    fn success_remove_only_connection() {
        // Arrange
        let connection = WireGuardConnection::new("some-id".into(), false);
        let repo_mock =
            WireGuardNmRepoMock::new(vec![WireGuardConnection::new("some-id".into(), false)]);
        assert_eq!(repo_mock.get_all_connections().len(), 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let connection = all_connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        // Assert
        assert!(result.is_ok());
        assert!(connection.is_none());
        assert!(all_connections.is_empty());
    }

    /// Test that ensures that an active connection can be removed and if it was the only one, no
    /// more connections remain in state
    #[test]
    fn success_remove_only_connection_active() {
        // Arrange
        let connection = WireGuardConnection::new("some-id".into(), true);
        let repo_mock =
            WireGuardNmRepoMock::new(vec![WireGuardConnection::new("some-id".into(), true)]);
        assert_eq!(repo_mock.get_all_connections().len(), 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let connection = all_connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        // Assert
        assert!(result.is_ok());
        assert!(connection.is_none());
        assert!(all_connections.is_empty());
    }

    /// Test that ensures that a connection can be removed from a list of multiple imported ones
    #[test]
    fn success_remove_connection_from_list() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-4".into(), false);
        let repo_mock = WireGuardNmRepoMock::init_with_no_active_connection_unordered();
        let count = repo_mock.get_all_connections().len();
        assert!(count > 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let connection = all_connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        // Assert
        assert!(result.is_ok());
        assert!(connection.is_none());
        assert_eq!(all_connections.len(), count - 1);
    }

    /// Test that ensures that an active connection can be removed from a list of multiple
    /// imported ones
    #[test]
    fn success_remove_active_connection_from_list() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-4".into(), true);
        let repo_mock = WireGuardNmRepoMock::init_with_one_active_connection();
        let count = repo_mock.get_all_connections().len();
        assert!(count > 1);
        assert_eq!(repo_mock.get_active_connections().len(), 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let connection = all_connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        // Assert
        assert!(result.is_ok());
        assert!(connection.is_none());
        assert_eq!(all_connections.len(), count - 1);
        assert_eq!(repo_mock.get_active_connections().len(), 0);
    }

    /// Test that ensures that an active connection can be removed from a list of multiple
    /// imported ones, with multiple of them being active (for whatever reason)
    #[test]
    fn success_remove_active_connection_from_multi_active_list() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-4".into(), true);
        let repo_mock = WireGuardNmRepoMock::init_with_multiple_active_connection();
        let count = repo_mock.get_all_connections().len();
        let active_count = repo_mock.get_active_connections().len();
        assert!(count > 1);
        assert!(active_count > 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let connection = all_connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        // Assert
        assert!(result.is_ok());
        assert!(connection.is_none());
        assert_eq!(all_connections.len(), count - 1);
        assert_eq!(repo_mock.get_active_connections().len(), active_count - 1);
    }

    /// Test that ensures that an attempt to remove a non existing connection results in the
    /// expected `not found` error
    #[test]
    fn error_not_found() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-that-does-not-exist".into(), false);
        let repo_mock = WireGuardNmRepoMock::init_with_no_active_connection_unordered();
        let count = repo_mock.get_all_connections().len();
        assert!(count > 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        // Assert
        assert!(result.is_err_and(|x| x == ConnectionRemovalError::NotFound));
        assert_eq!(all_connections.len(), count);
    }

    /// Test that ensures that an attempt to remove a non existing connection results in the
    /// expected `not found` error if for whatever reason the logic layer does not catch it, but
    /// the repository implementation does
    #[test]
    fn error_not_found_in_repo() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-1".into(), false);
        let repo_mock = WireGuardNmRepoMock::new_with_error(
            vec![WireGuardConnection::new("some-id-1".into(), false)],
            None,
            None,
            None,
            Some(AdapterConnectionRemovalError::ConnectionNotFound(
                NotFoundError,
            )),
        );
        let count = repo_mock.get_all_connections().len();
        assert_eq!(count, 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        // Assert
        assert!(result.is_err_and(|x| x == ConnectionRemovalError::NotFound));
        assert_eq!(all_connections.len(), count);
    }

    /// Test that ensures that an attempt to remove a connection results in the expected error
    /// for underlying errors on getting the connection lists
    #[test]
    fn error_connection_retrieval_in_repo() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-1".into(), false);
        let repo_mock = WireGuardNmRepoMock::new_with_error(
            vec![WireGuardConnection::new("some-id-1".into(), false)],
            None,
            None,
            None,
            Some(AdapterConnectionRemovalError::ImportedConnectionsRetrieval),
        );
        let count = repo_mock.get_all_connections().len();
        assert_eq!(count, 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        // Assert
        assert!(result.is_err_and(|x| x == ConnectionRemovalError::Infra));
        assert_eq!(all_connections.len(), count);
    }

    /// Test that ensures that an attempt to remove a connection results in the expected error for
    /// underlying errors on infra
    #[test]
    fn error_infra_in_repo() {
        // Arrange
        let connection = WireGuardConnection::new("some-id-1".into(), false);
        let repo_mock = WireGuardNmRepoMock::new_with_error(
            vec![WireGuardConnection::new("some-id-1".into(), false)],
            None,
            None,
            None,
            Some(AdapterConnectionRemovalError::Infrastructure(
                InfrastructureError,
            )),
        );
        let count = repo_mock.get_all_connections().len();
        assert_eq!(count, 1);
        let use_case = RemoveConnectionsUseCase::new(&repo_mock);
        // Act
        let result = use_case.remove(&connection);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        // Assert
        assert!(result.is_err_and(|x| x == ConnectionRemovalError::Infra));
        assert_eq!(all_connections.len(), count);
    }
}
