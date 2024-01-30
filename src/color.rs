use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Default, Debug, IntoPrimitive, Eq, Hash, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Color {
    #[default]
    BLACK = 0b0000_0000,
    WHITE = 0b0000_0001,
    GREEN = 0b0000_0010,
    BLUE = 0b0000_0011,
    RED = 0b0000_0100,
    YELLOW = 0b0000_0101,
    ORANGE = 0b0000_0110,
}
