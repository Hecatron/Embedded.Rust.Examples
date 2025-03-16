# Rust Setup

## Rust Drivers

First there is the `nrf-hal` rust library for hardware support of nrf chipsets

  * https://github.com/nrf-rs
  * https://github.com/nrf-rs/nrf-hal
  * https://github.com/nrf-rs/microbit

For the target CPU

  * Microbit 1.x - `thumbv6m-none-eabi`
  * Microbit 2.x - `thumbv7em-none-eabi`

In the case of the dependencies.
The example project for 1.x depends on the microbit crate.
This then depends on the microbit-common crate, which then depends on the nrf51-hal crate

  * https://github.com/nrf-rs/nrf-hal/tree/master/nrf51-hal

Note this is a nonstd setup so none of the `std` libraries will be available by default

## Tools

Some tools to install for development
```bash
cargo install probe-rs-tools
cargo install flip-link
# For Microbit V1.x
rustup target add thumbv6m-none-eabi
# For Microbit V2.x
rustup target add thumbv7em-none-eabi
```

## Debugging

The svd files are available from

  * https://github.com/nrf-rs/nrf-pacs/tree/master/svds

TODO

This seems to be one of the boards that works fine with probe-rs under windows
Probably due to the onboard Jtag
Although the firmware does need to be udpated first for the Jtag interface
