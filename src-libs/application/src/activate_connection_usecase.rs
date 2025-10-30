use ports::{
    inbound::activate_connection_port::ActivateConnectionPort,
    outbound::wireguard_port::WireGuardPort,
};

/// Use case for activating an available connections
pub struct ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    _wireguard_port: &'a W,
}

impl<'a, W> ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_port: &'a W) -> Self {
        Self {
            _wireguard_port: wireguard_port,
        }
    }
}

impl<'a, W> ActivateConnectionPort for ActivateConnectionUsecase<'a, W>
where
    W: WireGuardPort,
{
    fn activate(&self, _connection: &domain::models::WireGuardConnection) {
        todo!()
    }
}
