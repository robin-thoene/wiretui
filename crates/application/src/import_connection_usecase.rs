use ports::{
    inbound::import_connection_port::{ConnectionImportError, ImportConnectionPort},
    outbound::wireguard_port::WireGuardPort,
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
        if !config_file_path.exists() {
            return Err(ConnectionImportError::FileNotFound);
        }
        self.wireguard_port
            .import_from_file(config_file_path)
            .map_err(|_err| ConnectionImportError::Infra)?;
        Ok(())
    }
}

#[cfg(test)]
mod import_connection_usecase_tests {
    use super::*;
    use domain::models::WireGuardConnection;
    use ports::outbound::wireguard_port::{
        ConnectionActivationError, ConnectionDeactivationError, GetConnectionsError,
    };
    use std::cell::RefCell;

    #[derive(Default)]
    struct WireGuardNmRepoMock {
        available_connections: RefCell<Vec<WireGuardConnection>>,
    }

    impl WireGuardNmRepoMock {
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

        /// Internally mark the connection as activated/deactivated if it exists in the mock state
        pub fn set_internal_conn_state(&self, id: &str, is_active: bool) {
            let mut mutb = self.available_connections.borrow_mut();
            let idx = mutb.iter().position(|x| x.get_id() == id);
            if let Some(idx) = idx {
                mutb[idx] = WireGuardConnection::new(id.into(), is_active);
            }
        }
    }
    impl WireGuardPort for WireGuardNmRepoMock {
        fn get_imported_connections(
            &self,
        ) -> Result<Vec<WireGuardConnection>, GetConnectionsError> {
            let conns: Vec<WireGuardConnection> = self.get_all_connections();
            Ok(conns)
        }

        fn activate_connection(&self, id: &str) -> Result<(), ConnectionActivationError> {
            self.set_internal_conn_state(id, true);
            Ok(())
        }

        fn deactivate_connection(&self, id: &str) -> Result<(), ConnectionDeactivationError> {
            self.set_internal_conn_state(id, false);
            Ok(())
        }

        fn import_from_file(
            &self,
            _file_path: PathBuf,
        ) -> Result<(), ports::outbound::wireguard_port::ConnectionImportError> {
            Ok(())
        }
    }

    /// Test that ensures a valid WireGuard config file can be imported as connection successful
    #[test]
    fn success_import_connection_from_conf_file() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::default();
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file("todo");
        // Assert
        assert!(result.is_ok())
    }

    /// Test that ensures an import attempt fails with the expected error if the provided config
    /// file does not exist
    #[test]
    fn error_config_file_does_not_exist() {
        // Arrange
        let repo_mock = WireGuardNmRepoMock::default();
        let use_case = ImportConnectionUsecase::new(&repo_mock);
        // Act
        let result = use_case.import_from_file("non-existing-file-path/non-existing-file.conf");
        // Assert
        assert!(result.is_err_and(|err| err == ConnectionImportError::FileNotFound));
    }
}
