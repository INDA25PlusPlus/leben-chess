//! Types for representing chess pieces.

use PieceType::*;
use PlayerColor::*;

/// One of the standard chess piece types: Pawn, knight, bishop, rook, queen, king
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl PieceType {
    /// see: [Chess piece relative value - Wikipedia](https://en.wikipedia.org/wiki/Chess_piece_relative_value#Standard_valuations)
    ///
    /// returns: The standard valuation of the given piece type.
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

/// One of the piece colors: White or black
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerColor {
    White, Black
}

impl PlayerColor {
    /// returns: The other player's color
    pub fn other_player(&self) -> PlayerColor {
        match self {
            White => Black,
            Black => White,
        }
    }
}

/// Represents a piece on the chess board, with a given type and color.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: PlayerColor,
}

impl Piece {
    /// Gets a piece's FEN notation letter (pawn = "P", knight = "N", bishop = "B", rook = "R",
    /// queen = "Q", king = "K"), with white pieces represented with uppercase letters and black
    /// pieces with lowercase letters.
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


    /// see: [Chess symbols in Unicode - Wikipedia](https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode#Miscellaneous_symbols)
    ///
    /// returns: A piece's Unicode character
    pub fn get_unicode_char(&self) -> &'static str {
        match (self.piece_type, self.player) {
            (Pawn, White) => "♙",
            (Knight, White) => "♘",
            (Bishop, White) => "♗",
            (Rook, White) => "♖",
            (Queen, White) => "♕",
            (King, White) => "♔",
            (Pawn, Black) => "♟",
            (Knight, Black) => "♞",
            (Bishop, Black) => "♝",
            (Rook, Black) => "♜",
            (Queen, Black) => "♛",
            (King, Black) => "♚",
        }
    }

    /// Gets a [Piece] object given the corresponding FEN notation letter for the piece.
    ///
    /// returns: `Some(Piece)` if the character was parsed successfully, otherwise `None`.
    pub fn from_char(ch: char) -> Option<Piece> {
        match ch {
            'P' => Some(Piece { piece_type: Pawn, player: White }),
            'N' => Some(Piece { piece_type: Knight, player: White }),
            'B' => Some(Piece { piece_type: Bishop, player: White }),
            'R' => Some(Piece { piece_type: Rook, player: White }),
            'Q' => Some(Piece { piece_type: Queen, player: White }),
            'K' => Some(Piece { piece_type: King, player: White }),
            'p' => Some(Piece { piece_type: Pawn, player: Black }),
            'n' => Some(Piece { piece_type: Knight, player: Black }),
            'b' => Some(Piece { piece_type: Bishop, player: Black }),
            'r' => Some(Piece { piece_type: Rook, player: Black }),
            'q' => Some(Piece { piece_type: Queen, player: Black }),
            'k' => Some(Piece { piece_type: King, player: Black }),
            _ => None,
        }
    }
}
