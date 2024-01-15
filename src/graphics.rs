use super::Epd;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::{PixelColor, raw::RawU4},
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
    type Color = super::color::Color;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.set_pixel(
                pixel.0.x.try_into().unwrap(),
                pixel.0.y.try_into().unwrap(),
                pixel.1.into()
            );
        }

        Ok(())
    }
}

impl PixelColor for super::color::Color {
    type Raw = RawU4;
}
