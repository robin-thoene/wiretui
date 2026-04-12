# wiretui

## Summary

A minimal keyboard-driven TUI to manage WireGuard VPN connections, heavily inspired by
[bluetui](https://github.com/pythops/bluetui) and [impala](https://github.com/pythops/impala).

## Demo

TODO

<!-- ![icon](./assets/demo.png) -->

## Prerequisites

You will need to use a **Linux** based operating system using the **NetworkManager**.

## (Planned) features

- [x] list all imported connections
- [x] activate a connection
- [x] deactivate a connection
- [ ] import a new connection from a config file ([#7](https://github.com/robin-thoene/wiretui/issues/7))
- [ ] remove an existing connection ([#6](https://github.com/robin-thoene/wiretui/issues/6))
- [ ] search connection list ([#13](https://github.com/robin-thoene/wiretui/issues/13))

## Installation

Currently the only option to install is building from source, but it is planned to add pre-built
binaries that you can download from releases ([#3](https://github.com/robin-thoene/wiretui/issues/3)).

```shell
git clone https://github.com/robin-thoene/wiretui.git
cd wiretui
cargo build --release
```

## Usage

### Starting

```shell
wiretui
```

### Keybindings

`j`: move down

`k`: move up

`SPACE`: toggle a connection

`?`: open the help menu

`ESC`: close a menu

`q`: quit application

### Importing connections

Until this feature is integrated into wiretui, you can import connections from WireGuard config
files like so:

```shell
# Import from config using the CLI of the NetworkManager
nmcli connection import type wireguard file ./your_wireguard_config_file.conf
# Use this command if you want to disable the autoconnect functionality
nmcli connection modify IMPORTED_CONNECTION_NAME connection.autoconnect no
```

## Local development

### CLI

Run the application

```shell
cargo run --bin wiretui-bin
```

Run the tests

```shell
cargo test
```

Check if you violated the hexagonal architecture dependency rules

```shell
./scripts/lint_architecture.sh
```

Check the workspace rules (using [cargo-deny](https://github.com/EmbarkStudios/cargo-deny))

```shell
cargo deny check
```

### Debugging

This project contains a debugger configuration using [.vscode files](./.vscode/). Those can be
used in Neovim as well (see example [here](https://github.com/robin-thoene/dotfiles/blob/f388381f49d4b79e2755e18929d2462d198bd30d/.config/nvim/lua/plugins/nvim_dap.lua#L98)).

## Logging

This application uses the **log** crate in combination with the **env_logger** crate to set the
log level. By default the logs are written to a file, which is overridden every time you run the
application.

You can see it's current content with

```shell
cat ~/.local/state/wiretui/log.txt
```

To override the log level when running the application use

```shell
RUST_LOG=debug cargo run
```

Valid log level are `trace`, `debug`, `info`, `warn` and `error`.

## Contributing

As of now this project is publicly available but not actively asking for or accepting pull
requests until I finished my desired MVP state. This is due to the fact that the project is part
of a university module as well as a learning opportunity for myself.

For the moment you can contribute best by:

- submitting bug reports
- reviewing code to be idiomatic, secure and performant

## License

See the [LICENSE](https://github.com/robin-thoene/wiretui/blob/main/LICENSE)
