#![no_main]
#![no_std]

// Print panic message to probe console
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f7xx_hal::{
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let gpiob = p.GPIOB.split();
    let mut _green_led = gpiob.pb0.into_push_pull_output();
    let mut _blue_led = gpiob.pb7.into_push_pull_output();
    let mut _red_led = gpiob.pb14.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            _red_led.set_high();
        }
        for _ in 0..10_000 {
            _red_led.set_low();
        }
    }
}
