//use esp_idf_hal::delay::FreeRtos;
//use esp_idf_hal::gpio::*;
//use esp_idf_hal::peripherals::Peripherals;

//use defmt::info;
//use esp_hal::clock::CpuClock;
//use esp_hal::main;
//use esp_hal::time::{Duration, Instant};
//use panic_rtt_target as _;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    //let peripherals = Peripherals::take()?;
    //let mut led = PinDriver::output(peripherals.pins.gpio4)?;

    //rtt_target::rtt_init_defmt!();

    //let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    //let _peripherals = esp_hal::init(config);
    //esp_alloc::heap_allocator!(size: 64 * 1024);

    loop {
        //info!("Hello world!");
        //led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        //FreeRtos::delay_ms(1000);

        //led.set_low()?;
        //FreeRtos::delay_ms(1000);
    }
}
