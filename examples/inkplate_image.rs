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
use embedded_graphics::prelude::*;
use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

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
    esp_println::println!("begin");
    e.begin();

    esp_println::println!("from_slice");
    let bmp = RawBmp::from_slice(include_bytes!("starry-night.bmp")).unwrap();
    esp_println::println!("iterate");
    for pixel in bmp.pixels() {
        let index = ((pixel.position.x >> 1) as usize + pixel.position.y as usize * 300).min(134400 - 1);
        if pixel.position.x % 2 == 0 {
            e.buffer[index] = (e.buffer[index] & 0xf0) | (ab1024_ega::color::closest(pixel.color));
        } else {
            e.buffer[index] = (e.buffer[index] & 0x0f) | (ab1024_ega::color::closest(pixel.color) << 4);
        }
    }

    e.display();

    loop {}
}
