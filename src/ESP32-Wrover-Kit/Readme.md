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

## Other Devices

Typically with devices such as the TTGo or ESP-WROOM-32
They seem to use a CP210x usb to serial uart, this however cannot be used for debugging over jtag
For jtag to work it needs to be a FT2232, this typically has support for a serial port and a jtag interface
whereby it can operating in MPSSE mode for jtag, the CP210x serial interfaces cannot do this.

  * https://forum.sparkfun.com/viewtopic.php?t=15930
  * https://community.platformio.org/t/esp32-debugging-with-onboard-cp210/5215

So for these other devices a hookup of the segger is needed

  * https://dzone.com/articles/eclipse-jtag-debugging-the-esp32-with-a-segger-j-l
