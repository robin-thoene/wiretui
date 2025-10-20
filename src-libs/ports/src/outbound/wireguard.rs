use domain::models::WireGuardConnection;

/// Must be implemented by services handling WireGuard
pub trait WireGuardService {
    /// Retrieves all already imported and available WireGuard connections
    fn get_imported_connections(
        &self,
    ) -> Result<Vec<WireGuardConnection>, Box<dyn std::error::Error>>;
}
