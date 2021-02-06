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
    let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    // TODO old
    let mut rtccntl = dp.RTCCNTL;
    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;
    // (https://github.com/espressif/openocd-esp32/blob/97ba3a6bb9eaa898d91df923bbedddfeaaaf28c9/src/target/esp32.c#L431)
    // openocd disables the wdt's on halt
    // we will do it manually on startup
    wdt::disable_timg_wdts(&mut timg0, &mut timg1);
    wdt::disable_rtc_wdt(&mut rtccntl);

    // TODO new, issue with borrowing?
    //wdt::disable_wdts(&mut dp);


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
