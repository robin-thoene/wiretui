use crate::outbound::dbus_repository::{
    NetworkConnection, NetworkManagerConnectionProxyBlocking, NetworkManagerSettingsProxyBlocking,
};
use domain::models::WireGuardConnection;
use ports::outbound::wireguard_port::WireGuardPort;
use zbus::blocking::Connection;

/// Implementation that handles WireGuard using D-Bus
pub struct WireGuardDBusRepository {
    dbus_connection: Connection,
}

impl WireGuardDBusRepository {
    /// Creates a new instance
    ///
    /// # Errors
    ///
    /// The connection to the D-Bus might fail
    ///
    /// # Examples
    ///
    /// ```
    /// use adapters::outbound::wireguard_dbus::WireGuardDBusImpl;
    ///
    /// let _res = WireGuardDBusImpl::new();
    /// ```
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dbus_connection = Connection::system()?;
        Ok(Self { dbus_connection })
    }
}

impl WireGuardPort for WireGuardDBusRepository {
    fn get_imported_connections(
        &self,
    ) -> Result<Vec<WireGuardConnection>, Box<dyn std::error::Error>> {
        let settings_proxy = NetworkManagerSettingsProxyBlocking::new(&self.dbus_connection)?;
        let connections = settings_proxy.list_connections()?;
        let mut res = vec![];
        for connection in connections {
            let connection_proxy =
                NetworkManagerConnectionProxyBlocking::new(&self.dbus_connection, connection)?;
            let settings = connection_proxy.get_settings()?;
            let conn = NetworkConnection::try_from(&settings);
            if let Ok(conn) = conn
                && conn.conn_type == "wireguard"
            {
                res.push(WireGuardConnection::new(conn.id));
            }
        }
        Ok(res)
    }
}
