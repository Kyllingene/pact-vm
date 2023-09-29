use std::fmt::{Display, Debug};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum U3 {
   B000 = 0b000,
   B001 = 0b001,
   B010 = 0b010,
   B011 = 0b011,
   B100 = 0b100,
   B101 = 0b101,
   B110 = 0b110,
   B111 = 0b111,
}

impl From<u8> for U3 {
    fn from(value: u8) -> Self {
        match value & 0b0000_0111 {
            0b000 => Self::B000,
            0b001 => Self::B001,
            0b010 => Self::B010,
            0b011 => Self::B011,
            0b100 => Self::B100,
            0b101 => Self::B101,
            0b110 => Self::B110,
            0b111 => Self::B111,
            _ => unreachable!()
        }
    }
}

impl Debug for U3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Display for U3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum U4 {
    B0000 = 0b0000,
    B0001 = 0b0001,
    B0010 = 0b0010,
    B0011 = 0b0011,
    B0100 = 0b0100,
    B0101 = 0b0101,
    B0110 = 0b0110,
    B0111 = 0b0111,
    B1000 = 0b1000,
    B1001 = 0b1001,
    B1010 = 0b1010,
    B1011 = 0b1011,
    B1100 = 0b1100,
    B1101 = 0b1101,
    B1110 = 0b1110,
    B1111 = 0b1111,
}

impl From<u8> for U4 {
    fn from(value: u8) -> Self {
        match value & 0b0000_1111 {
            0b0000 => Self::B0000,
            0b0001 => Self::B0001,
            0b0010 => Self::B0010,
            0b0011 => Self::B0011,
            0b0100 => Self::B0100,
            0b0101 => Self::B0101,
            0b0110 => Self::B0110,
            0b0111 => Self::B0111,
            0b1000 => Self::B1000,
            0b1001 => Self::B1001,
            0b1010 => Self::B1010,
            0b1011 => Self::B1011,
            0b1100 => Self::B1100,
            0b1101 => Self::B1101,
            0b1110 => Self::B1110,
            0b1111 => Self::B1111,
            _ => unreachable!()
        }
    }
}

impl Debug for U4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Display for U4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
