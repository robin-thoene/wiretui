use ports::{
    inbound::import_connection_port::{ConnectionImportError, ImportConnectionPort},
    outbound::wireguard_port::{
        ConnectionImportError as AdapterConnectionImportError, WireGuardPort,
    },
};
use std::path::PathBuf;

/// Use case for importing new WireGuard connections
pub struct ImportConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> ImportConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> ImportConnectionPort for ImportConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    fn import_from_file(&self, config_file_path: &str) -> Result<(), ConnectionImportError> {
        let config_file_path = PathBuf::from(config_file_path);
        let not_found_msg = "could not find the config file to import at given path";
        if !config_file_path.exists() {
            log::error!("{}", not_found_msg);
            return Err(ConnectionImportError::FileNotFound);
        }
        // TODO: validate the content of the conf file to verify it is a wireguard config
        let id = self
            .wireguard_port
            .import_from_file(&config_file_path)
            .map_err(|err| match err {
                AdapterConnectionImportError::Infrastructure(infrastructure_error) => {
                    log::error!(
                        "failed to import the connection {:?} with underlying error {}",
                        config_file_path,
                        infrastructure_error
                    );
                    ConnectionImportError::Infra
                }
                AdapterConnectionImportError::FileNotFound(_inner) => {
                    log::error!("{}", not_found_msg);
                    ConnectionImportError::FileNotFound
                }
                AdapterConnectionImportError::CouldNotResolveConnectionId => {
                    log::error!(
                        "config file {:?} is not a valid WireGuard config file",
                        config_file_path
                    );
                    ConnectionImportError::InvalidConfig
                }
                AdapterConnectionImportError::CouldNotModify => {
                    log::error!("failed to modify the imported connection");
                    ConnectionImportError::CouldNotModify
                }
            })?;
        let deactivation_result = self.wireguard_port.deactivate_connection(&id);
        match deactivation_result {
            Ok(_) => Ok(()),
            Err(_) => {
                log::error!("failed to deactivate the newly imported connection {}", id);
                Err(ConnectionImportError::CouldNotModify)
            }
        }
    }
}

#[cfg(test)]
mod import_connection_usecase_tests {
    use super::*;
    use crate::testing::*;
    use ports::inbound::import_connection_port::ConnectionImportError;
    use ports::outbound::wireguard_port::{
        ConnectionDeactivationError as AdapterConnectionDeactivationError,
        ConnectionImportError as AdapterConnectionImportError,
    };

    /// Test that ensures a valid WireGuard config file can be imported as connection successful
    #[test]
    fn success_import_connection_from_conf_file() {
        // Arrange
        let connection_id = "mock_valid_wg";
        let test_file_path = format!(
            "{}/tests/data/{}.conf",
            env!("CARGO_MANIFEST_DIR"),
            connection_id
        );
        let repo_mock = WireGuardNmRepoMock::default();
        assert_eq!(repo_mock.get_all_connections().len(), 0);
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file(&test_file_path);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let new_connection = all_connections.iter().find(|x| x.get_id() == connection_id);
        // Assert
        assert!(result.is_ok());
        assert!(new_connection.is_some());
        assert!(!new_connection.unwrap().get_is_active());
    }

    /// Test that ensures an import attempt fails with the expected error if the provided config
    /// file does not exist
    #[test]
    fn error_config_file_does_not_exist() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::default();
        assert_eq!(repo_mock.get_all_connections().len(), 0);
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file("non-existing-file-path/non-existing-file.conf");
        // Assert
        assert!(result.is_err_and(|err| err == ConnectionImportError::FileNotFound));
    }

    /// Test that ensures a valid WireGuard config file can be imported as connection into an
    /// already filled list of imported connections
    #[test]
    fn success_import_connection_from_conf_file_and_add_to_list() {
        // Arrange
        let connection_id = "mock_valid_wg";
        let test_file_path = format!(
            "{}/tests/data/{}.conf",
            env!("CARGO_MANIFEST_DIR"),
            connection_id
        );
        let repo_mock = WireGuardNmRepoMock::init_with_no_active_connection();
        let conn_count = repo_mock.get_all_connections().len();
        assert!(conn_count > 1);
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file(&test_file_path);
        let all_connections = repo_mock.get_imported_connections().unwrap();
        let new_connection = all_connections.iter().find(|x| x.get_id() == connection_id);
        // Assert
        assert!(result.is_ok());
        assert!(new_connection.is_some());
        assert!(!new_connection.unwrap().get_is_active());
        assert_eq!(all_connections.len(), conn_count + 1);
    }

    /// Test that ensures that a failure in deactivating the imported configuration results in the
    /// expected custom error
    #[test]
    fn failed_deactivation_results_in_expected_error() {
        // Arrange
        let connection_id = "mock_valid_wg";
        let test_file_path = format!(
            "{}/tests/data/{}.conf",
            env!("CARGO_MANIFEST_DIR"),
            connection_id
        );
        let repo_mock = WireGuardNmRepoMock::new_with_error(
            vec![],
            None,
            Some(AdapterConnectionDeactivationError::CouldNotDeactivate),
            None,
            None,
        );
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file(&test_file_path);
        // Assert
        assert!(result.is_err_and(|err| err == ConnectionImportError::CouldNotModify));
    }

    /// Test that ensures that a failure in the modification of the imported configuration results
    /// in the expected custom error
    #[test]
    fn failed_modification_results_in_expected_error() {
        // Arrange
        let connection_id = "mock_valid_wg";
        let test_file_path = format!(
            "{}/tests/data/{}.conf",
            env!("CARGO_MANIFEST_DIR"),
            connection_id
        );
        let repo_mock = WireGuardNmRepoMock::new_with_error(
            vec![],
            None,
            None,
            Some(AdapterConnectionImportError::CouldNotModify),
            None,
        );
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file(&test_file_path);
        // Assert
        assert!(result.is_err_and(|err| err == ConnectionImportError::CouldNotModify));
    }
}
