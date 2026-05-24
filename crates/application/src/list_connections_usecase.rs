use domain::models::WireGuardConnection;
use ports::{
    inbound::list_connections_port::{ListConnectionError, ListConnectionsPort},
    outbound::wireguard_port::WireGuardPort,
};

/// Use case for retrieving all available connections
pub struct ListConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> ListConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> ListConnectionsPort for ListConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    fn get(&self) -> Result<Vec<WireGuardConnection>, ListConnectionError> {
        let mut res = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ListConnectionError::Infra)?;
        res.sort_by(|a, b| a.get_id().cmp(b.get_id()));
        Ok(res)
    }
}

#[cfg(test)]
mod list_connections_usecase_tests {
    use super::*;
    use ports::outbound::wireguard_port::{
        ConnectionActivationError, ConnectionDeactivationError, GetConnectionsError,
        InfrastructureError,
    };
    use std::path::PathBuf;

    /// Ensures that the use case returns an empty vec if the WireGuardPort returns an empty one
    /// as well
    #[test]
    fn returns_expected_empty_vec() {
        // Arrange
        #[derive(Default)]
        struct WireGuardNmRepoMock {}
        impl WireGuardPort for WireGuardNmRepoMock {
            fn get_imported_connections(
                &self,
            ) -> Result<Vec<WireGuardConnection>, GetConnectionsError> {
                Ok(vec![])
            }

            fn activate_connection(&self, _id: &str) -> Result<(), ConnectionActivationError> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), ConnectionDeactivationError> {
                Ok(())
            }

            fn import_from_file(
                &self,
                _file_path: PathBuf,
            ) -> Result<String, ports::outbound::wireguard_port::ConnectionImportError>
            {
                Ok("".into())
            }
        }
        // Act
        let result = ListConnectionsUseCase::new(&WireGuardNmRepoMock::default()).get();
        // Assert
        assert!(result.is_ok());
        let data = result.expect("expecting no error");
        assert_eq!(data, Vec::<WireGuardConnection>::new());
        assert_eq!(data.len(), 0);
    }

    /// Ensures that an error on the WireGuardPort results in the correct custom error return type
    #[test]
    fn return_infra_error_correctly() {
        // Arrange
        #[derive(Default)]
        struct WireGuardNmRepoMock {}
        impl WireGuardPort for WireGuardNmRepoMock {
            fn get_imported_connections(
                &self,
            ) -> Result<Vec<WireGuardConnection>, GetConnectionsError> {
                Err(GetConnectionsError::Infrastructure(InfrastructureError))
            }

            fn activate_connection(&self, _id: &str) -> Result<(), ConnectionActivationError> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), ConnectionDeactivationError> {
                Ok(())
            }

            fn import_from_file(
                &self,
                _file_path: PathBuf,
            ) -> Result<String, ports::outbound::wireguard_port::ConnectionImportError>
            {
                Ok("".into())
            }
        }
        // Act
        let result = ListConnectionsUseCase::new(&WireGuardNmRepoMock::default()).get();
        // Assert
        assert!(result.is_err_and(|x| x == ListConnectionError::Infra));
    }

    /// Ensures that the items that are retrieved from the WireGuardPort are returned correctly
    #[test]
    fn returns_expected_items() {
        // Arrange
        #[derive(Default)]
        struct WireGuardNmRepoMock {}
        impl WireGuardPort for WireGuardNmRepoMock {
            fn get_imported_connections(
                &self,
            ) -> Result<Vec<WireGuardConnection>, GetConnectionsError> {
                Ok(vec![
                    WireGuardConnection::new("some-id-0".to_string(), false),
                    WireGuardConnection::new("some-id-1".to_string(), false),
                    WireGuardConnection::new("some-id-2".to_string(), false),
                    WireGuardConnection::new("some-id-3".to_string(), false),
                    WireGuardConnection::new("some-id-4".to_string(), false),
                    WireGuardConnection::new("some-id-5".to_string(), false),
                ])
            }

            fn activate_connection(&self, _id: &str) -> Result<(), ConnectionActivationError> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), ConnectionDeactivationError> {
                Ok(())
            }

            fn import_from_file(
                &self,
                _file_path: PathBuf,
            ) -> Result<String, ports::outbound::wireguard_port::ConnectionImportError>
            {
                Ok("".into())
            }
        }
        let expected_data = vec![
            WireGuardConnection::new("some-id-0".to_string(), false),
            WireGuardConnection::new("some-id-1".to_string(), false),
            WireGuardConnection::new("some-id-2".to_string(), false),
            WireGuardConnection::new("some-id-3".to_string(), false),
            WireGuardConnection::new("some-id-4".to_string(), false),
            WireGuardConnection::new("some-id-5".to_string(), false),
        ];
        // Act
        let result = ListConnectionsUseCase::new(&WireGuardNmRepoMock::default()).get();
        // Assert
        assert!(result.is_ok());
        let data = result.expect("expecting no error");
        assert_eq!(data, expected_data);
        assert_eq!(data.len(), expected_data.len());
    }

    /// Ensures that the items that are retrieved from the WireGuardPort are sorted by id before
    /// being returned
    #[test]
    fn returns_items_sorted() {
        // Arrange
        #[derive(Default)]
        struct WireGuardNmRepoMock {}
        impl WireGuardPort for WireGuardNmRepoMock {
            fn get_imported_connections(
                &self,
            ) -> Result<Vec<WireGuardConnection>, GetConnectionsError> {
                Ok(vec![
                    WireGuardConnection::new("some-id-0".to_string(), false),
                    WireGuardConnection::new("some-id-3".to_string(), false),
                    WireGuardConnection::new("some-id-1".to_string(), false),
                    WireGuardConnection::new("some-id-5".to_string(), false),
                    WireGuardConnection::new("some-id-4".to_string(), false),
                    WireGuardConnection::new("some-id-2".to_string(), false),
                ])
            }

            fn activate_connection(&self, _id: &str) -> Result<(), ConnectionActivationError> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), ConnectionDeactivationError> {
                Ok(())
            }

            fn import_from_file(
                &self,
                _file_path: PathBuf,
            ) -> Result<String, ports::outbound::wireguard_port::ConnectionImportError>
            {
                Ok("".into())
            }
        }
        let expected_data = vec![
            WireGuardConnection::new("some-id-0".to_string(), false),
            WireGuardConnection::new("some-id-1".to_string(), false),
            WireGuardConnection::new("some-id-2".to_string(), false),
            WireGuardConnection::new("some-id-3".to_string(), false),
            WireGuardConnection::new("some-id-4".to_string(), false),
            WireGuardConnection::new("some-id-5".to_string(), false),
        ];
        // Act
        let result = ListConnectionsUseCase::new(&WireGuardNmRepoMock::default()).get();
        // Assert
        assert!(result.is_ok());
        let data = result.expect("expecting no error");
        assert_eq!(data, expected_data);
        assert_eq!(data.len(), expected_data.len());
    }
}
