use domain::models::WireGuardConnection;
use ports::{
    inbound::deactivate_connection_port::DeactivateConnectionPort,
    outbound::wireguard_port::WireGuardPort,
};
use std::error::Error;

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
    fn deactivate(&self, connection: &WireGuardConnection) -> Result<(), Box<dyn Error>> {
        match connection.get_is_active() {
            false => Err("connection is not active".into()),
            true => {
                let deactivation_result = self
                    .wireguard_port
                    .deactivate_connection(connection.get_id());
                match deactivation_result {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        log::error!(
                            "Error occurred while attempting to deactivate connection {}: {}",
                            connection.get_id(),
                            err
                        );
                        Err(err)
                    }
                }
            }
        }
    }
}
