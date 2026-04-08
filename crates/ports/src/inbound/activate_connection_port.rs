use domain::models::WireGuardConnection;

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
    /// If the provided connection can not be activated, the underlying error is
    /// returned as custom one.
    ///
    /// The main reasons are:
    /// - infrastructure failure while attempting to interact with the connection manager
    /// - logical errors, like trying to activate an already active connection
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), ConnectionActivationError>;
}

pub enum ConnectionActivationError {
    Infra,
    AlreadyActive,
    ConnectionNotFound,
}
