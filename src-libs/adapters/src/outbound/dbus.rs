use serde::Deserialize;
use std::collections::HashMap;
use zbus::{
    Result, proxy,
    zvariant::{OwnedObjectPath, OwnedValue, Type},
};

#[doc = "Used to automatically generated access to NetworkManager settings using zbus"]
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings",
    interface = "org.freedesktop.NetworkManager.Settings"
)]
pub trait NetworkManagerSettings {
    fn list_connections(&self) -> Result<Vec<OwnedObjectPath>>;
}

#[doc = "Used to automatically generated access to a NetworkManager connection using zbus"]
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings.Connection"
)]
pub trait NetworkManagerConnection {
    fn get_settings(&self) -> Result<HashMap<String, HashMap<String, OwnedValue>>>;
}

/// Custom model for a single NetworkManager connection
#[derive(Deserialize, Type, Debug)]
pub struct NetworkManagerConnection {
    #[serde(rename = "type")]
    pub t: String,
    pub id: String,
}

/// Potential error types that can occur when trying to parse a NetworkManager connection from
/// D-Bus
pub enum NetworkManagerConnectionParseError {
    /// No connection found in input
    Connection,
    /// Could not find 'type' in input
    Type,
    /// Could not find 'id' in input
    Id,
}

impl TryFrom<&HashMap<String, HashMap<String, OwnedValue>>> for NetworkManagerConnection {
    type Error = NetworkManagerConnectionParseError;

    fn try_from(
        value: &HashMap<String, HashMap<String, OwnedValue>>,
    ) -> std::result::Result<Self, Self::Error> {
        let conn = value
            .get("connection")
            .ok_or(NetworkManagerConnectionParseError::Connection)?;
        let t = conn
            .get("type")
            .ok_or(NetworkManagerConnectionParseError::Type)?
            .downcast_ref()
            .map_err(|_| NetworkManagerConnectionParseError::Type)?;
        let id = conn
            .get("id")
            .ok_or(NetworkManagerConnectionParseError::Id)?
            .downcast_ref()
            .map_err(|_| NetworkManagerConnectionParseError::Id)?;
        Ok(Self { t, id })
    }
}
