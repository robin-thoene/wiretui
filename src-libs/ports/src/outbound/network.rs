use domain::models::Connection;

pub trait NetworkService {
    /// Retrieves all already imported and available VPN connection
    fn get_imported_vpn_connections(&self) -> Result<Vec<Connection>, Box<dyn std::error::Error>>;
}
