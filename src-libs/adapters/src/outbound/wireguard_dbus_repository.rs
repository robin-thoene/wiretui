use crate::outbound::dbus_repository::{
    NetworkConnection, NetworkManagerActiveConnectionProxyBlocking,
    NetworkManagerConnectionProxyBlocking, NetworkManagerProxyBlocking,
    NetworkManagerSettingsProxyBlocking,
};
use domain::models::WireGuardConnection;
use ports::outbound::wireguard_port::WireGuardPort;
use zbus::{blocking::Connection, zvariant::OwnedObjectPath};

/// Internal representation of a single WireGuard connection
struct InternalWireGuardConnection {
    id: String,
    path: OwnedObjectPath,
    is_active: bool,
}

impl InternalWireGuardConnection {
    /// Create a new internal WireGuard connection representation
    ///
    /// # Arguments
    ///
    /// * `id` - The connection identifier
    /// * `path` - The path on the D-Bus
    /// * `is_active` - Whether the connection is currently active or not
    fn new(id: String, path: OwnedObjectPath, is_active: bool) -> Self {
        Self {
            id,
            path,
            is_active,
        }
    }
}

/// Repository that handles WireGuard using D-Bus
pub struct WireGuardDBusRepository {
    /// The system connection to the D-Bus on the operating system
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

    /// Internal function to get the imported D-Bus WireGuard connections
    fn get_imported_connections_internal(&self) -> zbus::Result<Vec<InternalWireGuardConnection>> {
        let settings_proxy = NetworkManagerSettingsProxyBlocking::new(&self.dbus_connection)?;
        let nm_proxy = NetworkManagerProxyBlocking::new(&self.dbus_connection)?;
        let connections = settings_proxy.list_connections()?;
        let mut res = vec![];
        let active_connection_ids: Vec<String> = nm_proxy
            .active_connections()?
            .iter()
            .map(|x| {
                // TODO: this is ugly, refactor
                let proxy =
                    NetworkManagerActiveConnectionProxyBlocking::new(&self.dbus_connection, x);
                if let Ok(proxy) = proxy {
                    let id = proxy.id();
                    if let Ok(id) = id {
                        id
                    } else {
                        "default".to_string()
                    }
                } else {
                    "default".to_string()
                }
            })
            .collect();
        for connection in connections {
            let connection_proxy =
                NetworkManagerConnectionProxyBlocking::new(&self.dbus_connection, &connection)?;
            let settings = connection_proxy.get_settings()?;
            let conn = NetworkConnection::try_from(&settings);
            if let Ok(conn) = conn
                && conn.conn_type == "wireguard"
            {
                let is_active = active_connection_ids.contains(&conn.id);
                res.push(InternalWireGuardConnection::new(
                    conn.id,
                    connection.clone(),
                    is_active,
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
            res.push(WireGuardConnection::new(
                connection.id.to_string(),
                connection.is_active,
            ));
        }
        Ok(res)
    }

    fn activate_connection(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connections = self.get_imported_connections_internal()?;
        let conn = connections.iter().find(|x| x.id == id);
        if let Some(conn) = conn {
            if conn.is_active {
                let msg = "Can not activate a connection that is already active";
                log::warn!("{}", msg);
                return Err(msg.into());
            }
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

    fn deactivate_connection(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let nm_proxy = NetworkManagerProxyBlocking::new(&self.dbus_connection)?;
        let active_connections = nm_proxy.active_connections()?;
        // TODO: this is ugly, refactor
        let mut active_connection_path: Option<OwnedObjectPath> = None;
        for ac in active_connections {
            let proxy =
                NetworkManagerActiveConnectionProxyBlocking::new(&self.dbus_connection, ac.clone());
            if let Ok(proxy) = proxy {
                let a_id = proxy.id();
                if let Ok(a_id) = a_id
                    && a_id == id
                {
                    active_connection_path = Some(ac);
                }
            }
        }
        if let Some(conn_path) = active_connection_path {
            let result = nm_proxy.deactivate_connection(&conn_path);
            match result {
                Ok(_ok) => Ok(()),
                Err(_err) => Err("Could not deactivate connection".into()),
            }
        } else {
            Err("Could not find active connection for provided id".into())
        }
    }
}
