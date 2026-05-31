use domain::models::WireGuardConnection;
use std::{
    error::Error,
    fmt::{self},
    path::Path,
};

/// Must be implemented by adapters handling WireGuard
pub trait WireGuardPort {
    /// Retrieve all already imported and available WireGuard connections
    ///
    /// # Errors
    ///
    /// If the imported connections can not be retrieved, the underlying error is returned as
    /// custom one. This happens mostly because of a failure connecting or interacting with the
    /// used infrastructure to access and interact with network connections.
    fn get_imported_connections(&self) -> Result<Vec<WireGuardConnection>, GetConnectionsError>;

    /// Activate a single connection
    ///
    /// # Arguments
    ///
    /// * `id` - The connection identifier
    ///
    /// # Errors
    ///
    /// If the connection for the provided identifier can not be activated, the underlying error
    /// is returned as custom one.
    ///
    /// The two main reasons are:
    /// - infrastructure failure while attempting to activate the connection
    /// - logical error (for example connection with id does not exist or is already active)
    fn activate_connection(&self, id: &str) -> Result<(), ConnectionActivationError>;

    /// Deactivate a single connection
    ///
    /// # Arguments
    ///
    /// * `id` - The connection identifier
    ///
    /// # Errors
    ///
    /// If the connection for the provided identifier can not be deactivated, the underlying error
    /// is returned as custom one.
    ///
    /// The two main reasons are:
    /// - infrastructure failure while attempting to deactivate the connection
    /// - logical error (for example connection with id does not exist)
    fn deactivate_connection(&self, id: &str) -> Result<(), ConnectionDeactivationError>;

    /// Import a single connection from a config file
    ///
    /// # Arguments
    ///
    /// * `config_file_path` - The path to the config file
    ///
    /// # Errors
    ///
    /// A new connection can not be created from the provided config file, a custom error is
    /// returned.
    fn import_from_file(&self, config_file_path: &Path) -> Result<String, ConnectionImportError>;

    /// Remove a single imported connection
    ///
    /// # Arguments
    ///
    /// * `id` - The connection identifier
    ///
    /// # Errors
    ///
    /// The connection can not be removed, a custom error is returned.
    fn remove_connection(&self, id: &str) -> Result<(), ConnectionRemovalError>;
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionRemovalError {
    /// Error while connecting to the infrastructure that is used to remove connections
    Infrastructure(InfrastructureError),
    /// Error while retrieving the imported connections
    ImportedConnectionsRetrieval,
    /// The connection to remove does not exist
    ConnectionNotFound(NotFoundError),
}
impl Error for ConnectionRemovalError {}
impl fmt::Display for ConnectionRemovalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionRemovalError::Infrastructure(inner) => {
                write!(f, "could not remove the connection: {}", inner)
            }
            ConnectionRemovalError::ImportedConnectionsRetrieval => {
                write!(f, "currently imported connections could not be retrieved")
            }
            ConnectionRemovalError::ConnectionNotFound(inner) => {
                write!(f, "the connection to remove could not be found: {}", inner)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionImportError {
    /// Error while connecting to the infrastructure that is used to import connections
    Infrastructure(InfrastructureError),
    /// The file to import the connection from can not be found
    FileNotFound(NotFoundError),
    /// The unique identifier for the new connection could not be determined
    CouldNotResolveConnectionId,
    /// Error while trying to modify the imported connection
    CouldNotModify,
}
impl Error for ConnectionImportError {}
impl fmt::Display for ConnectionImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionImportError::Infrastructure(inner) => {
                write!(f, "could not import the new connection: {}", inner)
            }
            ConnectionImportError::FileNotFound(inner) => {
                write!(
                    f,
                    "could not import new connection from config file: {}",
                    inner
                )
            }
            ConnectionImportError::CouldNotResolveConnectionId => {
                write!(
                    f,
                    "could not determine the identifier for the new connection"
                )
            }
            ConnectionImportError::CouldNotModify => {
                write!(f, "the imported connection could not be modified")
            }
        }
    }
}

/// Error that occurs when trying to connect with the infrastructure used to manage the network
/// connections
#[derive(Debug, Clone, Copy)]
pub struct InfrastructureError;
impl Error for InfrastructureError {}
impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "error while interacting with the network managing infrastructure"
        )
    }
}

/// Error that occurs when trying to access a connection that can not be found
#[derive(Debug, Clone, Copy)]
pub struct NotFoundError;
impl Error for NotFoundError {}
impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the resource could not be found")
    }
}

/// Error types that could occur when trying to receive all imported connections
#[derive(Debug, Clone, Copy)]
pub enum GetConnectionsError {
    /// Error while connecting to the infrastructure that is used to manage connections
    Infrastructure(InfrastructureError),
}
impl Error for GetConnectionsError {}
impl fmt::Display for GetConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            GetConnectionsError::Infrastructure(inner) => {
                write!(f, "could not retrieve imported connections: {}", inner)
            }
        }
    }
}

/// Error types that could occur when trying to activate a connection
#[derive(Debug)]
pub enum ConnectionActivationError {
    /// Error while connecting to the infrastructure that is used to manage connections
    Infrastructure(InfrastructureError),
    /// Error attempting to activate a connection that is already active
    ConnectionAlreadyActive,
    /// Error while retrieving the imported connections
    ImportedConnectionsRetrieval,
    /// Connection to activate could not be found
    ConnectionNotFound(NotFoundError),
    /// Activating the connection failed for reasons on the infra level
    CouldNotActivate,
}
impl Error for ConnectionActivationError {}
impl fmt::Display for ConnectionActivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ConnectionActivationError::Infrastructure(inner) => {
                write!(f, "could not activate connection: {}", inner)
            }
            ConnectionActivationError::ConnectionAlreadyActive => {
                write!(f, "targeted connection is already active")
            }
            ConnectionActivationError::ImportedConnectionsRetrieval => {
                write!(f, "currently imported connections could not be retrieved")
            }
            ConnectionActivationError::ConnectionNotFound(inner) => {
                write!(f, "could not activate connection: {}", inner)
            }
            ConnectionActivationError::CouldNotActivate => {
                write!(f, "could not activate the connection")
            }
        }
    }
}

/// Error types that could occur when trying to deactivate a connection
#[derive(Debug, Clone, Copy)]
pub enum ConnectionDeactivationError {
    /// Error while connecting to the infrastructure that is used to manage connections
    Infrastructure(InfrastructureError),
    /// Error while retrieving the currently active connections
    ActiveConnectionsRetrieval,
    /// Connection to deactivate could not be found
    ConnectionNotFound(NotFoundError),
    /// Deactivating the connection failed for reasons on the infra level
    CouldNotDeactivate,
}
impl Error for ConnectionDeactivationError {}
impl fmt::Display for ConnectionDeactivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ConnectionDeactivationError::Infrastructure(inner) => {
                write!(f, "could not deactivate connection: {}", inner)
            }
            ConnectionDeactivationError::ActiveConnectionsRetrieval => {
                write!(f, "currently active connections could not be retrieved")
            }
            ConnectionDeactivationError::ConnectionNotFound(inner) => {
                write!(f, "could not deactivate connection: {}", inner)
            }
            ConnectionDeactivationError::CouldNotDeactivate => {
                write!(f, "could not deactivate connection")
            }
        }
    }
}
