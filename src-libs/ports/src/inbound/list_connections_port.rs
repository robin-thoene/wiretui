use domain::models::WireGuardConnection;

/// Defines the inbound port to list all available connections
pub trait ListConnectionsPort {
    /// Retrieves all available WireGuard connections
    fn get(&self) -> Vec<WireGuardConnection>;
}
