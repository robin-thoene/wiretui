use adapters::{inbound::tui::App, outbound::wireguard_dbus_repository::WireGuardDBusRepository};
use application::{
    activate_connection_usecase::ActivateConnectionUsecase,
    list_connections_usecase::ListConnectionsUseCase,
};
use std::io;

fn main() -> io::Result<()> {
    // Build the dependencies
    let wireguard_dbus_adapter = WireGuardDBusRepository::new().expect("TODO: fix later");
    let list_connections_usecase = ListConnectionsUseCase::new(&wireguard_dbus_adapter);
    let activate_connection_usecase = ActivateConnectionUsecase::new(&wireguard_dbus_adapter);
    // Build and run the TUI application
    let mut tui_app = App::new(list_connections_usecase, activate_connection_usecase);
    tui_app.run()
}
