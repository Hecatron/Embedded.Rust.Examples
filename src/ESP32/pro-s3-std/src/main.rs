use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::*;
use esp_idf_hal::prelude::*;
use ws2812_spi::{Ws2812};
use smart_leds::{gamma, SmartLedsWrite, RGB8, hsv::Hsv, hsv::hsv2rgb};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    
    let spi = peripherals.spi2;
    let sclk = peripherals.pins.gpio15;
    let serial_in = peripherals.pins.gpio16; // SDI
    let serial_out = peripherals.pins.gpio18; // SDO

    let driver = SpiDriver::new::<SPI2>(
       spi,
       sclk,
       serial_out,
       Some(serial_in),
       &SpiDriverConfig::new(),
    ).unwrap();
    let config_1 = config::Config::new().baudrate(2.MHz().into());
    //let mut device_1 = SpiDeviceDriver::new(&driver, Some(cs_1), &config_1);


    let mut dev1 = SpiBusDriver::new(driver,&config_1).unwrap();
    let mut ws2812 = Ws2812::new(dev1);

    //let mut ws2812 = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(channel, led_pin).unwrap();

    loop {


        //let pixels = std::iter::repeat(RGBW8::new_alpha(6, 0, 0, White(0))).take(25);
        //ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        //let pixels = std::iter::repeat(RGBW8::new_alpha(0, 6, 0, White(0))).take(25);
        //ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        //let pixels = std::iter::repeat(RGBW8::new_alpha(0, 0, 6, White(0))).take(25);
        //ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

    }
}
