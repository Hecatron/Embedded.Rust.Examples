# Flashing Firmware

THe below lists 3 different ways to flash the firmware

TODO in debug mode (non release) the board just halts without looping
I think its the for loop

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


```yaml
[default.general]
chip = "STM32F767ZITx"
```

## Probe-RS

The last approach is to upload via probe-rs

```sh
probe-rs run --chip STM32F767ZITx
```

We can also set this to run via `cargo run`

.cargo/config
```yaml
[target.thumbv7em-none-eabihf]
runner = 'probe-rs run --chip STM32F767ZITx'
```
