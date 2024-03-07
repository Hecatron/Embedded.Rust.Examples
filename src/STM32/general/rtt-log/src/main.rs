#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f7xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    rcc::{HSEClock, HSEClockMode},
};

// Blinks an LED
#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop1();
    loop {}
}

pub fn loop1() {
    rprintln!("Hello, world!");
    let mut count1: u32 = 0;
    let dp = pac::Peripherals::take().unwrap();

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
        delay.delay(100.millis());
        rprintln!("Test {}", count1);
        count1 = count1 + 1;
        if count1 == u32::MAX {
            count1 = 0
        }
    }
}
