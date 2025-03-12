# Readme

https://github.com/rp-rs/rp2040-project-template
https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples

run-riscv --release --bin=blinky
cargo build --release --bin=blinky

## Picotool

this can be obtained from

 * https://www.raspberrypi.com/news/raspberry-pi-pico-windows-installer/

## Modes

The Rpi Pico 2W has two modes of operation
https://www.youtube.com/watch?v=qNbKqf8ZLKY

### BootSel Mode

In bootsel mode (default if not programmed) will show two usb devices
one winusb and one filesystem where we can copy across a U2F file
To enter into bootsel mode manually hold down the bootsel button while plugging the device in

### Programmed Mode

Outside of bootsel mode the device shows up as a single serial port called "Pico 2W"

### Arduino Programming

It looks as if when programming via Arduino
it flips the device into Bootsel mode manually, copied the file across, then resets it into Programmed Mode

### Debuging

Debugging can't be done via the USB port, instead a seperate SWD connection is needed
such as via the pico probe

## Wireless

  * https://docs.rs/cyw43/latest/cyw43/
  * https://crates.io/crates/cyw43

Also the LED for the wireless boards is connected to gpio0 of the cyw43 wireless chip
