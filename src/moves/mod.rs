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

impl TryFrom<((u8, u8), (u8, u8))> for PieceMovement {
    type Error = ();
    fn try_from(value: ((u8, u8), (u8, u8))) -> Result<Self, Self::Error> {
        Ok(PieceMovement {
            from: BoardPosition::try_from(value.0)?,
            to: BoardPosition::try_from(value.1)?
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ChessMove {
    pub piece_movement: PieceMovement,
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

fn leads_to_check(board: &mut Board, active_player: PlayerColor, piece_movement: PieceMovement) -> bool {
    let moved_piece = board.get_piece(piece_movement.from);
    let replaced_piece = board.get_piece(piece_movement.to);

    // test whether this move would put the active player in check
    board.set_piece(piece_movement.from, None);
    board.set_piece(piece_movement.to, moved_piece);
    let in_check = is_in_check(board, active_player);

    // undo move
    board.set_piece(piece_movement.from, moved_piece);
    board.set_piece(piece_movement.to, replaced_piece);

    in_check
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

    #[test]
    fn leads_to_check_test() {
        fn test_board(board: Board, active_player: PlayerColor, piece_movement: PieceMovement,
                      expected_value: bool
        ) {
            let mut cloned_board = board.clone();
            assert_eq!(leads_to_check(&mut cloned_board, active_player, piece_movement),
                       expected_value);
            assert_eq!(cloned_board, board);
        }

        test_board(Board::default_board(), PlayerColor::White,
                   PieceMovement::try_from(((3, 1), (3, 3))).unwrap(), false);
        test_board(Board::from_fen_string("rnbq1bnr/pppppppp/4k3/8/3P4/8/PPP1PPPP/RNBQKBNR")
                       .unwrap(), PlayerColor::Black,
                   PieceMovement::try_from(((4, 5), (4, 4))).unwrap(), true);
        test_board(Board::from_fen_string("8/2b1n3/3R1r2/4K3/6k1/8/8/8")
                       .unwrap(), PlayerColor::White,
                   PieceMovement::try_from(((3, 5), (5, 5))).unwrap(), true);
        test_board(Board::from_fen_string("8/2b1n3/2R2r2/4K3/6k1/8/8/8")
                       .unwrap(), PlayerColor::White,
                   PieceMovement::try_from(((2, 5), (5, 5))).unwrap(), true);
        test_board(Board::from_fen_string("8/2b1n3/2R2r2/4K3/6k1/8/8/8")
                       .unwrap(), PlayerColor::White,
                   PieceMovement::try_from(((2, 5), (3, 5))).unwrap(), false);
        test_board(Board::from_fen_string("8/2b1n3/2R2r2/4K3/5k2/8/8/8")
                       .unwrap(), PlayerColor::White,
                   PieceMovement::try_from(((2, 5), (3, 5))).unwrap(), true);
        test_board(Board::from_fen_string("8/2b1n3/2R2r2/4K3/5k2/8/8/8")
                       .unwrap(), PlayerColor::White,
                   PieceMovement::try_from(((2, 5), (3, 5))).unwrap(), true);
        test_board(Board::from_fen_string("8/2b1n3/3R1r2/4K3/5k2/8/8/8")
                       .unwrap(), PlayerColor::White,
                   PieceMovement::try_from(((0, 0), (0, 0))).unwrap(), true);
    }
}
