use domain::models::WireGuardConnection;

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
