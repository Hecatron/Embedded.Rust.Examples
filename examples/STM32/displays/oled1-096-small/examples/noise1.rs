// Send random raw data to the display, emulating an old untuned TV.
// https://github.com/jamwaffles/ssd1306/blob/master/examples/noise_i2c.rs
// To run
// cargo embed --example noise1 --release

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f7xx_hal as hal;
use crate::hal::{pac, prelude::*};

use embedded_graphics::{
    image::Image,
    image::ImageRaw,
    pixelcolor::BinaryColor,
    prelude::*
};
use rand::prelude::*;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let gpiob = dp.GPIOB.split();

    // Configure I2C1
    let scl = gpiob.pb8.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = hal::i2c::BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::fast(400_000.Hz()),
        &clocks,
        &mut rcc.apb1,
        50_000,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let mut buf = [0x00u8; 8192];
    let mut rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);

    loop {
        // Draw a random image
        rng.fill_bytes(&mut buf);
        let raw_image = ImageRaw::<BinaryColor>::new(&buf, 128);
        let image = Image::new(&raw_image, Point::zero());
        image.draw(&mut display).unwrap();
        display.flush().unwrap();
    }
}
