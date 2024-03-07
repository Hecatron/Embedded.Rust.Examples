# Readme

This is an example of logging / printing to a console using rtt

  * https://probe.rs/docs/tools/debugger/#configuring-rtt-to-transfer-data

Note that the critical-section feature needs to be enabled under cortex-m for this to work

```toml
[dependencies]
cortex-m = {version = "0.7.7", features = ["critical-section-single-core"]}
```

## Building / Flashing

```
# To build a release version
cargo build --release
# To flash the code to the device
cargo embed --release
```
