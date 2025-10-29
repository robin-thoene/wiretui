use domain::models::WireGuardConnection;
use ports::{
    inbound::list_connections_port::ListConnectionsPort, outbound::wireguard_port::WireGuardPort,
};

/// Use case for retrieving all available connections
pub struct ListConnectionsUseCase<W>
where
    W: WireGuardPort,
{
    wireguard_port: W,
}

impl<W> ListConnectionsUseCase<W>
where
    W: WireGuardPort,
{
    pub fn new(wireguard_dbus_repository: W) -> Self {
        Self {
            wireguard_port: wireguard_dbus_repository,
        }
    }
}

impl<W> ListConnectionsPort for ListConnectionsUseCase<W>
where
    W: WireGuardPort,
{
    fn get(&self) -> Vec<WireGuardConnection> {
        // TODO: error handling
        self.wireguard_port
            .get_imported_connections()
            .unwrap_or_default()
    }
}
