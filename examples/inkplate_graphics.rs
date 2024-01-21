#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    text::Text,
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
use profont::PROFONT_24_POINT;

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

    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, ab1024_ega::color::Color::BLACK);
    Text::new("My favourite colors:", Point::new(24, 48), text_style)
        .draw(&mut e)
        .unwrap();

    Circle::with_center(Point::new(150, 224), 200)
        .into_styled(PrimitiveStyle::with_fill(ab1024_ega::color::Color::RED))
        .draw(&mut e)
        .unwrap();
    Circle::with_center(Point::new(300, 224), 200)
        .into_styled(PrimitiveStyle::with_fill(ab1024_ega::color::Color::YELLOW))
        .draw(&mut e)
        .unwrap();
    Circle::with_center(Point::new(450, 224), 200)
        .into_styled(PrimitiveStyle::with_fill(ab1024_ega::color::Color::BLUE))
        .draw(&mut e)
        .unwrap();

    e.display().unwrap();

    let mut rtc = Rtc::new(peripherals.LPWR);
    rtc.sleep_deep(&[], &mut delay)
}
