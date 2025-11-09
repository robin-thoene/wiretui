use serde::Deserialize;
use std::{collections::HashMap, result};
use zbus::{
    proxy,
    zvariant::{OwnedObjectPath, OwnedValue, Type},
};

#[doc = "Used to automatically generate access to the NetworkManager using zbus"]
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager",
    interface = "org.freedesktop.NetworkManager"
)]
pub trait NetworkManager {
    #[zbus(property)]
    fn active_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn activate_connection(
        &self,
        connection: &OwnedObjectPath,
        device: &OwnedObjectPath,
        specific_object: &OwnedObjectPath,
    ) -> zbus::Result<OwnedObjectPath>;

    fn deactivate_connection(
        &self,
        active_connection: &OwnedObjectPath,
    ) -> zbus::Result<OwnedObjectPath>;
}

#[doc = "Used to automatically generate access to NetworkManager settings using zbus"]
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings",
    interface = "org.freedesktop.NetworkManager.Settings"
)]
pub trait NetworkManagerSettings {
    fn list_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[doc = "Used to automatically generate access to a NetworkManager connection using zbus"]
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings.Connection"
)]
pub trait NetworkManagerConnection {
    fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, OwnedValue>>>;
}

#[doc = "Used to automatically generate access to a NetworkManager active connection using zbus"]
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Connection.Active"
)]
pub trait NetworkManagerActiveConnection {
    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;
}

/// Custom model for a single NetworkManager connection
#[derive(Deserialize, Type, Debug)]
pub struct NetworkConnection {
    /// The connection type
    #[serde(rename = "type")]
    pub conn_type: String,
    /// The identifier of the connection
    pub id: String,
}

/// Potential error types that can occur when trying to parse a NetworkManager connection from
/// D-Bus
#[derive(Debug, PartialEq, Eq)]
pub enum NetworkConnectionParseError {
    /// No connection found in input
    Connection,
    /// Could not find 'type' in input
    Type,
    /// Could not find 'id' in input
    Id,
}

/// Tries to parse a network manager configuration from a D-Bus response
impl TryFrom<&HashMap<String, HashMap<String, OwnedValue>>> for NetworkConnection {
    type Error = NetworkConnectionParseError;

    fn try_from(
        value: &HashMap<String, HashMap<String, OwnedValue>>,
    ) -> result::Result<Self, Self::Error> {
        let conn = value.get("connection").ok_or(Self::Error::Connection)?;
        let conn_type = conn
            .get("type")
            .ok_or(Self::Error::Type)?
            .downcast_ref()
            .map_err(|_| Self::Error::Type)?;
        let id = conn
            .get("id")
            .ok_or(Self::Error::Id)?
            .downcast_ref()
            .map_err(|_| Self::Error::Id)?;
        Ok(Self { conn_type, id })
    }
}

#[cfg(test)]
mod network_connection_tests {
    use super::*;
    use zbus::zvariant::Value;

    /// Validates that the D-Bus response can be parsed into the NetworkConnection model
    #[test]
    fn try_from_success() {
        // Arrange
        let templ = HashMap::<String, OwnedValue>::new();
        let mut inner_hm = templ.clone();
        let mut hm = HashMap::<String, HashMap<String, OwnedValue>>::new();
        inner_hm.insert(
            "type".to_string(),
            Value::from("wireguard").try_into_owned().unwrap(),
        );
        inner_hm.insert(
            "id".to_string(),
            Value::from("some_id").try_into_owned().unwrap(),
        );
        hm.insert("some_key".to_string(), templ.clone());
        hm.insert("connection".to_string(), inner_hm);
        hm.insert("some_other_key".to_string(), templ.clone());
        // Act
        let result = NetworkConnection::try_from(&hm);
        // Assert
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(result.id, "some_id".to_string());
            assert_eq!(result.conn_type, "wireguard".to_string());
        }
    }

    /// Validates that a missing 'connection' entry in the D-Bus response results in the expected
    /// error type
    #[test]
    fn returns_correct_error_missing_connection() {
        // Arrange
        let templ = HashMap::<String, OwnedValue>::new();
        let mut inner_hm = templ.clone();
        let mut hm = HashMap::<String, HashMap<String, OwnedValue>>::new();
        inner_hm.insert(
            "type".to_string(),
            Value::from("wireguard").try_into_owned().unwrap(),
        );
        inner_hm.insert(
            "id".to_string(),
            Value::from("some_id").try_into_owned().unwrap(),
        );
        hm.insert("some_key".to_string(), templ.clone());
        hm.insert("conne_wrong_ction".to_string(), inner_hm);
        hm.insert("some_other_key".to_string(), templ.clone());
        // Act
        let result = NetworkConnection::try_from(&hm);
        // Assert
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, NetworkConnectionParseError::Connection);
        }
    }

    /// Validates that a missing 'id' in the D-Bus response results in the expected error type
    #[test]
    fn returns_correct_error_missing_id_in_connection() {
        // Arrange
        let templ = HashMap::<String, OwnedValue>::new();
        let mut inner_hm = templ.clone();
        let mut hm = HashMap::<String, HashMap<String, OwnedValue>>::new();
        inner_hm.insert(
            "type".to_string(),
            Value::from("wireguard").try_into_owned().unwrap(),
        );
        inner_hm.insert(
            "i_wrong_d".to_string(),
            Value::from("some_id").try_into_owned().unwrap(),
        );
        hm.insert("some_key".to_string(), templ.clone());
        hm.insert("connection".to_string(), inner_hm);
        hm.insert("some_other_key".to_string(), templ.clone());
        // Act
        let result = NetworkConnection::try_from(&hm);
        // Assert
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, NetworkConnectionParseError::Id);
        }
    }

    /// Validates that a missing 'id' in the D-Bus response results in the expected error type
    #[test]
    fn returns_correct_error_missing_type_in_connection() {
        // Arrange
        let templ = HashMap::<String, OwnedValue>::new();
        let mut inner_hm = templ.clone();
        let mut hm = HashMap::<String, HashMap<String, OwnedValue>>::new();
        inner_hm.insert(
            "ty_wrong_pe".to_string(),
            Value::from("wireguard").try_into_owned().unwrap(),
        );
        inner_hm.insert(
            "id".to_string(),
            Value::from("some_id").try_into_owned().unwrap(),
        );
        hm.insert("some_key".to_string(), templ.clone());
        hm.insert("connection".to_string(), inner_hm);
        hm.insert("some_other_key".to_string(), templ.clone());
        // Act
        let result = NetworkConnection::try_from(&hm);
        // Assert
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, NetworkConnectionParseError::Type);
        }
    }
}
