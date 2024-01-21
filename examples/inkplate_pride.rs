#![no_std]
#![no_main]

use dither::DitherTarget;
use embedded_graphics::pixelcolor::WebColors;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
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
use tinybmp::Bmp;

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

    let _bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("starry-night.bmp")).unwrap();
    let mut e = ab1024_ega::Epd::new(spi, rst, dc, busy, delay);
    e.init().unwrap();

    let mut ed: DitherTarget<'_, _, { ab1024_ega::WIDTH }, { ab1024_ega::WIDTH + 1 }> =
        DitherTarget::new(&mut e);

    let colors = [
        WebColors::CSS_RED,
        WebColors::CSS_ORANGE,
        WebColors::CSS_YELLOW,
        WebColors::CSS_GREEN,
        WebColors::CSS_BLUE,
        WebColors::CSS_PURPLE,
    ];
    let stripe_width = ab1024_ega::WIDTH / colors.len();

    for (index, color) in colors.into_iter().enumerate() {
        let style = PrimitiveStyleBuilder::new().fill_color(color).build();
        Rectangle::with_corners(
            Point::new((index * stripe_width).try_into().unwrap(), 0),
            Point::new(
                ab1024_ega::WIDTH.try_into().unwrap(),
                ab1024_ega::HEIGHT.try_into().unwrap(),
            ),
        )
        .into_styled(style)
        .draw(&mut ed)
        .unwrap();
    }
    e.display().unwrap();

    let mut rtc = Rtc::new(peripherals.LPWR);
    rtc.sleep_deep(&[], &mut delay)
}
