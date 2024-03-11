#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio0.into_push_pull_output();

    led.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}
