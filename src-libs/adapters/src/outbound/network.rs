use dbus::{
    arg::{RefArg, Variant},
    blocking::Connection as DConnection,
};
use domain::models::Connection;
use ports::outbound::network::NetworkService;
use std::{collections::HashMap, time::Duration};

#[derive(Default)]
pub struct NetworkServiceImpl {}

impl NetworkService for NetworkServiceImpl {
    fn get_imported_vpn_connections(&self) -> Result<Vec<Connection>, Box<dyn std::error::Error>> {
        let conn = DConnection::new_system()?;
        let proxy = conn.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager/Settings",
            Duration::from_millis(5000),
        );
        let (conns,): (Vec<dbus::Path<'static>>,) = proxy.method_call(
            "org.freedesktop.NetworkManager.Settings",
            "ListConnections",
            (),
        )?;

        // TODO: simplify
        let mut res = vec![];
        for path in conns {
            let conn_p = conn.with_proxy(
                "org.freedesktop.NetworkManager",
                &path,
                Duration::from_millis(5000),
            );
            type NMSettings = HashMap<String, HashMap<String, Variant<Box<dyn RefArg>>>>;
            let (settings,): (NMSettings,) = conn_p.method_call(
                "org.freedesktop.NetworkManager.Settings.Connection",
                "GetSettings",
                (),
            )?;
            let name = settings
                .get("connection")
                .filter(|c| c.get("type").and_then(|t| t.0.as_str()) == Some("wireguard"))
                .and_then(|c| c.get("id").and_then(|i| i.0.as_str()));
            if let Some(id) = name {
                res.push(Connection::new(id.to_string()));
            }
        }
        Ok(res)
    }
}
