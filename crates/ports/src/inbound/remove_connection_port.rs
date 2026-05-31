use domain::models::WireGuardConnection;
use std::{error::Error, fmt};

/// Defines the inbound port to remove an existing connection
pub trait RemoveConnectionPort {
    /// Remove an imported WireGuard connection
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection to remove
    ///
    /// # Errors
    ///
    /// The provided connection may not be able to be removed, most likely because
    /// - an error on the underlying infrastructure
    /// - the connection does not exist
    fn remove(&self, connection: &WireGuardConnection) -> Result<(), ConnectionRemovalError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionRemovalError {
    Infra,
    NotFound,
}

impl Error for ConnectionRemovalError {}
impl fmt::Display for ConnectionRemovalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionRemovalError::Infra => write!(f, "an error occurred"),
            ConnectionRemovalError::NotFound => {
                write!(f, "the connection to remove does not exist")
            }
        }
    }
}
