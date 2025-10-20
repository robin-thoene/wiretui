use dbus::{
    Path,
    arg::{RefArg, Variant},
    blocking::{Connection, Proxy},
};
use domain::models::WireGuardConnection;
use ports::outbound::wireguard::WireGuardService;
use std::{collections::HashMap, time::Duration};

/// NetworkManager D-Bus destination
const NM_DESTINATION: &str = "org.freedesktop.NetworkManager";
/// Path of the setting object within the NetworkManager D-Bus implementation
const NM_SETTINGS_PATH: &str = "/org/freedesktop/NetworkManager/Settings";
/// D-Bus settings interface on the NetworkManager settings object
const NM_SETTINGS_INTERFACE: &str = "org.freedesktop.NetworkManager.Settings";
/// Method to list all connections on the D-Bus settings interface on the NetworkManager settings
/// object
const NM_SETTINGS_INTERFACE_LS_CONN_METHOD: &str = "ListConnections";
/// D-Bus interface for a single settings connection on the NetworkManager settings connection
/// object
const NM_SETTINGS_CONNECTION_INTERFACE: &str = "org.freedesktop.NetworkManager.Settings.Connection";
/// Method to GET the connection settings on the D-Bus interface for a single settings connection
/// on the NetworkManager settings connection object
const NM_SETTINGS_CONNECTION_INTERFACE_GET_METHOD: &str = "GetSettings";

/// Implementation that handles WireGuard using D-Bus
pub struct WireGuardDBusImpl {
    /// The connection to the NetworkManager D-Bus destination
    nm_dbus_connection: Connection,
}

impl WireGuardDBusImpl {
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
        Ok(Self {
            nm_dbus_connection: Connection::new_system()?,
        })
    }

    /// Get a proxy for the NetworkManager D-Bus destination for a given path within it
    ///
    /// # Arguments
    ///
    /// * `path` - The desired object path
    fn get_nm_proxy<'a>(&'a self, path: &'a str) -> Proxy<'a, &'a Connection> {
        self.nm_dbus_connection
            .with_proxy(NM_DESTINATION, path, Duration::from_millis(5000))
    }

    /// Get the settings proxy in the NetworkManager D-Bus destination
    fn get_nm_settings_proxy(&self) -> Proxy<'_, &Connection> {
        self.get_nm_proxy(NM_SETTINGS_PATH)
    }
}

impl WireGuardService for WireGuardDBusImpl {
    fn get_imported_connections(
        &self,
    ) -> Result<Vec<WireGuardConnection>, Box<dyn std::error::Error>> {
        let proxy = self.get_nm_settings_proxy();
        let (conns,): (Vec<Path<'static>>,) = proxy.method_call(
            NM_SETTINGS_INTERFACE,
            NM_SETTINGS_INTERFACE_LS_CONN_METHOD,
            (),
        )?;

        // TODO: simplify
        let mut res = vec![];
        for path in conns {
            let conn_p = self.get_nm_proxy(&path);
            type NMSettings = HashMap<String, HashMap<String, Variant<Box<dyn RefArg>>>>;
            let (settings,): (NMSettings,) = conn_p.method_call(
                NM_SETTINGS_CONNECTION_INTERFACE,
                NM_SETTINGS_CONNECTION_INTERFACE_GET_METHOD,
                (),
            )?;
            let name = settings
                .get("connection")
                .filter(|c| c.get("type").and_then(|t| t.0.as_str()) == Some("wireguard"))
                .and_then(|c| c.get("id").and_then(|i| i.0.as_str()));
            if let Some(id) = name {
                res.push(WireGuardConnection::new(id.to_string()));
            }
        }
        Ok(res)
    }
}
