# ESP32 Overview

This is a bunch of docs I've created for how to get the ESP32 working with rust
Some of this is based on the xtensa-rust-quickstart

  * [Hardware](./Hardware.md)

## Setup

  * [Installation of Tools](./Setup/Install.md)
  * [Building the toolchain](./Setup/ToolchainBuild.md)
  * [VSCode Setup](./Setup/VSCode.md)

## Develop

  * [Building the Source](./Develop/Build.md)

## Debugging

  * [Flashing to the Board](./Debug/Flashing.md)
  * [JTag Setup](./Debug/JTagSetup.md)
  * [Debugging using JTag](./Debug/Debug.md)

## Links

  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://dentrassi.de/2019/06/16/rust-on-the-esp-and-how-to-get-started/

## Libs

  * https://github.com/esp-rs
    Root organisation for esp32 / rust code

  * https://github.com/esp-rs/esp32
    A peripheral access crate the ESP32 - svd based

  * https://github.com/esp-rs/esp32-hal
    depends on the esp32 library, HAL abstraction

## Examples

  * https://github.com/ivmarkov/rust-esp32-std-demo
  * https://github.com/esp-rs/esp32-hal/tree/master/examples
  * https://dentrassi.de/2019/06/16/rust-on-the-esp-and-how-to-get-started/
  * https://github.com/lexxvir/esp32-hello
    Uses it's own implementation / binding to esp's libs
