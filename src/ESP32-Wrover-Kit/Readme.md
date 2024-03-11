# Readme

This is a set of examples relating to the ESP-WROVER-KIT use with rust
This is the plain original ESP32 variant
Currently I'm trying to use this with cortex-debug

  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-wrover-kit-v3.html

## Toolchain

To setup the toolchain for the ESP32

  * https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html

```
cargo install espup
espup install
```

## OpenOcd / GDB

  * Openocd for the esp32 can be downloaded from  
    https://github.com/espressif/openocd-esp32/releases
  * GDB can be obtained from
    https://github.com/espressif/binutils-gdb/releases
  * we also need xtensa-esp-elf for objdump
    https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/tools/idf-tools.html#xtensa-esp-elf

## flashing

For the original ESP32 dev boards they typically show up as a serial / com port
```
# Install the cargo flashing tool
cargo install cargo-espflash
# To flash the firmware
cargo espflash flash
```

Typically this shows up as the last com port under windows
so with COM6 and COM7 it'd be COM7


## SPI Flash Voltage

Depending on the model there's a different voltage to pick.
Currently I'm using the 3.3V one

  * ESP32-WROVER - 1.8V
  * ESP32-WROVER-B - 3.3V
  * https://docs.espressif.com/projects/esp-idf/en/v4.2.1/esp32/hw-reference/modules-and-boards.html

## TODO

From the looks of things with cortex-debug

  * Flashing through openocd doesn't seem to update the board
  * For launch.json The svd file?
