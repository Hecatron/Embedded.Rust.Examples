#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use panic_rtt_target as _;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let mut pin4 = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    loop {
        info!("Setting GPIO4 High!");
        pin4.set_high();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(2000) {}

        info!("Setting GPIO4 Low!");
        pin4.set_low();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(2000) {}
    }
}
