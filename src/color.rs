use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, PartialEq, Copy, Clone, TryFromPrimitive)]
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
