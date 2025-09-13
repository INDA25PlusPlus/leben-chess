pub mod piece;
pub mod board_pos;

use std::fmt::{Display, Formatter};
use crate::board::board_pos::BoardPosition;
use crate::board::piece::{Piece, PieceType::*, PlayerColor::*, PlayerColor};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board { squares: [[Option<Piece>; 8]; 8] }

impl Display for Board {
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

#[derive(Copy, Clone, Debug)]
pub enum OccupantState {
    Empty,
    Friendly,
    Enemy,
}

impl Board {
    const EMPTY_BOARD: Board = Board {
        squares: [[None; 8]; 8]
    };

    const DEFAULT_BOARD: Board = Board {
        squares: [
            [
                Some(Piece { piece_type: Rook, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Rook, player: Black }),
            ],
            [
                Some(Piece { piece_type: Knight, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Knight, player: Black }),
            ],
            [
                Some(Piece { piece_type: Bishop, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Bishop, player: Black }),
            ],
            [
                Some(Piece { piece_type: Queen, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Queen, player: Black }),
            ],
            [
                Some(Piece { piece_type: King, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: King, player: Black }),
            ],
            [
                Some(Piece { piece_type: Bishop, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Bishop, player: Black }),
            ],
            [
                Some(Piece { piece_type: Knight, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Knight, player: Black }),
            ],
            [
                Some(Piece { piece_type: Rook, player: White }),
                Some(Piece { piece_type: Pawn, player: White }),
                None, None, None, None,
                Some(Piece { piece_type: Pawn, player: Black }),
                Some(Piece { piece_type: Rook, player: Black }),
            ],
        ]
    };

    const fn square_at(&self, pos: BoardPosition) -> &Option<Piece> {
        &self.squares[pos.file.get() as usize][pos.rank.get() as usize]
    }

    const fn square_at_mut(&mut self, pos: BoardPosition) -> &mut Option<Piece> {
        &mut self.squares[pos.file.get() as usize][pos.rank.get() as usize]
    }

    pub fn get_piece(&self, pos: BoardPosition) -> Option<Piece> {
        *self.square_at(pos)
    }

    pub fn set_piece(&mut self, pos: BoardPosition, piece: Option<Piece>) {
        *self.square_at_mut(pos) = piece;
    }

    pub fn get_occupant_state(&self, pos: BoardPosition, active_player: PlayerColor) -> OccupantState {
        match self.get_piece(pos) {
            None => OccupantState::Empty,
            Some(piece) => if piece.player == active_player {
                OccupantState::Friendly
            } else {
                OccupantState::Enemy
            }
        }
    }

    /// Instantiate a board from a 2D array of pieces, arranged first by file and then by rank
    pub const fn from_array(squares: [[Option<Piece>; 8]; 8]) -> Board {
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

#[derive(Copy, Clone, Debug)]
pub struct BoardIterator<'a> {
    board: &'a Board,
    file: u8,
    rank: u8,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (BoardPosition, Option<Piece>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rank > 7 {
            return None;
        }
        let pos = BoardPosition::try_from((self.file, self.rank)).unwrap();
        let piece = self.board.get_piece(pos);
        self.file += 1;
        if self.file > 7 {
            self.file = 0;
            self.rank += 1;
        }
        Some((pos, piece))
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = <BoardIterator<'a> as Iterator>::Item;
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator { board: self, file: 0, rank: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_display() {
        let board = Board::default_board();
        let expected = concat!(
            "\n",
            "8 r n b q k b n r\n",
            "7 p p p p p p p p\n",
            "6                \n",
            "5                \n",
            "4                \n",
            "3                \n",
            "2 P P P P P P P P\n",
            "1 R N B Q K B N R\n",
            "  a b c d e f g h"
        ).to_string();
        assert_eq!(format!("{}", board), expected);
    }

    #[test]
    fn board_iter() {
        let board = Board::default_board();
        let pieces: Vec<Option<Piece>> = board
            .into_iter()
            .take(20)
            .skip(6)
            .map(|(pos, piece)| piece)
            .collect();
        let expected = vec![
            Some(Piece { piece_type: Knight, player: White }),
            Some(Piece { piece_type: Rook, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            Some(Piece { piece_type: Pawn, player: White }),
            None,
            None,
            None,
            None,
        ];
        assert_eq!(pieces, expected);
    }
}
