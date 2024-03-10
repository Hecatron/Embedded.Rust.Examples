# Readme

This is a set of examples relating to the ES32 ProS3 board use with rust

  * https://esp32s3.com/pros3.html

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
