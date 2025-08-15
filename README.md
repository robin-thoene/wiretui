# hexa-rs

![icon](./assets/hexa-rs-icon.png)

DISCLAIMER: This image is AI generated. If you have design skills and want to contribute, a human
created image is very much welcome.

## Summary

This template initializes a new Rust project following the hexagonal architecture principals,
agnostic about what kind of application you want to develop. After the use of this template, be
sure to address the `# TODO` that are left across the code base.

## Local development

Run the application

```shell
cargo run
```

Run the application with hot reloading (using [watchexec](https://github.com/watchexec/watchexec))

```shell
watchexec -r -- cargo run
```

Run the tests

```shell
cargo test
```

Run the tests with hot reloading (using [watchexec](https://github.com/watchexec/watchexec))

```shell
watchexec -r -- cargo test
```

Check if you violated the hexagonal architecture dependency rules

```shell
./scripts/lint_architecture.sh
```

Check the workspace rules (using [cargo-deny](https://github.com/EmbarkStudios/cargo-deny))

```shell
cargo deny check
```
