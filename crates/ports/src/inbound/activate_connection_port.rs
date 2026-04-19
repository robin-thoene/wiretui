use domain::models::WireGuardConnection;
use std::{error::Error, fmt};

/// Defines the inbound port to activate an available connection
pub trait ActivateConnectionPort {
    /// Activate an available WireGuard connection
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection to activate
    ///
    /// # Errors
    ///
    /// If the provided connection can not be activated, the underlying error is
    /// returned as custom one.
    ///
    /// The main reasons are:
    /// - infrastructure failure while attempting to interact with the connection manager
    /// - logical errors, like trying to activate an already active connection
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), ConnectionActivationError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionActivationError {
    Infra,
    AlreadyActive,
    NotFound,
}
impl Error for ConnectionActivationError {}
impl fmt::Display for ConnectionActivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionActivationError::Infra => {
                write!(f, "an error occurred")
            }
            ConnectionActivationError::AlreadyActive => {
                write!(f, "the connection is already active")
            }
            ConnectionActivationError::NotFound => write!(f, "the connection could not be found"),
        }
    }
}
