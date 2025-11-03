/// Represents a single VPN connection
#[derive(Debug, PartialEq)]
pub struct WireGuardConnection {
    /// The identifier of the connection
    id: String,
    /// Whether the connection is currently active or not
    is_active: bool,
}

impl WireGuardConnection {
    /// Creates a new connection
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier
    /// * `is_active` - Whether the connection is currently active or not
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::models::WireGuardConnection;
    ///
    /// let id = "unique-identifier";
    /// let connection = WireGuardConnection::new(id.to_string(), false);
    /// assert_eq!(connection.get_id(), id);
    /// assert_eq!(connection.get_is_active(), &false);
    /// ```
    pub fn new(id: String, is_active: bool) -> Self {
        Self { id, is_active }
    }

    /// Retrieves the internal store identifier of the connection
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::models::WireGuardConnection;
    ///
    /// let id = "unique-identifier";
    /// let connection = WireGuardConnection::new(id.to_string(), false);
    /// assert_eq!(connection.get_id(), id);
    /// ```
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Retrieves the internal state of the connection, i.e. Whether it is
    /// currently active or not
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::models::WireGuardConnection;
    ///
    /// let id = "unique-identifier";
    /// let connection = WireGuardConnection::new(id.to_string(), true);
    /// assert_eq!(connection.get_is_active(), &true);
    /// ```
    pub fn get_is_active(&self) -> &bool {
        &self.is_active
    }
}
