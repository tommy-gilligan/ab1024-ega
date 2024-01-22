#![no_std]
#![no_main]

//! This example demonstrates drawing a few embedded-graphics primitives
//!
//! ![Inkplate displaying 3 circles in a row.  The last circle in the row is blue and overlaps a
//! preceding yellow circle.  The yellow circle overlaps a preceding red circle.][graphics]
//!
#![doc = ::embed_doc_image::embed_image!("graphics", "examples/graphics_photo.jpg")]
//!

use embedded_graphics::{
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
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

    let mut display = ab1024_ega::Display::new(spi, rst, dc, busy, delay);

    Circle::with_center(Point::new(150, 224), 200)
        .into_styled(PrimitiveStyle::with_fill(ab1024_ega::color::Color::RED))
        .draw(&mut display)
        .unwrap();
    Circle::with_center(Point::new(300, 224), 200)
        .into_styled(PrimitiveStyle::with_fill(ab1024_ega::color::Color::YELLOW))
        .draw(&mut display)
        .unwrap();
    Circle::with_center(Point::new(450, 224), 200)
        .into_styled(PrimitiveStyle::with_fill(ab1024_ega::color::Color::BLUE))
        .draw(&mut display)
        .unwrap();

    display.init().unwrap();
    display.display().unwrap();

    Rtc::new(peripherals.LPWR).sleep_deep(&[], &mut delay)
}
