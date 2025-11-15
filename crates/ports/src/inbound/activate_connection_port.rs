use domain::models::WireGuardConnection;
use std::error::Error;

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
    /// TODO: describe error
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), Box<dyn Error>>;
}
