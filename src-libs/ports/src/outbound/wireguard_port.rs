use domain::models::WireGuardConnection;
use std::error::Error;

/// Must be implemented by adapters handling WireGuard
pub trait WireGuardPort {
    /// Retrieves all already imported and available WireGuard connections
    fn get_imported_connections(&self) -> Result<Vec<WireGuardConnection>, Box<dyn Error>>;

    /// Activate a single connection
    fn activate_connection(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
