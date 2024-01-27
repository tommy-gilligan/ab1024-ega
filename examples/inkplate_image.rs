#![no_std]
#![no_main]

//! This example demonstrates using the driver with a dither to approximate a display that can
//! render Rgb888 at each pixel.
//!
//! ![Inkplate displaying a dithered version of Vincent van Gogh's The Starry Night][photo]
//!
#![doc = ::embed_doc_image::embed_image!("photo", "examples/image_photo.jpg")]
//!

use ab1024_ega::color::Color;
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

const RGB_DISPLAY_PAIRS: [(Rgb888, Color); 7] = [
    (Rgb888::new(0x00, 0x00, 0x00), Color::BLACK),
    (Rgb888::new(0xFF, 0xFF, 0xFF), Color::WHITE),
    (Rgb888::new(0x10, 0xcb, 0x10), Color::GREEN),
    (Rgb888::new(0x20, 0x20, 0xff), Color::BLUE),
    (Rgb888::new(0xff, 0x30, 0x20), Color::RED),
    (Rgb888::new(0xff, 0xff, 0x50), Color::YELLOW),
    (Rgb888::new(0xf0, 0x70, 0x20), Color::ORANGE),
];

fn rgb_to_epd(color: Rgb888) -> Color {
    let (_, display) = RGB_DISPLAY_PAIRS
        .into_iter()
        .min_by_key(|(rgb, _): &(Rgb888, Color)| {
            let r: u16 =
                (<u8 as Into<u16>>::into(color.r())).abs_diff(<u8 as Into<u16>>::into(rgb.r()));
            let g: u16 =
                (<u8 as Into<u16>>::into(color.g())).abs_diff(<u8 as Into<u16>>::into(rgb.g()));
            let b: u16 =
                (<u8 as Into<u16>>::into(color.b())).abs_diff(<u8 as Into<u16>>::into(rgb.b()));
            r + g + b
        })
        .unwrap();

    display
}

fn epd_to_rgb(color: Color) -> Rgb888 {
    let (display, _) = RGB_DISPLAY_PAIRS
        .into_iter()
        .find(|(_, c): &(Rgb888, Color)| *c == color)
        .unwrap();

    display
}

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
    let mut display = ab1024_ega::Display::new(spi, rst, dc, busy, delay);

    let mut ed: DitherTarget<'_, _, _, _, { ab1024_ega::WIDTH + 1 }> =
        DitherTarget::new(&mut display, &rgb_to_epd, &epd_to_rgb);
    bmp.draw(&mut ed).unwrap();

    display.init().unwrap();
    display.display().unwrap();

    Rtc::new(peripherals.LPWR).sleep_deep(&[], &mut delay)
}
