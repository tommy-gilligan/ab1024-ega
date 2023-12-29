#[allow(dead_code)]
pub(super) const BLACK: u8 = 0b00000000;
#[allow(dead_code)]
pub(super) const WHITE: u8 = 0b00000001;
#[allow(dead_code)]
pub(super) const GREEN: u8 = 0b00000010;
#[allow(dead_code)]
pub(super) const BLUE: u8 = 0b00000011;
#[allow(dead_code)]
pub(super) const RED: u8 = 0b00000100;
#[allow(dead_code)]
pub(super) const YELLOW: u8 = 0b00000101;
#[allow(dead_code)]
pub(super) const ORANGE: u8 = 0b00000110;

const RGB_DISPLAY_PAIRS: [(u32, u8); 7] = [
    (0x00000000, BLACK),
    (0x00FFFFFF, WHITE),
    (0x0000FF00, GREEN),
    (0x000000FF, BLUE),
    (0x00FF0000, RED),
    (0x00FFFF00, YELLOW),
    (0x00FF8000, ORANGE),
];

#[inline]
fn RED8(a: u32) -> i16 {
    (((a) >> 16) & 0xff) as i16
}
#[inline]
fn GREEN8(a: u32) -> i16 {
    (((a) >> 8) & 0xff) as i16
}
#[inline]
fn BLUE8(a: u32) -> i16 {
    (((a)) & 0xff) as i16
}
#[inline]
fn SQR(a: i16) -> i32 {
    (a as i32) * (a as i32)
}

pub fn closest(color: u32) -> u8 {
    let r = RED8(color);
    let g = GREEN8(color);
    let b = BLUE8(color);

    let (_, display) = RGB_DISPLAY_PAIRS.into_iter().min_by_key(|(rgb, _): &(u32, u8)| {
        SQR(r - RED8(*rgb)) + SQR(g - GREEN8(*rgb)) + SQR(b - BLUE8(*rgb))
    }).unwrap();

    display
}
