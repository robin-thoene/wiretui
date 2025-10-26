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

/// Should only be used to try to parse the "connection" from the network manager connection
/// settings into a corresponding model
impl From<&HashMap<String, OwnedValue>> for NetworkManagerConnection {
    fn from(value: &HashMap<String, OwnedValue>) -> Self {
        Self {
            t: value
                .get("type")
                .expect("'type' is expected to exist")
                .downcast_ref()
                .expect("'type' is expected to be casted into String"),
            id: value
                .get("id")
                .expect("'id' is expected to exist")
                .downcast_ref()
                .expect("'id' is expected to be casted into String"),
        }
    }
}
