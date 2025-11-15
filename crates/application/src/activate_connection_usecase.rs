use domain::models::WireGuardConnection;
use ports::{
    inbound::activate_connection_port::ActivateConnectionPort,
    outbound::wireguard_port::WireGuardPort,
};
use std::error::Error;

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
    fn activate(&self, connection: &WireGuardConnection) -> Result<(), Box<dyn Error>> {
        match connection.get_is_active() {
            true => {
                let msg = "connection is already active";
                log::warn!("{}", msg);
                Err(msg.into())
            }
            false => {
                let activation_result =
                    self.wireguard_port.activate_connection(connection.get_id());
                match activation_result {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        log::error!(
                            "error occurred while attempting to activate connection {}: {}",
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
