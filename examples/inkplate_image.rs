#![no_std]
#![no_main]

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
    Delay,
};
use tinybmp::Bmp;

const WIDTH: usize = 600;

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

    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("starry-night.bmp")).unwrap();
    let mut e = ab1024_ega::Epd::new(spi, rst, dc, busy, delay);
    e.begin();

    let mut ed: DitherTarget<'_, _, _, WIDTH, { WIDTH + 1 }> =
        DitherTarget::new(&mut e, ab1024_ega::color::closestrgb);
    bmp.draw(&mut ed).unwrap();
    e.display().unwrap();

    loop {}
}
