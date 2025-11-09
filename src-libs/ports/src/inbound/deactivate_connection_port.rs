use domain::models::WireGuardConnection;
use std::error::Error;

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
    /// TODO: describe error
    fn deactivate(&self, connection: &WireGuardConnection) -> Result<(), Box<dyn Error>>;
}
