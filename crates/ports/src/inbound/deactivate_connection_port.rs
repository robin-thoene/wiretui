use domain::models::WireGuardConnection;
use std::{error::Error, fmt};

/// Defines the inbound port to deactivate an available connection
pub trait DeactivateConnectionPort {
    /// Deactivate an available WireGuard connection
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection to deactivate
    ///
    /// # Errors
    ///
    /// If the provided connection can not be deactivated, the underlying error is
    /// returned as custom one.
    ///
    /// The main reasons are:
    /// - infrastructure failure while attempting to interact with the connection manager
    /// - logical errors, like trying to deactivate a connection that is not active at the moment
    fn deactivate(
        &self,
        connection: &WireGuardConnection,
    ) -> Result<(), ConnectionDeactivationError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionDeactivationError {
    Infra,
    NotActive,
    NotFound,
}
impl Error for ConnectionDeactivationError {}
impl fmt::Display for ConnectionDeactivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionDeactivationError::Infra => write!(f, "an error occurred"),
            ConnectionDeactivationError::NotActive => {
                write!(f, "the connection is not active at the moment")
            }
            ConnectionDeactivationError::NotFound => {
                write!(f, "the connection to deactivate could not be found")
            }
        }
    }
}
