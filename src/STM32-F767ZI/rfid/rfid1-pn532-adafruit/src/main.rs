#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f7xx_hal as hal;
use pn532::{requests::SAMMode, spi::SPIInterface, Pn532, Request};

use crate::hal::{
    pac,
    prelude::*,
    spi::{self, Spi},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Setting up SPI Interface");

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    //let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    // Setup timer
    let timer = dp.TIM1.counter_ms(&clocks);

    // Prepare pins for SPI
    let mut cs = gpioc.pc9.into_push_pull_output();
    let sck = gpioc.pc10.into_alternate();
    let miso = gpioc.pc11.into_alternate();
    let mosi = gpioc.pc12.into_alternate();

    // Set NCS pin to high (disabled) initially
    cs.set_high();

    // Initialize SPI
    let spi = Spi::new(dp.SPI3, (sck, miso, mosi)).enable::<u8>(
        spi::Mode {
            polarity: spi::Polarity::IdleHigh,
            phase: spi::Phase::CaptureOnSecondTransition,
        },
        250.kHz(),
        &clocks,
        &mut rcc.apb1,
    );

    rprintln!("Setting up pn532");

    // setup spi interface for pn532
    let interface = SPIInterface {
        spi,
        cs,
    };

    // Setup pn532
    let mut pn532: Pn532<_, _, 32> = Pn532::new(interface, timer);
    if let Err(_e) = pn532.process(&Request::sam_configuration(SAMMode::Normal, false), 0, 50.millis()){
        rprintln!("Could not initialize PN532: {_e:?}")
    }

    // Read tag
    if let Ok(_uid) = pn532.process(&Request::INLIST_ONE_ISO_A_TARGET, 7, 1000.millis()){
        let result = pn532.process(&Request::ntag_read(10), 17, 50.millis()).unwrap();
        if result[0] == 0x00 {
            rprintln!("page 10: {:?}", &result[1..5]);
        }
    }
    loop {}
}
