use domain::models::WireGuardConnection;

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
    /// If the provided connection can not be deactivated, the underlying error is
    /// returned as custom one.
    ///
    /// The main reasons are:
    /// - infrastructure failure while attempting to interact with the connection manager
    /// - logical errors, like trying to deactivate a connection that is not active at the moment
    fn deactivate(
        &self,
        connection: &WireGuardConnection,
    ) -> Result<(), ConnectionDeactivationError>;
}

pub enum ConnectionDeactivationError {
    Infra,
    NotActive,
    NotFound,
}
