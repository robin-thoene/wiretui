use adapters::{inbound::tui::App, outbound::wireguard_dbus_repository::WireGuardDBusRepository};
use application::{
    activate_connection_usecase::ActivateConnectionUsecase,
    deactivate_connection_usecase::DeactivateConnectionUsecase,
    list_connections_usecase::ListConnectionsUseCase,
};
use env_logger::{Builder, Env, Target};
use std::{
    env,
    fs::{self, File},
    io,
};

fn main() -> io::Result<()> {
    // Setup the logger
    let home = env::var("HOME").expect("expect a home env to be set");
    let local_state_path = format!("{}/.local/state/wiretui", home);
    fs::create_dir_all(&local_state_path)?;
    let log_file = File::create(format!("{}/log.txt", &local_state_path))?;
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env)
        .target(Target::Pipe(Box::new(log_file)))
        .init();
    log::debug!("configured logger");
    // Build the dependencies
    let wireguard_dbus_adapter =
        WireGuardDBusRepository::new().expect("could not connect to D-Bus");
    let list_connections_usecase = ListConnectionsUseCase::new(&wireguard_dbus_adapter);
    let activate_connection_usecase = ActivateConnectionUsecase::new(&wireguard_dbus_adapter);
    let deactivate_connection_usecase = DeactivateConnectionUsecase::new(&wireguard_dbus_adapter);
    log::debug!("buit the dependencies");
    // Build and run the TUI application
    let mut tui_app = App::new(
        list_connections_usecase,
        activate_connection_usecase,
        deactivate_connection_usecase,
    );
    log::debug!("created the TUI application");
    log::info!("running TUI application ...");
    let result = tui_app.run();
    log::info!("TUI application stopped");
    result
}
