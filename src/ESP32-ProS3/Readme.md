# Readme

This is a set of examples relating to the ES32 ProS3 board use with rust

  * https://esp32s3.com/pros3.html

probe-rs seems to not be not far enough along for the ESP32
so trying cortex-debug next

  * https://docs.esp-rs.org/book/tooling/debugging/openocd.html

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

## flashing

It looks as if probe-rs can't flash this board at the moment.
It's interface is a Espjtag interface

In order to flash we need to use the esptool flash util
also make sure the driver isn't set to winusb (it's set to it's original) so that we can access it as a com port
```
# Install the cargo flashing tool
cargo install cargo-espflash
# To flash the firmware
cargo espflash flash
```

## TODO

  * For launch.json The svd file?
