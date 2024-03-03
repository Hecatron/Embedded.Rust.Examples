# Readme

This basic example is targeted at the STM32F767ZI
for the NUCLEO-F767ZI board

  * https://www.st.com/en/evaluation-tools/nucleo-f767zi.html
  * https://www.st.com/resource/en/user_manual/um1974-stm32-nucleo144-boards-mb1137-stmicroelectronics.pdf

This uses the below Hal library and uses this as an example
To build the below library outside of a dependency `cargo build --features=stm32f767,rt`

  * https://github.com/stm32-rs/stm32f7xx-hal
  * https://github.com/stm32-rs/stm32f7xx-hal/blob/main/examples/blinky.rs


## Building / Flashing

```
# To build a release version
cargo build --release
# To flash the code to the device
cargo embed --release
```
