use esp_idf_hal::delay::FreeRtos;
use log::info;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    // We use log instead of defmt with std for the esp32
    // https://github.com/knurling-rs/defmt/discussions/888
    // Due to some compiler issues
    rtt_target::rtt_init_log!();

    loop {
        info!("Hello world!");
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);
    }
}

// TODO not sure if this is actually running or outputing at this stage
// Try a ProS3 board or vanilla ESP32 with an LED output
// Or check with scope
// maybe try esp 5.2.3
