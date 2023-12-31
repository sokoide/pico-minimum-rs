# pico-minimum-rs

* Minimum bare metal Raspberry Pi Pico example in Rust

## How this was generated

* Install prereqs

```sh
rustup target add thumbv6m-none-eabi
cargo install cargo-generate
cargo install flip-link probe-run elfuf2-rs probe-rs-debugger
```

* Generate a repo by `cargo generate --git https://github.com/rp-rs/rp2040-project-template`

## How to run

* CLI
  * Connect to Debug probe or picoprobe
  * Run it

    ```sh
    cargo run
    ```

* VSCode
  * Install `Debugger for probe-rs` vscode extension
  * [F5] to debug

## Links

* [RP2040 project template](https://github.com/rp-rs/rp2040-project-template)
