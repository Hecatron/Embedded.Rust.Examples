# ESP32 Overview

## Targets

There's no target included by default in the rust toolchains
so we may need to setup a custom one using a custom llvm

  * https://doc.rust-lang.org/beta/rustc/targets/custom.html

## Libs

  * https://github.com/esp-rs
    Root organisation for esp32 / rust code

  * https://github.com/esp-rs/esp32
    A peripheral access crate the ESP32 - svd based

  * https://github.com/esp-rs/esp32-hal
    depends on the esp32 library, HAL abstraction

  * https://github.com/rust-embedded-community/embedded-sdmmc-rs
    sd card


## Toolchain

  * https://github.com/MabezDev/rust-xtensa
  * https://github.com/espressif/llvm-project

## Examples

  * https://github.com/esp-rs/esp32-hal/tree/master/examples
  * https://github.com/MabezDev/xtensa-rust-quickstart
  * https://github.com/lexxvir/esp32-hello
    Uses it's own implementation / binding to esp's libs
  * https://dentrassi.de/2019/06/16/rust-on-the-esp-and-how-to-get-started/


## Wifi Example

  * https://github.com/esp-rs/esp32-wifi

It looks like this wifi lib is experimental so not included in the global crates
so needs to be downloaded / built manually
It uses esp32-hal which depends on the esp32 crate

It uses the wifi library blobs from here

  * https://github.com/espressif/esp32-wifi-lib

Although I think the bindings need regenerating for the newer blobs


## LEDs

For the ESP-WROVER-KIT board

  * Leds are on GPIO0, GPIO2, GPIO4

## SPI PSRAM

Dev board is a ESP32-WROVER-B

By Default there's 512KiB (KiloBytes) or 4Mb (Megabits)
This is split into 328k DRAM and 192k IRAM (Instruction Ram)

There's also and additional 4Mb PSRAM on some boards such as the ESP-WROVER-B
called pseudo static RAM

  * https://thingpulse.com/esp32-how-to-use-psram/
  * https://arduinojson.org/v6/how-to/use-external-ram-on-esp32/
  * https://www.esp32.com/viewtopic.php?t=1831
  * http://blog.pagefault-limited.co.uk/lolin32-lite-esp32-8mb-psram-upgrade-mod

Apparantley only 4Mb is accessible even when 8Mb is attached. but you can access the rest via bank switching

  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/himem.html

## Flash Size

By Default the flash size is 4MiB (MegaByte)
