# wiretui

## Summary

A minimal keyboard-driven TUI to manage WireGuard VPN connections

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
