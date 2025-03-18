#![no_std]
#![no_main]

// examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
// Also at https://github.com/okhsunrog/esp32s3test/tree/main

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::peripherals::Peripherals;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::{Duration, Instant};
use panic_rtt_target as _;

use smart_leds::SmartLedsWriteAsync;
use smart_leds::{brightness, RGB8};
use ws2812_async::{Grb, Ws2812};
const NUM_LEDS: usize = 1;

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    //let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    //let peripherals = esp_hal::init(config);
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(esp_hal::time::Rate::from_khz(3200))
            .with_mode(Mode::_0),
    );
    


    // TODO LED is a WS2812B on GPIO8
    //let mut led = Output::new(peripherals.GPIO0, Level::High, OutputConfig::default());

    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        //led.toggle();
    }
}




fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
