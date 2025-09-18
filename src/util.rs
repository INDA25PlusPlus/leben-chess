//! Utility integer types used in various other parts of the library.

use crate::board::board_pos::BoardPosition;

/// Contains a `u8` value with the invariant of always being in the `0b0000_0000` to `0b0000_0111`
/// range (inclusive).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct U3 { value: u8 }

impl U3 {
    /// returns: `Some(U3)` if value is in the range `0b0000_0000` to `0b0000_0111` (inclusive),
    /// otherwise `None`.
    pub const fn new(value: u8) -> Option<U3> {
        if value > 0b00000111 {
            None
        } else {
            Some(U3 { value })
        }
    }

    /// returns: The underlying `u8` value.
    pub const fn get(self) -> u8 {
        self.value
    }
}

impl Into<u8> for U3 {
    fn into(self) -> u8 {
        self.get()
    }
}

impl Into<usize> for U3 {
    fn into(self) -> usize {
        self.get() as usize
    }
}

impl TryFrom<u8> for U3 {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

/// Contains a `u8` value with the invariant of always being in the `0b0000_0000` to `0b0011_1111`
/// range (inclusive).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct U6 { value: u8 }

impl U6 {
    /// returns `Some(U6)` if value is in the range `0b0000_0000` to `0b0011_1111` (inclusive),
    /// otherwise `None`.
    pub const fn new(value: u8) -> Option<U6> {
        if value > 0b00111111 {
            None
        } else {
            Some(U6 { value })
        }
    }

    /// returns: The underlying `u8` value.
    pub const fn get(self) -> u8 {
        self.value
    }
}

impl Into<u8> for U6 {
    fn into(self) -> u8 {
        self.get()
    }
}

impl Into<usize> for U6 {
    fn into(self) -> usize {
        self.get() as usize
    }
}

impl TryFrom<u8> for U6 {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl Into<BoardPosition> for U6 {
    fn into(self) -> BoardPosition {
        let x: U3 = ((self.value >> 3) & 0b0000_0111).try_into().unwrap();
        let y: U3 = (self.value & 0b0000_0111).try_into().unwrap();
        BoardPosition { file: x, rank: y }
    }
}

impl From<BoardPosition> for U6 {
    fn from(board_pos: BoardPosition) -> Self {
        let (x, y): (u8, u8) = board_pos.into();
        U6 { value: (x << 3) | y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u3_test() {
        for i in 0x00..0xff {
            assert_eq!(matches!(U3::new(i), None), i > 7);
        }
    }

    #[test]
    fn u6_test() {
        for i in 0x00..0xff {
            assert_eq!(matches!(U6::new(i), None), i > 63);
        }
    }
}
