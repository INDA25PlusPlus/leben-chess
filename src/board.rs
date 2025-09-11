use crate::util::U3;

use PieceType::*;
use PlayerColor::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl PieceType {
    pub fn piece_value(&self) -> Option<u8> {
        match self {
            Pawn => Some(1),
            Knight => Some(3),
            Bishop => Some(3),
            Rook => Some(5),
            Queen => Some(9),
            King => None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerColor {
    White, Black
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: PlayerColor,
}

impl Piece {
    /// Gets a piece's FEN notation letter
    pub fn get_char(&self) -> &'static str {
        match (self.piece_type, self.player) {
            (Pawn, White) => "P",
            (Knight, White) => "N",
            (Bishop, White) => "B",
            (Rook, White) => "R",
            (Queen, White) => "Q",
            (King, White) => "K",
            (Pawn, Black) => "p",
            (Knight, Black) => "n",
            (Bishop, Black) => "b",
            (Rook, Black) => "r",
            (Queen, Black) => "q",
            (King, Black) => "k",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BoardPosition {
    pub file: U3,
    pub rank: U3
}

impl Into<(u8, u8)> for BoardPosition {
    fn into(self) -> (u8, u8) {
        (self.file.into(), self.rank.into())
    }
}

impl TryFrom<(u8, u8)> for BoardPosition {
    type Error = ();
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Ok(BoardPosition { file: value.0.try_into()?, rank: value.1.try_into()? })
    }
}
