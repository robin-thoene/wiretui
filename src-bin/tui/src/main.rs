use adapters::{inbound::tui::App, outbound::wireguard_dbus_repository::WireGuardDBusRepository};
use application::{
    activate_connection_usecase::ActivateConnectionUsecase,
    deactivate_connection_usecase::DeactivateConnectionUsecase,
    list_connections_usecase::ListConnectionsUseCase,
};
use env_logger::{Builder, Env, Target};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    // Setup the logger
    let log_file = File::create("wiretui_log.txt")?; // TODO: put this into the correct dir
    let env = Env::default().filter_or("RUST_LOG", "warn");
    Builder::from_env(env)
        .target(Target::Pipe(Box::new(log_file)))
        .init();
    log::debug!("Configured logger");
    // Build the dependencies
    let wireguard_dbus_adapter = WireGuardDBusRepository::new().expect("TODO: fix later");
    let list_connections_usecase = ListConnectionsUseCase::new(&wireguard_dbus_adapter);
    let activate_connection_usecase = ActivateConnectionUsecase::new(&wireguard_dbus_adapter);
    let deactivate_connection_usecase = DeactivateConnectionUsecase::new(&wireguard_dbus_adapter);
    log::debug!("Buit the dependencies");
    // Build and run the TUI application
    let mut tui_app = App::new(
        list_connections_usecase,
        activate_connection_usecase,
        deactivate_connection_usecase,
    );
    log::debug!("Created the TUI application");
    log::info!("Starting TUI application ...");
    let result = tui_app.run();
    log::info!("TUI application stopped");
    result
}
