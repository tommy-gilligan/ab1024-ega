#![no_std]
#![no_main]

//! This example demonstrates using the driver with a dither to approximate a display that can
//! render Rgb888 at each pixel.
//!
//! ![Inkplate displaying a dithered version of Vincent van Gogh's The Starry Night][photo]
//!
#![doc = ::embed_doc_image::embed_image!("photo", "examples/image_photo.jpg")]
//!

use dither::DitherTarget;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    Delay, Rtc,
};
use tinybmp::Bmp;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut delay = Delay::new(&clocks);
    let rst = io.pins.gpio19.into_push_pull_output();
    let dc = io.pins.gpio33.into_push_pull_output();
    let busy = io.pins.gpio32.into_floating_input();
    let cs = io.pins.gpio27.into_push_pull_output();

    let spi = ExclusiveDevice::new_no_delay(
        Spi::new(peripherals.SPI2, 200u32.kHz(), SpiMode::Mode0, &clocks)
            .with_sck(io.pins.gpio18)
            .with_mosi(io.pins.gpio23),
        cs,
    );

    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("starry-night.bmp")).unwrap();
    let mut display = ab1024_ega::Epd::new(spi, rst, dc, busy, delay);

    let mut ed: DitherTarget<'_, _, { ab1024_ega::WIDTH }, { ab1024_ega::WIDTH + 1 }> =
        DitherTarget::new(&mut display);
    bmp.draw(&mut ed).unwrap();

    display.init().unwrap();
    display.display().unwrap();

    Rtc::new(peripherals.LPWR).sleep_deep(&[], &mut delay)
}
