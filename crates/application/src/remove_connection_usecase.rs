use domain::models::WireGuardConnection;
use ports::{
    inbound::remove_connection_port::{ConnectionRemovalError, RemoveConnectionPort},
    outbound::wireguard_port::{
        ConnectionDeactivationError as AdapterConnectionDeactivationError,
        ConnectionRemovalError as AdapterConnectionRemovalError, WireGuardPort,
    },
};

/// Use case for removing a single imported connection
pub struct RemoveConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    wireguard_port: &'a W,
}

impl<'a, W> RemoveConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self { wireguard_port }
    }
}

impl<'a, W> RemoveConnectionPort for RemoveConnectionsUseCase<'a, W>
where
    W: WireGuardPort,
{
    fn remove(&self, connection: &WireGuardConnection) -> Result<(), ConnectionRemovalError> {
        let connections = self
            .wireguard_port
            .get_imported_connections()
            .map_err(|_e| ConnectionRemovalError::Infra)?;
        let conn = connections
            .iter()
            .find(|x| x.get_id() == connection.get_id());
        match conn {
            Some(conn) => {
                if *conn.get_is_active() {
                    // If the connection is currently active, deactivate it first before attempting
                    // to remove it
                    let deactivation_result =
                        self.wireguard_port.deactivate_connection(conn.get_id());
                    if let Err(err) = deactivation_result {
                        log::error!(
                            "an error occurred while deactivation connection {}: {}",
                            conn.get_id(),
                            err
                        );
                        return match err {
                            AdapterConnectionDeactivationError::Infrastructure(_inner) => {
                                Err(ConnectionRemovalError::Infra)
                            }
                            AdapterConnectionDeactivationError::ActiveConnectionsRetrieval => {
                                Err(ConnectionRemovalError::Infra)
                            }
                            AdapterConnectionDeactivationError::NotFound(_inner) => {
                                Err(ConnectionRemovalError::NotFound)
                            }
                            AdapterConnectionDeactivationError::CouldNotDeactivate => {
                                Err(ConnectionRemovalError::Infra)
                            }
                        };
                    }
                }
                let result = self.wireguard_port.remove_connection(conn.get_id());
                match result {
                    Ok(_) => Ok(()),
                    Err(err) => match err {
                        AdapterConnectionRemovalError::Infrastructure(inner) => {
                            log::error!(
                                "error removing the connection {}: {}",
                                conn.get_id(),
                                inner
                            );
                            Err(ConnectionRemovalError::Infra)
                        }
                        AdapterConnectionRemovalError::ImportedConnectionsRetrieval => {
                            Err(ConnectionRemovalError::Infra)
                        }
                        AdapterConnectionRemovalError::ConnectionNotFound(_inner) => {
                            Err(ConnectionRemovalError::NotFound)
                        }
                    },
                }
            }
            None => Err(ConnectionRemovalError::NotFound),
        }
    }
}

#[cfg(test)]
mod remove_connection_usecase_tests {}
