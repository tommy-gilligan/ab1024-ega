use super::Epd;
use crate::color::Color;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::{raw::RawU4, PixelColor, Rgb888, RgbColor},
    prelude::{OriginDimensions, RawData, Size},
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
    type Error = super::error::Error<BUSY::Error, RST::Error, DC::Error, S::Error>;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.set_pixel(
                pixel.0.x.try_into().unwrap(),
                pixel.0.y.try_into().unwrap(),
                pixel.1,
            )?;
        }

        Ok(())
    }
}

impl PixelColor for super::color::Color {
    type Raw = RawU4;
}

impl From<RawU4> for Color {
    fn from(color: RawU4) -> Self {
        color.into_inner().try_into().unwrap()
    }
}

impl From<Rgb888> for Color {
    fn from(color: Rgb888) -> Self {
        let (_, display) = RGB_DISPLAY_PAIRS
            .into_iter()
            .min_by_key(|(rgb, _): &(Rgb888, Color)| {
                let r: u32 =
                    (<u8 as Into<u32>>::into(color.r())).abs_diff(<u8 as Into<u32>>::into(rgb.r()));
                let g: u32 =
                    (<u8 as Into<u32>>::into(color.g())).abs_diff(<u8 as Into<u32>>::into(rgb.g()));
                let b: u32 =
                    (<u8 as Into<u32>>::into(color.b())).abs_diff(<u8 as Into<u32>>::into(rgb.b()));
                r * r + g * g + b * b
            })
            .unwrap();

        display
    }
}

impl From<Color> for Rgb888 {
    fn from(color: Color) -> Self {
        let (display, _) = RGB_DISPLAY_PAIRS
            .into_iter()
            .find(|(_, c): &(Rgb888, Color)| *c == color)
            .unwrap();

        display
    }
}

const RGB_DISPLAY_PAIRS: [(Rgb888, Color); 7] = [
    (Rgb888::new(0x00, 0x00, 0x00), Color::BLACK),
    (Rgb888::new(0xFF, 0xFF, 0xFF), Color::WHITE),
    (Rgb888::new(0x10, 0xcb, 0x10), Color::GREEN),
    (Rgb888::new(0x20, 0x20, 0xff), Color::BLUE),
    (Rgb888::new(0xff, 0x30, 0x20), Color::RED),
    (Rgb888::new(0xff, 0xff, 0x50), Color::YELLOW),
    (Rgb888::new(0xf0, 0x70, 0x20), Color::ORANGE),
];
