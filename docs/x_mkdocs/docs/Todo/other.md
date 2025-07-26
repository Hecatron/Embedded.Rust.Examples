# TODO

https://craigjb.com/2024/09/12/probe-rs-vexriscv/
https://github.com/dbrgn/embedded-hal-mock

## Web Assembly

https://github.com/barafael/wasm-on-mcu/tree/main
https://github.com/wasmi-labs/wasmi


## Std / no_std

```toml
[unstable]
# build-std = ["std"]
build-std = ["std", "panic_abort"]
```

cargo +nightly build
cargo +nightly -Zbuild-std=std build

https://github.com/hobofan/cargo-nono


rustup +nightly target add thumbv8m.main-none-eabihf

https://github.com/nostd-rs/nostd


https://github.com/gerardcl/esp32-c3-rust-std
