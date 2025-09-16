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

pub(crate) fn get_en_passant_pos(active_player: PlayerColor,
                                 en_passant_target: BoardPosition) -> Option<BoardPosition>
{
    let offset = match active_player {
        PlayerColor::White => (0, -1),
        PlayerColor::Black => (0, 1),
    };
    en_passant_target.add(offset)
}

fn is_first_move_pawn(active_player: PlayerColor,
                      pos: BoardPosition) -> Option<(BoardPosition, BoardPosition)>
{
    match active_player {
        PlayerColor::White => if pos.rank.get() == 1 {
            Some((pos.add((0, 1)).unwrap(), pos.add((0, 2)).unwrap()))
        } else { None },
        PlayerColor::Black => if pos.rank.get() == 6 {
            Some((pos.add((0, -1)).unwrap(), pos.add((0, -2)).unwrap()))
        } else { None },
    }
}

fn add_en_passant_moves(board: &mut Board, active_player: PlayerColor, pos: BoardPosition,
                        en_passant_target: BoardPosition, bitmap: &mut BoardBitmap)
{
    // check that the target square is actually capturable by the pawn
    let capture_offsets = match active_player {
        PlayerColor::White => ((-1, 1), (1, 1)),
        PlayerColor::Black => ((-1, -1), (1, -1)),
    };
    let capture_squares = (
        pos.add(capture_offsets.0),
        pos.add(capture_offsets.1)
    );
    if Some(en_passant_target) != capture_squares.0 && Some(en_passant_target) != capture_squares.1 {
        return;
    }

    let en_passanted_pos = match get_en_passant_pos(active_player, en_passant_target) {
        Some(pos) => pos,
        None => return
    };

    // check for the special case where the captured pawn blocked check
    let moved_piece = board.get_piece(pos);
    let destination_piece = board.get_piece(en_passant_target);
    let en_passanted_piece = board.get_piece(en_passanted_pos);

    board.set_piece(pos, None);
    board.set_piece(en_passant_target, moved_piece);
    board.set_piece(en_passanted_pos, None);

    // if move is legal, add to bitmap
    if !is_in_check(board, active_player) {
        bitmap.set(en_passant_target, true);
    }

    // undo move
    board.set_piece(pos, moved_piece);
    board.set_piece(en_passant_target, destination_piece);
    board.set_piece(en_passanted_pos, en_passanted_piece);
}

fn add_castling_moves(board: &mut Board, active_player: PlayerColor,
                      castling_rights: CastlingRights, bitmap: &mut BoardBitmap)
{
    if is_in_check(&board, active_player) {
        return;
    }
    let mut add_on_side = |rook_pos: BoardPosition, king_moves_from: BoardPosition,
                           king_moves_to: BoardPosition, must_be_empty: &[BoardPosition],
                           passes_through: &[BoardPosition]|
    {
        let piece = if let Some(piece) = board.get_piece(rook_pos) {
            piece
        } else {
            return;
        };
        if !matches!(piece.piece_type, PieceType::Rook) { return; }
        for square in must_be_empty {
            if !matches!(board.get_piece(*square), None) { return; }
        }
        for square in passes_through {
            if leads_to_check(board, active_player,
                              PieceMovement { from: king_moves_from, to: *square })
            {
                return;
            }
        }
        bitmap.set(king_moves_to, true);
    };

    let rank = match active_player {
        PlayerColor::White => 0,
        PlayerColor::Black => 7,
    };
    let king_moves_from = BoardPosition::try_from((4, rank)).unwrap();
    if castling_rights.queenside {
        let rook_pos = BoardPosition::try_from((0, rank)).unwrap();
        let king_moves_to = BoardPosition::try_from((2, rank)).unwrap();
        let must_be_empty = &[
            BoardPosition::try_from((1, rank)).unwrap(),
            BoardPosition::try_from((2, rank)).unwrap(),
            BoardPosition::try_from((3, rank)).unwrap(),
        ];
        let passes_through = &[
            BoardPosition::try_from((2, rank)).unwrap(),
            BoardPosition::try_from((3, rank)).unwrap(),
        ];
        add_on_side(rook_pos, king_moves_from, king_moves_to, must_be_empty, passes_through);
    }
    if castling_rights.kingside {
        let rook_pos = BoardPosition::try_from((7, rank)).unwrap();
        let king_moves_to = BoardPosition::try_from((6, rank)).unwrap();
        let must_be_empty = &[
            BoardPosition::try_from((5, rank)).unwrap(),
            BoardPosition::try_from((6, rank)).unwrap(),
        ];
        let passes_through = &[
            BoardPosition::try_from((5, rank)).unwrap(),
            BoardPosition::try_from((6, rank)).unwrap(),
        ];
        add_on_side(rook_pos, king_moves_from, king_moves_to, must_be_empty, passes_through);
    }
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
