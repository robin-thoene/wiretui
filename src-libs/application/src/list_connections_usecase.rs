use domain::models::WireGuardConnection;
use ports::{
    inbound::list_connections_port::ListConnectionsPort, outbound::wireguard_port::WireGuardPort,
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
    pub fn new(wireguard_dbus_repository: &'a W) -> Self {
        Self {
            wireguard_port: wireguard_dbus_repository,
        }
    }
}

impl<'a, W> ListConnectionsPort for ListConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    fn get(&self) -> Vec<WireGuardConnection> {
        // TODO: error handling
        let mut res = self
            .wireguard_port
            .get_imported_connections()
            .unwrap_or_default();
        res.sort_by(|a, b| a.get_id().cmp(b.get_id()));
        res
    }
}

#[cfg(test)]
mod list_connections_usecase_tests {
    use super::*;
    use std::error::Error;

    /// Ensures that the use case returns an empty vec if the WireGuardPort returns an empty one
    /// as well
    #[test]
    fn returns_expected_empty_vec() {
        // Arrange
        #[derive(Default)]
        struct WireGuardDbusRepoMock {}
        impl WireGuardPort for WireGuardDbusRepoMock {
            fn get_imported_connections(&self) -> Result<Vec<WireGuardConnection>, Box<dyn Error>> {
                Ok(vec![])
            }

            fn activate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
            }
        }
        // Act
        let result = ListConnectionsUseCase::new(&WireGuardDbusRepoMock::default()).get();
        // Assert
        assert_eq!(result, Vec::<WireGuardConnection>::new());
        assert_eq!(result.len(), 0);
    }

    /// Ensures that an error on the WireGuardPort results in a fallback to an empty vec
    #[test]
    fn fallback_to_empty_vec() {
        // Arrange
        #[derive(Default)]
        struct WireGuardDbusRepoMock {}
        impl WireGuardPort for WireGuardDbusRepoMock {
            fn get_imported_connections(&self) -> Result<Vec<WireGuardConnection>, Box<dyn Error>> {
                Err("some error".into())
            }

            fn activate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
            }
        }
        // Act
        let result = ListConnectionsUseCase::new(&WireGuardDbusRepoMock::default()).get();
        // Assert
        assert_eq!(result, Vec::<WireGuardConnection>::new());
        assert_eq!(result.len(), 0);
    }

    /// Ensures that the items that are retrieved from the WireGuardPort are returned correctly
    #[test]
    fn returns_expected_items() {
        // Arrange
        #[derive(Default)]
        struct WireGuardDbusRepoMock {}
        impl WireGuardPort for WireGuardDbusRepoMock {
            fn get_imported_connections(&self) -> Result<Vec<WireGuardConnection>, Box<dyn Error>> {
                Ok(vec![
                    WireGuardConnection::new("some-id-0".to_string(), false),
                    WireGuardConnection::new("some-id-1".to_string(), false),
                    WireGuardConnection::new("some-id-2".to_string(), false),
                    WireGuardConnection::new("some-id-3".to_string(), false),
                    WireGuardConnection::new("some-id-4".to_string(), false),
                    WireGuardConnection::new("some-id-5".to_string(), false),
                ])
            }

            fn activate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
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
        let result = ListConnectionsUseCase::new(&WireGuardDbusRepoMock::default()).get();
        // Assert
        assert_eq!(result, expected_data);
        assert_eq!(result.len(), expected_data.len());
    }

    /// Ensures that the items that are retrieved from the WireGuardPort are sorted by id before
    /// being returned
    #[test]
    fn returns_items_sorted() {
        // Arrange
        #[derive(Default)]
        struct WireGuardDbusRepoMock {}
        impl WireGuardPort for WireGuardDbusRepoMock {
            fn get_imported_connections(&self) -> Result<Vec<WireGuardConnection>, Box<dyn Error>> {
                Ok(vec![
                    WireGuardConnection::new("some-id-0".to_string(), false),
                    WireGuardConnection::new("some-id-3".to_string(), false),
                    WireGuardConnection::new("some-id-1".to_string(), false),
                    WireGuardConnection::new("some-id-5".to_string(), false),
                    WireGuardConnection::new("some-id-4".to_string(), false),
                    WireGuardConnection::new("some-id-2".to_string(), false),
                ])
            }

            fn activate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
            }

            fn deactivate_connection(&self, _id: &str) -> Result<(), Box<dyn Error>> {
                Ok(())
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
        let result = ListConnectionsUseCase::new(&WireGuardDbusRepoMock::default()).get();
        // Assert
        assert_eq!(result, expected_data);
        assert_eq!(result.len(), expected_data.len());
    }
}
