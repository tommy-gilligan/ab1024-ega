use super::Epd;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb888, RgbColor},
    prelude::{OriginDimensions, Size},
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
        Size::new(
            super::WIDTH.try_into().unwrap(),
            super::HEIGHT.try_into().unwrap(),
        )
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

            let nibble = match pixel.1 {
                Rgb888::BLACK => super::color::BLACK,
                Rgb888::WHITE => super::color::WHITE,
                Rgb888::GREEN => super::color::GREEN,
                Rgb888::BLUE => super::color::BLUE,
                Rgb888::RED => super::color::RED,
                Rgb888::YELLOW => super::color::YELLOW,
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
