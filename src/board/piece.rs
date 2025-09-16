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

impl PlayerColor {
    pub fn other_player(&self) -> PlayerColor {
        match self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: PlayerColor,
}

impl Piece {
    /// Gets a piece's FEN notation letter
    pub(crate) fn get_char(&self) -> &'static str {
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
