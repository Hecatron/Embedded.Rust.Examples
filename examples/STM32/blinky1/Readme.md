# Readme

This basic example is targeted at the STM32F767ZI
for the NUCLEO-F767ZI board

  * https://www.st.com/en/evaluation-tools/nucleo-f767zi.html
  * https://www.st.com/resource/en/user_manual/um1974-stm32-nucleo144-boards-mb1137-stmicroelectronics.pdf

This uses the below Hal library and uses this as an example
To build the below library outside of a dependency `cargo build --features=stm32f767,rt`

  * https://github.com/stm32-rs/stm32f7xx-hal

## Debugger

For VSCode install the extension "Debugger for probe-rs"

  * https://probe.rs/docs/tools/debugger/#using-the-launch-request-type

Switched the "JLink" driver to "WinUSB"
using Zadig
note don't target the JLinkCDC driver, just the JLink one under interface 2 / bulk device


Seems to only work with the latest STLink Firmware for the STLink Interface
```sh
cargo flash --release --chip STM32F767ZITx --connect-under-reset
```



## Depends

```sh
# Needed target for the stm32
rustup target add thumbv7em-none-eabihf

# For flashing / debugging
cargo install cargo-binstall
cargo binstall probe-rs
```

## Building

```
cargo build --release
```

## Flashing

Todo switch the driver using

  * https://zadig.akeo.ie/
