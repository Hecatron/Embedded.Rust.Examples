#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;
use stm32f7xx_hal as hal;
use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

// Blinks an LED
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let gpiob = p.GPIOB.split();
    let mut led_red = gpiob.pb14.into_push_pull_output();
    let mut led_blue = gpiob.pb7.into_push_pull_output();
    let mut led_green = gpiob.pb0.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led_red.set_high();
            led_blue.set_low();
            led_green.set_low();
        }
        for _ in 0..10_000 {
            led_red.set_low();
            led_blue.set_high();
            led_green.set_low();
        }
        for _ in 0..10_000 {
            led_red.set_low();
            led_blue.set_low();
            led_green.set_high();
        }
    }
}
