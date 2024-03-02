# Flashing Firmware

There are two ways to flash the firmware

## Cargo Flash

The first is the use of cargo flash which is bundled with probe-rs
```sh
cargo flash --release --chip STM32F767ZITx
```

## Cargo Embed

The second approach is to use cargo embed which is a more advanced version of cargo flash
```sh
cargo embed --release
```

cargo embed can be configured with a `Embed.toml` file

  * https://probe.rs/docs/tools/cargo-embed/#configuration
  * https://github.com/probe-rs/probe-rs/blob/master/probe-rs/src/bin/probe-rs/cmd/cargo_embed/config/default.toml

TODO in debug mode (non release) the board just halts without looping
