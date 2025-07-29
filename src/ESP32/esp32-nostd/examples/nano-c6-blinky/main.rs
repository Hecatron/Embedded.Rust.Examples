#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use panic_rtt_target as _;

extern crate alloc;

//use crate::ws2812::Ws2812;
//use smart_leds_trait::RGB8;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 64 * 1024);


    
    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // let mut ws = Ws2812::new(spi);
    // loop {
    //     data[0] = RGB8 {
    //         r: 0,
    //         g: 0,
    //         b: 0x10,
    //     };
    //     data[1] = RGB8 {
    //         r: 0,
    //         g: 0x10,
    //         b: 0,
    //     };
    //     data[2] = RGB8 {
    //         r: 0x10,
    //         g: 0,
    //         b: 0,
    //     };
    //     ws.write(data.iter().cloned()).unwrap();
    //     delay.delay_ms(1000 as u16);
    //     ws.write(empty.iter().cloned()).unwrap();
    //     delay.delay_ms(1000 as u16);
    // }

}
