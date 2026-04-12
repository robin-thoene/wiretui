use domain::models::WireGuardConnection;
use ports::{
    inbound::deactivate_connection_port::{ConnectionDeactivationError, DeactivateConnectionPort},
    outbound::wireguard_port::{
        ConnectionDeactivationError as AdapterConnectionDeactivationError, WireGuardPort,
    },
};

/// Use case for deactivating an available connection
pub struct DeactivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> DeactivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> DeactivateConnectionPort for DeactivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    fn deactivate(
        &self,
        connection: &WireGuardConnection,
    ) -> Result<(), ConnectionDeactivationError> {
        let connections = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ConnectionDeactivationError::Infra)?;
        let conn = connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        match conn {
            Some(conn) => match conn.get_is_active() {
                false => Err(ConnectionDeactivationError::NotActive),
                true => {
                    let deactivation_result =
                        self.wireguard_port.deactivate_connection(conn.get_id());
                    match deactivation_result {
                        Ok(()) => Ok(()),
                        Err(err) => {
                            log::error!(
                                "error occurred while attempting to deactivate connection {}: {}",
                                conn.get_id(),
                                err
                            );
                            match err {
                                AdapterConnectionDeactivationError::Infrastructure(_i) => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                                AdapterConnectionDeactivationError::ActiveConnectionsRetrieval => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                                AdapterConnectionDeactivationError::NotFound(_n) => {
                                    Err(ConnectionDeactivationError::NotFound)
                                }
                                AdapterConnectionDeactivationError::CouldNotDeactivate => {
                                    Err(ConnectionDeactivationError::Infra)
                                }
                            }
                        }
                    }
                }
            },
            None => Err(ConnectionDeactivationError::Infra),
        }
    }
}
