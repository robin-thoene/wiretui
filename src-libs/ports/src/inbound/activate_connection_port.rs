use domain::models::WireGuardConnection;
use std::error::Error;

/// Defines the inbound port to activate an available connections
pub trait ActivateConnectionPort {
    /// Activate an available WireGuard connections
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the connection to activate
    ///
    /// # Errors
    ///
    /// TODO: describe error
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), Box<dyn Error>>;
}
