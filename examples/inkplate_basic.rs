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

    let mut e = ab1024_ega::Epd::new(spi, rst, dc, busy, delay);
    e.init().unwrap();
    let colors = [
        ab1024_ega::color::Color::BLACK,
        ab1024_ega::color::Color::WHITE,
        ab1024_ega::color::Color::GREEN,
        ab1024_ega::color::Color::BLUE,
        ab1024_ega::color::Color::RED,
        ab1024_ega::color::Color::YELLOW,
        ab1024_ega::color::Color::ORANGE,
    ];
    let stripe_width = ab1024_ega::WIDTH / colors.len();

    for (index, color) in colors.into_iter().enumerate() {
        for x in (index * stripe_width)..ab1024_ega::WIDTH {
            for y in 0..ab1024_ega::HEIGHT {
                e.set_pixel(x, y, color);
            }
        }
    }
    e.display().unwrap();

    let mut rtc = Rtc::new(peripherals.LPWR);
    rtc.sleep_deep(&[], &mut delay)
}
