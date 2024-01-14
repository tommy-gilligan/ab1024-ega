use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor};
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(u8)]
pub enum Color {
    BLACK = 0b00000000,
    WHITE = 0b00000001,
    GREEN = 0b00000010,
    BLUE = 0b00000011,
    RED = 0b00000100,
    YELLOW = 0b00000101,
    ORANGE = 0b00000110,
}

const RGB_DISPLAY_PAIRS: [(u32, Color); 7] = [
    (0x00000000, Color::BLACK),
    (0x00FFFFFF, Color::WHITE),
    (0x0010cB10, Color::GREEN),
    (0x002020FF, Color::BLUE),
    (0x00ff3020, Color::RED),
    (0x00ffff50, Color::YELLOW),
    (0x00f07020, Color::ORANGE),
];

const RGB_DISPLAY_PAIRS_RGB: [(Rgb888, Color); 7] = [
    (Rgb888::new(0x00, 0x00, 0x00), Color::BLACK),
    (Rgb888::new(0xFF, 0xFF, 0xFF), Color::WHITE),
    (Rgb888::new(0x10, 0xcb, 0x10), Color::GREEN),
    (Rgb888::new(0x20, 0x20, 0xff), Color::BLUE),
    (Rgb888::new(0xff, 0x30, 0x20), Color::RED),
    (Rgb888::new(0xff, 0xff, 0x50), Color::YELLOW),
    (Rgb888::new(0xf0, 0x70, 0x20), Color::ORANGE),
];

pub fn closest(color: Rgb888) -> Color {
    let (_, display) = RGB_DISPLAY_PAIRS
        .into_iter()
        .min_by_key(|(rgb, _): &(u32, Color)| {
            let r = (color.r() as u32).abs_diff((((*rgb) >> 16) & 0xff));
            let g = (color.g() as u32).abs_diff((((*rgb) >> 8) & 0xff));
            let b = (color.b() as u32).abs_diff(((*rgb) & 0xff));
            r * r + g * g + b * b
        })
        .unwrap();

    display
}

pub fn closestrgb(color: Rgb888) -> Rgb888 {
    let (display, _) = RGB_DISPLAY_PAIRS_RGB
        .into_iter()
        .min_by_key(|(rgb, _): &(Rgb888, Color)| {
            let r = (color.r() as u32).abs_diff(rgb.r() as u32);
            let g = (color.g() as u32).abs_diff(rgb.g() as u32);
            let b = (color.b() as u32).abs_diff(rgb.b() as u32);
            r * r + g * g + b * b
        })
        .unwrap();

    display
}
