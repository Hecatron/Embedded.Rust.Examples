// Send random raw data to the display, emulating an old untuned TV. This example retrieves the
// underlying display properties struct and allows calling of the low-level `draw()` method,
// sending a 1024 byte buffer straight to the display.
// Run with `cargo run --example noise1`. Best results when using `--release`.
// https://github.com/embedded-graphics/embedded-graphics/blob/master/src/image/image_raw.rs

use embedded_graphics::{image::Image, image::ImageRaw, pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::prelude::*;
use std::{thread, time::Duration};

fn main() {
    let (mut window, mut display) = setup_disp();
    let mut buf = [0x00u8; 8192];
    let mut rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);

    'running: loop {
        // Draw a random image
        rng.fill_bytes(&mut buf);
        let raw_image = ImageRaw::<BinaryColor>::new(&buf, 128);
        let image = Image::new(&raw_image, Point::zero());
        image.draw(&mut display).unwrap();

        // Update the simulator window
        if !update_win(&mut window, &mut display) {
            break 'running;
        };
    }
}

/// Setup the simulator window and display
pub fn setup_disp() -> (Window, SimulatorDisplay<BinaryColor>) {
    let display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let window = Window::new("Noise 1", &output_settings);
    (window, display)
}

/// Perform an update on the simulator window
/// Also capture the quit event and provide a minimum wait for the window events
pub fn update_win(window: &mut Window, display: &mut SimulatorDisplay<BinaryColor>) -> bool {
    window.update(&display);
    if window.events().any(|e| e == SimulatorEvent::Quit) {
        return false;
    }
    thread::sleep(Duration::from_millis(20));
    true
}
