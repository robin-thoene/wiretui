use domain::models::WireGuardConnection;

/// Defines the inbound port to activate an available connections
pub trait ActivateConnectionPort {
    /// Activate an available WireGuard connections
    fn activate(&self, connection: &WireGuardConnection);
}
