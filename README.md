# wiretui

## Summary

A minimal keyboard-driven TUI to manage WireGuard VPN connections, heavily inspired by
[bluetui](https://github.com/pythops/bluetui) and [impala](https://github.com/pythops/impala).

## ⚠️Warning

This project is currently in development and is not yet stable nor meant to be used.

## Prerequisites

You will need to use a Linux based operating system with the **networkmanager**.

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

This application uses the **log** create in combination with **env_logger** to set the log level.
By default the logs are written to a file, which is overridden every time you run the application.

You can see it's current content with

```shell
cat ~/.local/state/wiretui/log.txt
```

To override the log level when running the application use

```shell
RUST_LOG=debug cargo run
```

Valid log level are `trace`, `debug`, `info`, `warn` and `error`.
