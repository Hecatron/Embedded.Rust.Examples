use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led_red = PinDriver::output(peripherals.pins.gpio0).unwrap();
    let mut led_green = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let mut led_blue = PinDriver::output(peripherals.pins.gpio4).unwrap();

    loop {
        let _ = led_red.set_high();
        sleep(Duration::from_millis(1000));
        let _ =led_red.set_low();
        sleep(Duration::from_millis(1000));

        let _ = led_green.set_high();
        sleep(Duration::from_millis(1000));
        let _ =led_green.set_low();
        sleep(Duration::from_millis(1000));

        let _ = led_blue.set_high();
        sleep(Duration::from_millis(1000));
        let _ =led_blue.set_low();
        sleep(Duration::from_millis(1000));
    }
}
