use super::Epd;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb888, RgbColor},
    prelude::{Dimensions, OriginDimensions, PixelColor, Point, Size},
    primitives::Rectangle,
    Pixel,
};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};
impl<D, S, RST, DC, BUSY> OriginDimensions for Epd<D, S, RST, DC, BUSY>
where
    D: DelayNs,
    S: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BUSY: InputPin,
{
    fn size(&self) -> Size {
        Size::new(super::WIDTH, super::HEIGHT)
    }
}

impl<D, S, RST, DC, BUSY> DrawTarget for Epd<D, S, RST, DC, BUSY>
where
    D: DelayNs,
    S: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BUSY: InputPin,
{
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let point = pixel.0;
            let [r, g, b] = [pixel.1.r(), pixel.1.g(), pixel.1.b()];

            let nibble = match (r, g, b) {
                (0x00, 0x00, 0x00) => super::color::BLACK,
                (0xff, 0xff, 0xff) => super::color::WHITE,
                (0x00, 0xff, 0x00) => super::color::GREEN,
                (0x00, 160, 0x00) => super::color::ORANGE,
                (0x00, 0x00, 0xff) => super::color::BLUE,
                (0xff, 0x00, 0x00) => super::color::RED,
                (0xff, 0xff, 0x00) => super::color::YELLOW,
                _ => super::color::ORANGE,
            };

            let index = (300usize * point.y as usize + (point.x as usize >> 1)).min(134399);

            if point.x % 2 == 0 {
                self.buffer[index] = (self.buffer[index] & 0x0f) | (nibble << 4);
            } else {
                self.buffer[index] = (self.buffer[index] & 0xf0) | nibble;
            }
        }

        Ok(())
    }
}
