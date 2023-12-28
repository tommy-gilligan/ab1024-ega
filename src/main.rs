#![no_std]
#![no_main]

mod epd;

use heapless::FnvIndexSet;
use esp_backtrace as _;
use esp_println::println;
use tinyqoi::Qoi;
use hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    gpio::IO,
    spi::{master::Spi, SpiMode},
    Delay
};
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        PrimitiveStyle, PrimitiveStyleBuilder, Rectangle
    },
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio18;
    let miso = io.pins.gpio25;
    let din = io.pins.gpio23;
    let cs = io.pins.gpio27;

    let rst = io.pins.gpio19.into_push_pull_output();
    let dc = io.pins.gpio33.into_push_pull_output();
    let busy = io.pins.gpio32.into_floating_input();

    let mut spi = Spi::new(peripherals.SPI2, 200u32.kHz(), SpiMode::Mode0, &clocks).with_pins(
        Some(sclk),
        Some(din),
        Some(miso),
        Some(cs),
    );

    let mut e = epd::Epd::new(spi, rst, dc, busy, delay);
    e.begin();
    let data = include_bytes!("remapped.qoi");
    let qoi = Qoi::new(data).unwrap();

    let mut pixels = FnvIndexSet::<_, 8>::new();
    for pixel in qoi.pixels() {
        pixels.insert(pixel).unwrap();
    }
    for pixel in &pixels {
        println!("{} {} {}", pixel.r(), pixel.g(), pixel.b());
    }

    Image::new(&qoi, Point::zero()).draw(&mut e).unwrap();
    e.display();

    loop {
    }
}
