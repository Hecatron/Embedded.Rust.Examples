# Readme

This is a set of examples relating to the ESP-WROVER-KIT use with rust
This is the plain original ESP32 variant

  * https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-wrover-kit-v3.html

Currently I'm trying to use this with probe-rs

  * https://github.com/probe-rs/probe-rs

## Toolchain

To setup the toolchain for the ESP32

  * https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html

```
cargo install espup
espup install
```

## flashing

### Flashing with espflash

For the original ESP32 dev boards they typically show up as a serial / com port
```
# Install the cargo flashing tool
cargo install cargo-espflash
# To flash the firmware
cargo espflash flash
```

Typically this shows up as the last com port under windows
so with COM6 and COM7 it'd be COM7

### Flashing with probe-rs

In order to flash with probe-rs you need to switch the serial port driver with WinUsb using zadig
For the ESP-WROVER-KIT there were two serial ports, for me trying this with the first port - interface0 seems to work
listed as FTDIBUS (v2.12.28.0) by default

To restore the original driver:

  * https://github.com/pbatard/libwdi/wiki/FAQ#Help_Zadig_replaced_the_driver_for_the_wrong_device_How_do_I_restore_it

To flash
```
cargo embed
```

## TODO

From the looks of things with probe-rs

  * Flashing sort of works, but the probe-rs halts / pauses after the flash as does the esp32
  * For debugging attaching works but not launching I think due to the above
  * single stepping doesn't work but breakpoints does
  * RTT logging doesn't work

  * For launch.json The svd file?
