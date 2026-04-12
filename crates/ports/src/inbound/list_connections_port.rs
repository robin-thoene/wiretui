use domain::models::WireGuardConnection;
use std::{error::Error, fmt};

/// Defines the inbound port to list all available connections
///
/// # Errors
///
/// The connections can not be retrieved from the underlying connection manager, most likely to an
/// error outside of the control of this program
pub trait ListConnectionsPort {
    /// Retrieves all available WireGuard connections
    fn get(&self) -> Result<Vec<WireGuardConnection>, ListConnectionError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListConnectionError {
    Infra,
}
impl Error for ListConnectionError {}
impl fmt::Display for ListConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListConnectionError::Infra => write!(f, "an error occurred"),
        }
    }
}
