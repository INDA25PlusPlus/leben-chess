use crate::board::BoardPosition;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct U3 { value: u8 }

impl U3 {
    pub const fn new(value: u8) -> Option<U3> {
        if value > 0b00000111 {
            None
        } else {
            Some(U3 { value })
        }
    }

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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct U6 { value: u8 }

impl U6 {
    pub const fn new(value: u8) -> Option<U6> {
        if value > 0b00111111 {
            None
        } else {
            Some(U6 { value })
        }
    }

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
