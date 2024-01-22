#![no_std]
#![no_main]

//! This example demonstrates using the driver without embedded-graphics
//!
//! ![Inkplate displaying a solid rectangle for each color in AB1024-EGA palette.  The rectangles are black, white, green, blue, red, yellow and orange.][no_graphics]
//!
#![doc = ::embed_doc_image::embed_image!("no_graphics", "examples/no_graphics_photo.jpg")]
//!

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

    let colors = [
        ab1024_ega::color::Color::BLACK,
        ab1024_ega::color::Color::WHITE,
        ab1024_ega::color::Color::GREEN,
        ab1024_ega::color::Color::BLUE,
        ab1024_ega::color::Color::RED,
        ab1024_ega::color::Color::YELLOW,
        ab1024_ega::color::Color::ORANGE,
    ];
    let mut display = ab1024_ega::Epd::new(spi, rst, dc, busy, delay);
    for (index, color) in colors.into_iter().enumerate() {
        for x in (index * ab1024_ega::WIDTH / colors.len())..ab1024_ega::WIDTH {
            for y in 0..ab1024_ega::HEIGHT {
                display.set_pixel(x, y, color);
            }
        }
    }

    display.init().unwrap();
    display.display().unwrap();

    Rtc::new(peripherals.LPWR).sleep_deep(&[], &mut delay)
}
