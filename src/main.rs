#![no_std]
#![no_main]

mod epd;

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    gpio::IO,
    spi::{master::Spi, SpiMode},
    Delay
};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        PrimitiveStyle, PrimitiveStyleBuilder, Rectangle
    },
};

// EPAPER_RST_PIN  19
// EPAPER_DC_PIN   33
// EPAPER_BUSY_PIN 32

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
    println!("before begin");
    e.begin();
    println!("after begin");

    println!("before display");
    e.display();
    println!("after display");

    loop {
    }
}
