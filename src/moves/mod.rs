use crate::board::board_pos::BoardPosition;
use crate::board::piece::PieceType;
use crate::moves::util::BoardBitmap;

pub mod util;
mod move_patterns;

#[derive(Copy, Clone, Debug)]
pub enum PromotionType {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl Into<PieceType> for PromotionType {
    fn into(self) -> PieceType {
        match self {
            PromotionType::Knight => PieceType::Knight,
            PromotionType::Bishop => PieceType::Bishop,
            PromotionType::Rook => PieceType::Rook,
            PromotionType::Queen => PieceType::Queen,
        }
    }
}

impl TryFrom<PieceType> for PromotionType {
    type Error = ();
    fn try_from(value: PieceType) -> Result<Self, Self::Error> {
        match value {
            PieceType::Pawn => Err(()),
            PieceType::Knight => Ok(PromotionType::Knight),
            PieceType::Bishop => Ok(PromotionType::Bishop),
            PieceType::Rook => Ok(PromotionType::Rook),
            PieceType::Queen => Ok(PromotionType::Queen),
            PieceType::King => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PieceMovement {
    pub from: BoardPosition,
    pub to: BoardPosition,
}

#[derive(Copy, Clone, Debug)]
pub struct ChessMove {
    pub chess_move: PieceMovement,
    pub promotion: Option<PromotionType>,
}

#[derive(Clone, Debug)]
pub enum AvailableMovesResult {
    Ok(BoardBitmap),
    Stalemate,
    Checkmate,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct CastlingRights {
    queenside: bool,
    kingside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            queenside: true,
            kingside: true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct MoveContext {
    castling_rights: CastlingRights,
    en_passant_target: Option<BoardPosition>,
}
