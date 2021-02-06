#![no_std]
#![no_main]

use esp32_hal::target;
use hal::prelude::*;
use xtensa_lx::timer::delay;
use panic_halt as _;
use esp32_hal as hal;

mod wdt;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
const CORE_HZ: u32 = 40_000_000;

#[entry]
fn main() -> ! {

    // Get Peripherals
    let mut dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    // Disable Watchdog timers
    wdt::disable_wdts(&mut dp);

    // Set GPIO4 for output
    let pins = dp.GPIO.split();
    let mut led = pins.gpio4.into_push_pull_output();

    //let mut testint = 0;

    loop {
        //testint = 1;
        led.set_high().unwrap();
        delay(CORE_HZ);
        //testint = 3;
        led.set_low().unwrap();
        delay(CORE_HZ);
    }
}
