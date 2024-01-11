#![no_std]
#![no_main]

use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
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
use nalgebra::Vector3;
use tinybmp::Bmp;

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
    e.begin();

    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("starry-night.bmp")).unwrap();
    for (pixel, color) in bmp.pixels().zip(dither::Dither::<
        _,
        _,
        { ab1024_ega::WIDTH },
        { ab1024_ega::WIDTH + 1 },
    >::new(
        bmp.pixels().map(|c| {
            let color = c.1;
            Vector3::<i16>::new(color.r().into(), color.g().into(), color.b().into())
        }),
        ab1024_ega::color::closestrgb,
    )) {
        e.set_pixel(
            pixel.0.x as usize,
            pixel.0.y as usize,
            ab1024_ega::color::closest(color),
        )
    }

    e.display().unwrap();

    loop {}
}
