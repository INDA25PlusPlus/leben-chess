use crate::board::{Board, OccupantState};
use crate::board::board_pos::{BoardPosition, BoardLineIterator, CaptureType};
use crate::board::piece::{Piece, PieceType, PlayerColor};
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

fn find_kings(board: &Board, active_player: PlayerColor) -> impl Iterator<Item=BoardPosition> {
    let own_king_predicate = move |piece: Piece|
        piece.player == active_player
        && matches!(piece.piece_type, PieceType::King);
    let square_predicate = move |(_, square): &(BoardPosition, Option<Piece>)|
        square.map_or(false, own_king_predicate);
    board.into_iter()
        .filter(square_predicate)
        .map(|(pos, _)| pos)
}

fn is_in_check(board: &Board, player: PlayerColor) -> bool {
    find_kings(board, player).any(|pos| {
        let king_check_board_lines = match player {
            PlayerColor::White => move_patterns::WHITE_KING_CHECK_BOARD_LINES,
            PlayerColor::Black => move_patterns::BLACK_KING_CHECK_BOARD_LINES,
        };
        for (piece_type, board_lines) in king_check_board_lines {
            // try to find enemy pieces of a certain type
            let mut iter = BoardLineIterator::new(pos, board_lines);
            while let Some(target_square) = iter.next() {
                // return true if target_square contains an enemy piece of the right type
                match board.get_occupant_state(target_square.position, player) {
                    OccupantState::Empty => continue,
                    OccupantState::Friendly => {}
                    OccupantState::Enemy => {
                        if matches!(
                            target_square.capture_type,
                            CaptureType::Normal | CaptureType::CaptureOnly
                        ) {
                            if let Some(piece) = board.get_piece(target_square.position) {
                                if piece.piece_type == *piece_type {
                                    return true;
                                }
                            }
                        }
                    }
                }
                iter.skip_line()
            }
        }
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_in_check_test() {
        assert_eq!(is_in_check(&Board::default_board(), PlayerColor::White), false);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "rnbqkbnr/ppp2ppp/4p3/1B1p4/4P1Q1/8/PPPP1PPP/RNB1K1NR"
        ).unwrap(), PlayerColor::Black), true);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "8/8/8/8/8/2Kk4/8/8"
        ).unwrap(), PlayerColor::White), true);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "8/8/8/8/8/2Kk4/8/8"
        ).unwrap(), PlayerColor::Black), true);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "1n3qrb/p3pppp/1np1k3/2KQ1P2/1pbr4/8/PPP1PPPP/NNR1B1RB"
        ).unwrap(), PlayerColor::White), false);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "1n3qrb/p3pppp/1np1k3/1K1Q1P2/1pbr4/8/PPP1PPPP/NNR1B1RB"
        ).unwrap(), PlayerColor::White), true);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "8/8/8/2kn4/8/2K5/8/8"
        ).unwrap(), PlayerColor::White), true);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "8/4n3/8/2k5/8/2K5/8/8"
        ).unwrap(), PlayerColor::White), false);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "8/8/2k5/8/2KN4/8/8/8"
        ).unwrap(), PlayerColor::Black), true);
        assert_eq!(is_in_check(&Board::from_fen_string(
            "8/8/2k5/8/2K5/8/4N3/8"
        ).unwrap(), PlayerColor::Black), false);
    }
}
