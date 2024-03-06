#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _; // panic handler
use stm32f7xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    rcc::{HSEClock, HSEClockMode},
};

// Blinks an LED
#[entry]
fn main() -> ! {
    loop1();
    loop {}
}

pub fn loop1() {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Setup the gpio
        let gpiob = dp.GPIOB.split();
        let mut led_red = gpiob.pb14.into_push_pull_output();
        let mut led_blue = gpiob.pb7.into_push_pull_output();
        let mut led_green = gpiob.pb0.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc
            .cfgr
            .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
            .sysclk(48.MHz())
            .freeze();

        // Create a delay abstraction based on general-pupose 32-bit timer TIM5
        let mut delay = dp.TIM5.delay_us(&clocks);

        loop {
            delay.delay(1.secs());
            led_red.set_high();
            led_blue.set_low();
            led_green.set_low();

            delay.delay(1.secs());
            led_red.set_low();
            led_blue.set_high();
            led_green.set_low();

            delay.delay(1.secs());
            led_red.set_low();
            led_blue.set_low();
            led_green.set_high();
        }
    }
}
