use std::{error::Error, fmt, path::PathBuf};

/// Defines the inbound port to import new connections
pub trait ImportConnectionPort {
    /// Import a new WireGuard connection from a config file
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the config file on the Filesystem
    ///
    /// # Errors
    ///
    /// The provided configuration file can not be activated, most likely because
    /// - an error on the underlying infrastructure
    /// - the file does not exist
    /// - the file does not contain a valid WireGuard configuration
    fn import_from_file(&self, file_path: PathBuf) -> Result<(), ConnectionImportError>;
}

#[derive(Debug)]
pub enum ConnectionImportError {
    Infra,
    FileNotFound,
    InvalidConfig,
}
impl Error for ConnectionImportError {}
impl fmt::Display for ConnectionImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionImportError::Infra => write!(f, "an error occurred"),
            ConnectionImportError::FileNotFound => write!(f, "the config file does not exist"),
            ConnectionImportError::InvalidConfig => write!(f, "the config file is not valid"),
        }
    }
}
