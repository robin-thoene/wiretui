use domain::models::WireGuardConnection;
use ports::{
    inbound::activate_connection_port::{ActivateConnectionPort, ConnectionActivationError},
    outbound::wireguard_port::{
        ConnectionActivationError as AdapterConnectionActivationError, WireGuardPort,
    },
};

/// Use case for activating an available connection
pub struct ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> ActivateConnectionPort for ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), ConnectionActivationError> {
        let connections = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ConnectionActivationError::Infra)?;
        let conn = connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        match conn {
            Some(conn) => match conn.get_is_active() {
                true => {
                    log::warn!("{}", "connection is already active");
                    Err(ConnectionActivationError::AlreadyActive)
                }
                false => {
                    // Ensure that previously active connections are deactivated
                    for conn in connections.iter().filter(|x| *x.get_is_active()) {
                        let deactivate_res =
                            self.wireguard_port.deactivate_connection(conn.get_id());
                        if let Err(_err) = deactivate_res {
                            return Err(ConnectionActivationError::Infra);
                        }
                    }
                    // Activate the connection
                    let activation_result = self.wireguard_port.activate_connection(conn.get_id());
                    match activation_result {
                        Ok(()) => Ok(()),
                        Err(err) => {
                            log::error!(
                                "error occurred while attempting to activate connection {}: {}",
                                conn.get_id(),
                                err
                            );
                            match err {
                                AdapterConnectionActivationError::Infrastructure(_i) => {
                                    Err(ConnectionActivationError::Infra)
                                }
                                AdapterConnectionActivationError::ConnectionAlreadyActive => {
                                    Err(ConnectionActivationError::AlreadyActive)
                                }
                                AdapterConnectionActivationError::ImportedConnectionsRetrieval => {
                                    Err(ConnectionActivationError::Infra)
                                }
                                AdapterConnectionActivationError::ConnectionNotFound(_i) => {
                                    Err(ConnectionActivationError::NotFound)
                                }
                                AdapterConnectionActivationError::CouldNotActivate => {
                                    Err(ConnectionActivationError::Infra)
                                }
                            }
                        }
                    }
                }
            },
            None => Err(ConnectionActivationError::NotFound),
        }
    }
}
