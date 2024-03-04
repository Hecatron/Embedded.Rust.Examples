// Send random raw data to the display, emulating an old untuned TV. This example retrieves the
// underlying display properties struct and allows calling of the low-level `draw()` method,
// sending a 1024 byte buffer straight to the display.
// Run with `cargo run --example noise1`. Best results when using `--release`.

use embedded_graphics::{
    pixelcolor::BinaryColor,
    image::ImageRaw, image::Image,
    prelude::*
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};
use rand::prelude::*;

fn main() -> ! {
    let mut window = setup_win();
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let mut buf = [0x00u8; 1024];
    let mut rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);
    loop {
        rng.fill_bytes(&mut buf);
        let raw_image = ImageRaw::<BinaryColor>::new(&buf, 128);
        let image = Image::new(&raw_image, Point::zero());
        image.draw(&mut display).unwrap();
        window.update(&display);
    }
}

pub fn setup_win() -> Window{
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let window = Window::new("Hello World", &output_settings);
    window
}
