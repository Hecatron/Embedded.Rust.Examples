# Arm Overview

## Target

For the NUCLEO-F767ZI this uses a STM32F767ZIT6
This is a Cortex®-M7 core

  * https://doc.rust-lang.org/beta/rustc/platform-support.html

If the core contains a FPU then it's considered a M7F core
according to the pdf this should include a FPU

  * https://www.st.com/en/microcontrollers-microprocessors/stm32f767zi.html#documentation

So the target should be: thumbv7em-none-eabihf


## Cargo Libs

  * https://github.com/jonlamb-gh/oxcc-nucleo-f767zi
    Has depends on the below
  * https://github.com/jonlamb-gh/oxcc-stm32f767-hal
  * https://github.com/rust-embedded/cortex-m

## Examples

  * https://github.com/rust-embedded/cortex-m-quickstart
  * https://github.com/rust-embedded/discovery
  * https://dev.to/minkovsky/rusted-brains-running-rust-firmware-on-a-cortex-m-microcontroller-3had
  * https://www.youtube.com/watch?v=CnVIz4KVm8M
