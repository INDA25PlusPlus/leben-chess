use std::fmt::{Debug, Formatter};
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

#[derive(Clone, Eq, PartialEq)]
pub struct Board { squares: [[Option<Piece>; 8]; 8] }

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in (0u8..8).rev() {
            write!(f, "\n{}", rank + 1)?;
            for file in 0u8..8 {
                let pos = BoardPosition { file: file.try_into().unwrap(), rank: rank.try_into().unwrap() };
                let piece = self.get_piece(pos);
                if let Some(piece) = piece {
                    write!(f, " {}", piece.get_char())?;
                } else {
                    write!(f, "  ")?;
                }
            }
        }
        write!(f, "\n  a b c d e f g h")?;
        Ok(())
    }
}

impl Board {
    const EMPTY_BOARD: Board = Board {
        squares: [[None; 8]; 8]
    };

    const DEFAULT_BOARD: Board = Board {
        squares: [
            [
                Some(Piece { piece_type: Rook, player: White }),
                Some(Piece { piece_type: Knight, player: White }),
                Some(Piece { piece_type: Bishop, player: White }),
                Some(Piece { piece_type: Queen, player: White }),
                Some(Piece { piece_type: King, player: White }),
                Some(Piece { piece_type: Bishop, player: White }),
                Some(Piece { piece_type: Knight, player: White }),
                Some(Piece { piece_type: Rook, player: White }),
            ],
            [ Some(Piece { piece_type: Pawn, player: White }); 8 ],
            [ None, None, None, None, None, None, None, None ],
            [ None, None, None, None, None, None, None, None ],
            [ None, None, None, None, None, None, None, None ],
            [ None, None, None, None, None, None, None, None ],
            [ Some(Piece { piece_type: Pawn, player: Black }); 8 ],
            [
                Some(Piece { piece_type: Rook, player: Black }),
                Some(Piece { piece_type: Knight, player: Black }),
                Some(Piece { piece_type: Bishop, player: Black }),
                Some(Piece { piece_type: Queen, player: Black }),
                Some(Piece { piece_type: King, player: Black }),
                Some(Piece { piece_type: Bishop, player: Black }),
                Some(Piece { piece_type: Knight, player: Black }),
                Some(Piece { piece_type: Rook, player: Black }),
            ]
        ]
    };

    fn square_at(&self, pos: BoardPosition) -> &Option<Piece> {
        &self.squares[pos.file.get() as usize][pos.rank.get() as usize]
    }

    fn square_at_mut(&mut self, pos: BoardPosition) -> &mut Option<Piece> {
        &mut self.squares[pos.file.get() as usize][pos.rank.get() as usize]
    }

    pub fn get_piece(&self, pos: BoardPosition) -> Option<Piece> {
        *self.square_at(pos)
    }

    pub fn set_piece(&mut self, pos: BoardPosition, piece: Option<Piece>) {
        *self.square_at_mut(pos) = piece;
    }

    /// Instantiate a board from a 2D array of pieces, arranged first by file and then by rank
    pub fn from_array(squares: [[Option<Piece>; 8]; 8]) -> Board {
        Board { squares }
    }

    /// Instantiate an empty board
    pub fn empty_board() -> Board {
        Board::EMPTY_BOARD
    }

    /// Instantiate a board with the default chess piece configuration
    pub fn default_board() -> Board {
        Board::DEFAULT_BOARD
    }
}
