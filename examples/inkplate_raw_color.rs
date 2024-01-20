#![no_std]
#![no_main]

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    Delay,
};
use embedded_graphics::{
    image::{ImageRaw, Image},
    prelude::Point,
    Drawable
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let delay = Delay::new(&clocks);
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

    let mut e = ab1024_ega::Epd::new(spi, rst, dc, busy, delay);
    e.init().unwrap();

    const IMAGE_DATA: &[u8] = &[
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
        0b00110011,
    ];
    let image_raw: ImageRaw<ab1024_ega::color::Color> = ImageRaw::new(IMAGE_DATA, 8);
    let image = Image::new(&image_raw, Point::new(0, 0));
    image.draw(&mut e).unwrap();
    e.display().unwrap();

    loop {}
}