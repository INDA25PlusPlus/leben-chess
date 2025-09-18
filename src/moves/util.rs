//! Utility bitmap types used in the `moves` module.

use std::fmt::{Debug, Display, Formatter};
use crate::board::board_pos::BoardPosition;
use crate::util::U6;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
struct Bitmap64 {
    data: u64
}

impl Bitmap64 {
    fn all_zeros() -> Bitmap64 {
        Bitmap64::default()
    }

    fn all_ones() -> Bitmap64 {
        Bitmap64 {
            data: 0xffff_ffff_ffff_ffff
        }
    }

    fn get(&self, index: U6) -> bool {
        (self.data.rotate_right(index.get() as u32) & 0x1) == 1
    }

    fn set(&mut self, index: U6, value: bool) {
        if value {
            self.data |= 0x0000_0000_0000_0001u64.rotate_left(index.get() as u32);
        } else {
            self.data &= 0xffff_ffff_ffff_fffeu64.rotate_left(index.get() as u32);
        }
    }
}

impl Debug for Bitmap64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:064b}", self.data)
    }
}

/// Represents a mapping between a given chess board square and a boolean value.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct BoardBitmap {
    bitmap: Bitmap64
}

impl BoardBitmap {
    /// returns: A bitmap with `false` assigned to each square.
    pub fn all_zeros() -> BoardBitmap {
        BoardBitmap::default()
    }

    /// returns: A bitmap with `true` assigned to each square.
    pub fn all_ones() -> BoardBitmap {
        BoardBitmap {
            bitmap: Bitmap64::all_ones()
        }
    }

    /// returns: The boolean value which a given square maps to.
    pub fn get(&self, index: BoardPosition) -> bool {
        self.bitmap.get(index.into())
    }

    /// Sets the boolean value which a given square maps to.
    pub fn set(&mut self, index: BoardPosition, value: bool) {
        self.bitmap.set(index.into(), value)
    }

    /// returns: Whether each square is mapped to `false`.
    pub fn is_all_zeros(&self) -> bool {
        self.bitmap.data == 0x0000_0000_0000_0000
    }
}

impl Display for BoardBitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in (0u8..8).rev() {
            write!(f, "\n{}", rank + 1)?;
            for file in 0u8..8 {
                let pos = BoardPosition::try_from((file, rank)).unwrap().into();
                let value = self.bitmap.get(pos);
                write!(f, " {}", if value { "1" } else { "0" })?;
            }
        }
        write!(f, "\n  a b c d e f g h")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::U3;
    use super::*;

    const TEST_POSITION_SET: [BoardPosition; 8] = [
        BoardPosition { file: U3::new(3).unwrap(), rank: U3::new(5).unwrap()},
        BoardPosition { file: U3::new(7).unwrap(), rank: U3::new(4).unwrap()},
        BoardPosition { file: U3::new(2).unwrap(), rank: U3::new(7).unwrap()},
        BoardPosition { file: U3::new(1).unwrap(), rank: U3::new(7).unwrap()},
        BoardPosition { file: U3::new(0).unwrap(), rank: U3::new(3).unwrap()},
        BoardPosition { file: U3::new(3).unwrap(), rank: U3::new(0).unwrap()},
        BoardPosition { file: U3::new(5).unwrap(), rank: U3::new(1).unwrap()},
        BoardPosition { file: U3::new(5).unwrap(), rank: U3::new(0).unwrap()},
    ];

    #[test]
    fn board_bitmap_set_get() {
        let mut bitmap = BoardBitmap::all_zeros();
        for p in TEST_POSITION_SET {
            bitmap.set(p, true);
        }

        for i in 0..7 {
            for j in 0..7 {
                let pos = BoardPosition::try_from((i, j)).unwrap();
                let in_list = TEST_POSITION_SET.iter().find(|p| **p == pos).is_some();
                assert_eq!(bitmap.get(pos), in_list);
            }
        }
    }

    #[test]
    fn board_bitmap_display() {
        let mut bitmap = BoardBitmap::all_zeros();
        for p in TEST_POSITION_SET {
            bitmap.set(p, true);
        }

        let expected = concat!(
            "\n",
            "8 0 1 1 0 0 0 0 0\n",
            "7 0 0 0 0 0 0 0 0\n",
            "6 0 0 0 1 0 0 0 0\n",
            "5 0 0 0 0 0 0 0 1\n",
            "4 1 0 0 0 0 0 0 0\n",
            "3 0 0 0 0 0 0 0 0\n",
            "2 0 0 0 0 0 1 0 0\n",
            "1 0 0 0 1 0 1 0 0\n",
            "  a b c d e f g h",
        ).to_string();
        assert_eq!(format!("{}", bitmap), expected);
    }
}
