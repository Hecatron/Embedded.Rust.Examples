# Readme

This is a set of examples relating to the STM32F767ZI use with rust
For the NUCLEO-F767ZI board

  * https://www.st.com/en/evaluation-tools/nucleo-f767zi.html
  * https://www.st.com/resource/en/user_manual/um1974-stm32-nucleo144-boards-mb1137-stmicroelectronics.pdf

Currently this is setup for use with probe-rs

  * https://github.com/probe-rs/probe-rs

The following HAL library is used
To build the below library outside of a dependency `cargo build --features=stm32f767,rt`

  * https://github.com/stm32-rs/stm32f7xx-hal

For the debugger it's setup to use probe-rs

  * https://probe.rs/docs/tools/debugger/

## Toolchain

```
# For the Arm target
rustup update
rustup target install thumbv7m-none-eabi

# This can be used as a seperate tool to flash the board outside of probe-rs
cargo install cargo-flash
```


## flashing

```
# To build a release version
cargo build --release
# To flash the code to the device
cargo embed --release
```
