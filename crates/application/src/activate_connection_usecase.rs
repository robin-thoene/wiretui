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
    use crate::testing::*;
    use domain::models::WireGuardConnection;

    /// Ensures that activating a single imported connection that not yet is active works
    #[test]
    fn success_single_available_connection() {
        // Arrange
        let repo_mock =
            WireGuardNmRepoMock::new(vec![WireGuardConnection::new("some-id".into(), false)]);
        let id_to_activate = "some-id";
        assert_eq!(repo_mock.get_all_connections().iter().len(), 1);
        assert_eq!(repo_mock.get_active_connections().iter().len(), 0);
        // Act
        let result = ActivateConnectionUsecase::new(&repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            repo_mock
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
        let repo_mock = WireGuardNmRepoMock::init_with_no_active_connection();
        let id_to_activate = "some-id-4";
        assert!(repo_mock.get_all_connections().iter().len() > 2);
        assert_eq!(repo_mock.get_active_connections().iter().len(), 0);
        // Act
        let result = ActivateConnectionUsecase::new(&repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            repo_mock
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
        let repo_mock = WireGuardNmRepoMock::init_with_one_active_connection();
        let id_to_activate = "some-id-2";
        assert!(repo_mock.get_all_connections().iter().len() > 2);
        assert_eq!(repo_mock.get_active_connections().iter().len(), 1);
        // Act
        let result = ActivateConnectionUsecase::new(&repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            repo_mock
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
        let repo_mock = WireGuardNmRepoMock::init_with_multiple_active_connection();
        let id_to_activate = "some-id-2";
        assert!(repo_mock.get_all_connections().iter().len() > 2);
        assert!(repo_mock.get_active_connections().iter().len() > 1);
        // Act
        let result = ActivateConnectionUsecase::new(&repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_ok());
        assert_eq!(repo_mock.get_active_connections().iter().len(), 1);
        assert!(
            repo_mock
                .get_active_connections()
                .first()
                .expect("a single active connection exists here")
                .get_is_active(),
        );
        assert_eq!(
            repo_mock
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
        let repo_mock = WireGuardNmRepoMock::init_with_one_active_connection();
        let id_to_activate = "some-id-4";
        assert!(repo_mock.get_all_connections().iter().len() > 2);
        // Act
        let result = ActivateConnectionUsecase::new(&repo_mock)
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
            repo_mock
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
        let repo_mock = WireGuardNmRepoMock::init_with_no_active_connection();
        let id_to_activate = "some-id-does-not-exist";
        assert!(repo_mock.get_all_connections().iter().len() > 2);
        // Act
        let result = ActivateConnectionUsecase::new(&repo_mock)
            .activate(&WireGuardConnection::new(id_to_activate.into(), false));
        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("expect to be an error"),
            ConnectionActivationError::NotFound
        );
    }
}
