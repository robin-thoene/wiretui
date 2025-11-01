use crate::outbound::dbus_repository::{
    NetworkConnection, NetworkManagerConnectionProxyBlocking, NetworkManagerProxyBlocking,
    NetworkManagerSettingsProxyBlocking,
};
use domain::models::WireGuardConnection;
use ports::outbound::wireguard_port::WireGuardPort;
use zbus::{blocking::Connection, zvariant::OwnedObjectPath};

/// Repository internal representation of a single WireGuard connection
struct InternalWireGuardConnection {
    id: String,
    path: OwnedObjectPath,
}

impl InternalWireGuardConnection {
    /// Create a new internal WireGuard connection representation
    ///
    /// # Arguments
    ///
    /// * `id` - The connection identifier
    /// * `path` - The path on the D-Bus
    fn new(id: String, path: OwnedObjectPath) -> Self {
        Self { id, path }
    }

    /// Get the unique identifier
    fn get_id(&self) -> &str {
        &self.id
    }
}

/// Repository that handles WireGuard using D-Bus
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
    /// use adapters::outbound::wireguard_dbus_repository::WireGuardDBusRepository;
    ///
    /// let _res = WireGuardDBusRepository::new();
    /// ```
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dbus_connection = Connection::system()?;
        Ok(Self { dbus_connection })
    }

    /// Repository internal method to get the imported D-Bus WireGuard connections
    fn get_imported_connections_internal(&self) -> zbus::Result<Vec<InternalWireGuardConnection>> {
        let settings_proxy = NetworkManagerSettingsProxyBlocking::new(&self.dbus_connection)?;
        let connections = settings_proxy.list_connections()?;
        let mut res = vec![];
        for connection in connections {
            let connection_proxy =
                NetworkManagerConnectionProxyBlocking::new(&self.dbus_connection, &connection)?;
            let settings = connection_proxy.get_settings()?;
            let conn = NetworkConnection::try_from(&settings);
            if let Ok(conn) = conn
                && conn.conn_type == "wireguard"
            {
                res.push(InternalWireGuardConnection::new(
                    conn.id,
                    connection.clone(),
                ));
            }
        }
        Ok(res)
    }
}

impl WireGuardPort for WireGuardDBusRepository {
    fn get_imported_connections(
        &self,
    ) -> Result<Vec<WireGuardConnection>, Box<dyn std::error::Error>> {
        let internal_connections = self.get_imported_connections_internal()?;
        let mut res = vec![];
        for connection in internal_connections {
            res.push(WireGuardConnection::new(connection.get_id().to_string()));
        }
        Ok(res)
    }

    fn activate_connection(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connections = self.get_imported_connections_internal()?;
        let conn = connections.iter().find(|x| x.get_id() == id);
        if let Some(conn) = conn {
            let r_path =
                OwnedObjectPath::try_from("/").expect("Expect the root objectpath to be created");
            let nm_proxy = NetworkManagerProxyBlocking::new(&self.dbus_connection)?;
            let result = nm_proxy.activate_connection(&conn.path, &r_path, &r_path);
            match result {
                Ok(_ok) => Ok(()),
                Err(_err) => Err("Could not activate connection".into()),
            }
        } else {
            Err("could not find connection".into())
        }
    }
}
