//! Functions and types for determining, querying and performing legal chess moves.

use crate::board::{Board, OccupantState};
use crate::board::board_pos::{BoardPosition, BoardLineIterator, CaptureType};
use crate::board::piece::{Piece, PieceType, PlayerColor};
use crate::chess::ChessError;
use crate::moves::util::BoardBitmap;

pub mod util;
mod move_patterns;

/// Represents a valid piece type which a pawn may promote to.
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

/// Represents the movement of a piece from one square to another, without any additional
/// information.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

/// Represents any chess move, which includes the movement from one square to another, and may
/// include a pawn promotion type (see [PromotionType]).
#[derive(Copy, Clone, Debug)]
pub struct ChessMove {
    pub piece_movement: PieceMovement,
    pub promotion: Option<PromotionType>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct CastlingRights {
    pub queenside: bool,
    pub kingside: bool,
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
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<BoardPosition>,
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

pub(crate) fn is_in_check(board: &Board, player: PlayerColor) -> bool {
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

fn leads_to_check(board: &mut Board, active_player: PlayerColor,
                  piece_movement: PieceMovement) -> bool
{
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

fn create_en_passant_target(active_player: PlayerColor,
                            piece_movement: PieceMovement) -> Option<BoardPosition>
{
    let pawn_start_rank = match active_player {
        PlayerColor::White => 1,
        PlayerColor::Black => 6,
    };
    let double_move_rank = match active_player {
        PlayerColor::White => 3,
        PlayerColor::Black => 4,
    };
    if piece_movement.from.rank.get() == pawn_start_rank
        && piece_movement.to.rank.get() == double_move_rank {
        let offset = match active_player {
            PlayerColor::White => (0, 1),
            PlayerColor::Black => (0, -1),
        };
        piece_movement.from.add(offset)
    } else {
        None
    }
}

fn get_en_passant_pos(active_player: PlayerColor,
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

pub(crate) fn get_available_moves(board: &mut Board, active_player: PlayerColor, pos: BoardPosition,
                                  move_context: MoveContext) -> BoardBitmap
{
    let mut bitmap = BoardBitmap::all_zeros();
    if let Some(piece) = board.get_piece(pos) {
        if piece.player != active_player { return bitmap; }
        let board_lines = move_patterns::get_board_lines(piece);
        let mut iter = BoardLineIterator::new(pos, board_lines);
        while let Some(target_square) = iter.next() {
            match board.get_occupant_state(target_square.position, active_player) {
                OccupantState::Empty => if matches!(
                    target_square.capture_type,
                    CaptureType::Normal | CaptureType::MoveOnly
                ) {
                    bitmap.set(target_square.position, true);
                },
                OccupantState::Friendly => {
                    iter.skip_line()
                },
                OccupantState::Enemy => if matches!(
                    target_square.capture_type,
                    CaptureType::Normal | CaptureType::CaptureOnly
                ) {
                    bitmap.set(target_square.position, true);
                    iter.skip_line();
                },
            }
        }
        match piece.piece_type {
            PieceType::Pawn => {
                if let Some(en_passant_target) = move_context.en_passant_target {
                    add_en_passant_moves(board, active_player, pos, en_passant_target, &mut bitmap);
                }
                if let Some((forward_move_pos, double_move_pos)) =
                    is_first_move_pawn(active_player, pos)
                {
                    let occupant_forward = board.get_occupant_state(
                        forward_move_pos,
                        active_player);
                    let occupant_double_move = board.get_occupant_state(
                        double_move_pos,
                        active_player);
                    match (occupant_forward, occupant_double_move) {
                        (OccupantState::Empty, OccupantState::Empty)
                            => bitmap.set(double_move_pos, true),
                        _ => {}
                    }
                }
            }
            PieceType::King => add_castling_moves(board, active_player,
                                                  move_context.castling_rights, &mut bitmap),
            _ => {}
        }
    } else {
        return bitmap;
    }
    for file in 0..8 {
        for rank in 0..8 {
            let move_to = BoardPosition::try_from((file, rank)).unwrap();
            if bitmap.get(move_to) {
                let leads_to_check = leads_to_check(
                    board, active_player,
                    PieceMovement {
                        from: pos,
                        to: move_to,
                    });
                if leads_to_check {
                    bitmap.set(move_to, false);
                }
            }
        }
    }
    bitmap
}

#[derive(Clone, Debug)]
pub(crate) struct MoveResult {
    pub captured_piece: Option<Piece>,
    pub new_en_passant_target: Option<BoardPosition>,
    pub removes_queenside_castling_rights: bool,
    pub removes_kingside_castling_rights: bool,
}

/// Performs a chess move without checking whether the move is legal, taking into consideration
/// en passant, castling and promotion rules.
///
/// returns: `Result<MoveResult, ChessError>`
pub(crate) fn do_move(board: &mut Board, active_player: PlayerColor, chess_move: ChessMove,
                      move_context: MoveContext) -> Result<MoveResult, ChessError>
{
    let mut result = MoveResult {
        captured_piece: None,
        new_en_passant_target: None,
        removes_queenside_castling_rights: false,
        removes_kingside_castling_rights: false,
    };
    if let Some(moved_piece) = board.get_piece(chess_move.piece_movement.from) {
        if !matches!(moved_piece.piece_type, PieceType::Pawn)
            && matches!(chess_move.promotion, Some(_))
        {
            return Err(ChessError::UnexpectedPromotionType);
        }
        let mut piece_after_move = moved_piece;
        result.captured_piece = board.get_piece(chess_move.piece_movement.to);
        match moved_piece.piece_type {
            PieceType::Pawn => {
                // double move creates en passant target
                result.new_en_passant_target = create_en_passant_target(active_player, chess_move.piece_movement);

                // promotion
                let promotion_rank = match active_player {
                    PlayerColor::White => 7,
                    PlayerColor::Black => 0,
                };
                if chess_move.piece_movement.to.rank.get() == promotion_rank {
                    if let Some(promotion) = chess_move.promotion {
                        piece_after_move = Piece {
                            piece_type: promotion.into(),
                            player: active_player,
                        };
                    } else {
                        return Err(ChessError::MissingPromotionType);
                    }
                } else {
                    if matches!(chess_move.promotion, Some(_)) {
                        return Err(ChessError::UnexpectedPromotionType);
                    }
                }

                // capture en passant
                if let Some(en_passant_target) = move_context.en_passant_target {
                    if chess_move.piece_movement.to == en_passant_target {
                        if let Some(en_passant_pos) = get_en_passant_pos(active_player,
                                                                         en_passant_target)
                        {
                            result.captured_piece = board.get_piece(en_passant_pos);
                            // at this point, if the function is gonna fail, it has already
                            // happened. therefore, we can safely mutate the board
                            board.set_piece(en_passant_pos, None);
                        }
                    }
                }
            }
            PieceType::King => {
                let rank = match active_player {
                    PlayerColor::White => 0,
                    PlayerColor::Black => 7,
                };
                let (queenside_move, kingside_move) = (
                    PieceMovement {
                        from: BoardPosition::try_from((4, rank)).unwrap(),
                        to: BoardPosition::try_from((2, rank)).unwrap(),
                    },
                    PieceMovement {
                        from: BoardPosition::try_from((4, rank)).unwrap(),
                        to: BoardPosition::try_from((6, rank)).unwrap(),
                    },
                );
                if chess_move.piece_movement == queenside_move {
                    let rook_from = BoardPosition::try_from((0, rank)).unwrap();
                    let rook_to = BoardPosition::try_from((3, rank)).unwrap();
                    let rook = board.get_piece(rook_from);
                    board.set_piece(rook_from, None);
                    board.set_piece(rook_to, rook);
                } else if chess_move.piece_movement == kingside_move {
                    let rook_from = BoardPosition::try_from((7, rank)).unwrap();
                    let rook_to = BoardPosition::try_from((5, rank)).unwrap();
                    let rook = board.get_piece(rook_from);
                    board.set_piece(rook_from, None);
                    board.set_piece(rook_to, rook);
                }
                result.removes_queenside_castling_rights = true;
                result.removes_kingside_castling_rights = true;
            }
            PieceType::Rook => {
                let rank = match active_player {
                    PlayerColor::White => 0,
                    PlayerColor::Black => 7,
                };
                if chess_move.piece_movement.from == BoardPosition::try_from((0, rank)).unwrap() {
                    result.removes_queenside_castling_rights;
                }
                if chess_move.piece_movement.from == BoardPosition::try_from((7, rank)).unwrap() {
                    result.removes_kingside_castling_rights;
                }
            }
            _ => {}
        }
        board.set_piece(chess_move.piece_movement.from, None);
        board.set_piece(chess_move.piece_movement.to, Some(piece_after_move));
    }
    Ok(result)
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
                      expected_value: bool)
        {
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

    #[test]
    fn get_available_moves_test() {
        fn test_board(mut board: Board, active_player: PlayerColor, pos: &str,
                      move_context: Option<MoveContext>, squares: &[&str])
        {
            let pos = BoardPosition::try_from(pos).unwrap();
            let move_context = move_context.unwrap_or(MoveContext {
                castling_rights: CastlingRights::default(),
                en_passant_target: None,
            });
            let mut bitmap = BoardBitmap::all_zeros();
            for square in squares {
                let square = BoardPosition::try_from(*square).unwrap();
                bitmap.set(BoardPosition::try_from(square).unwrap(), true);
            }
            let available_moves = get_available_moves(&mut board, active_player,
                                                      BoardPosition::try_from(pos).unwrap(),
                                                      move_context);
            assert_eq!(
                available_moves,
                bitmap,
                "piece: {},\nboard: {},\nexpected: {}\ngot: {}",
                pos,
                board,
                bitmap,
                available_moves,
            );
        }

        test_board(Board::default_board(), PlayerColor::White, "e1", None, &[]);

        // position r1bqk2r/pppp1ppp/5n2/4p3/1b2P3/2NP1Q1P/PPPB1PP1/R3KB1R
        let board_1 = Board::from_fen_string(
            "r1bqk2r/pppp1ppp/5n2/4p3/1b2P3/2NP1Q1P/PPPB1PP1/R3KB1R"
        ).unwrap();
        test_board(board_1.clone(), PlayerColor::White, "a1", None,
                   &["b1", "c1", "d1"],
        );
        test_board(board_1.clone(), PlayerColor::White, "e1", None,
                   &["c1", "d1", "e2"],
        );
        test_board(board_1.clone(), PlayerColor::White, "f1", None,
                   &["e2"],
        );
        test_board(board_1.clone(), PlayerColor::White, "h1", None,
                   &["g1", "h2"],
        );
        test_board(board_1.clone(), PlayerColor::White, "a2", None,
                   &["a3", "a4"],
        );
        test_board(board_1.clone(), PlayerColor::White, "b2", None,
                   &["b3"],
        );
        test_board(board_1.clone(), PlayerColor::White, "c2", None,
                   &[],
        );
        test_board(board_1.clone(), PlayerColor::White, "d2", None,
                   &["c1", "e3", "f4", "g5", "h6"],
        );
        test_board(board_1.clone(), PlayerColor::White, "f2", None,
                   &[],
        );
        test_board(board_1.clone(), PlayerColor::White, "g2", None,
                   &["g3", "g4"],
        );
        test_board(board_1.clone(), PlayerColor::White, "c3", None,
                   &["b1", "d1", "e2", "a4", "b5", "d5"],
        );
        test_board(board_1.clone(), PlayerColor::White, "d3", None,
                   &["d4"],
        );
        test_board(board_1.clone(), PlayerColor::White, "f3", None,
                   &["d1", "e2", "e3", "g3", "f4", "g4", "f5", "h5", "f6"],
        );
        test_board(board_1.clone(), PlayerColor::White, "h3", None,
                   &["h4"],
        );
        test_board(board_1.clone(), PlayerColor::White, "e4", None,
                   &[],
        );
        test_board(board_1.clone(), PlayerColor::Black, "b4", None,
                   &["a3", "c3", "a5", "c5", "d6", "e7", "f8"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "e5", None,
                   &[],
        );
        test_board(board_1.clone(), PlayerColor::Black, "f6", None,
                   &["e4", "g4", "d5", "h5", "g8"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "a7", None,
                   &["a6", "a5"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "b7", None,
                   &["b6", "b5"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "c7", None,
                   &["c6", "c5"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "d7", None,
                   &["d6", "d5"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "f7", None,
                   &[],
        );
        test_board(board_1.clone(), PlayerColor::Black, "g7", None,
                   &["g6", "g5"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "h7", None,
                   &["h6", "h5"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "a8", None,
                   &["b8"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "c8", None,
                   &[],
        );
        test_board(board_1.clone(), PlayerColor::Black, "d8", None,
                   &["e7"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "e8", None,
                   &["f8", "g8", "e7"],
        );
        test_board(board_1.clone(), PlayerColor::Black, "h8", None,
                   &["f8", "g8"],
        );

        // position r3k1nr/pppq1ppp/2n5/3pP3/3Pp3/2N5/PPPQ1PPP/R3KB1R
        let board_2 = Board::from_fen_string(
            "r3k1nr/pppq1ppp/2n5/3pP3/3Pp3/2N5/PPPQ1PPP/R3KB1R"
        ).unwrap();
        let context_2 = Some(MoveContext {
            castling_rights: CastlingRights::default(),
            en_passant_target: Some(BoardPosition::try_from("d6").unwrap()),
        });
        test_board(board_2.clone(), PlayerColor::White, "a1", context_2,
                   &["b1", "c1", "d1"],
        );
        test_board(board_2.clone(), PlayerColor::White, "e1", context_2,
                   &["c1", "d1", "e2"],
        );
        test_board(board_2.clone(), PlayerColor::White, "f1", context_2,
                   &["e2", "d3", "c4", "b5", "a6"],
        );
        test_board(board_2.clone(), PlayerColor::White, "h1", context_2,
                   &["g1"],
        );
        test_board(board_2.clone(), PlayerColor::White, "a2", context_2,
                   &["a3", "a4"],
        );
        test_board(board_2.clone(), PlayerColor::White, "b2", context_2,
                   &["b3", "b4"],
        );
        test_board(board_2.clone(), PlayerColor::White, "c2", context_2,
                   &[],
        );
        test_board(board_2.clone(), PlayerColor::White, "d2", context_2,
                   &["c1", "d1", "e2", "d3", "e3", "f4", "g5", "h6"],
        );
        test_board(board_2.clone(), PlayerColor::White, "f2", context_2,
                   &["f3", "f4"],
        );
        test_board(board_2.clone(), PlayerColor::White, "g2", context_2,
                   &["g3", "g4"],
        );
        test_board(board_2.clone(), PlayerColor::White, "h2", context_2,
                   &["h3", "h4"],
        );
        test_board(board_2.clone(), PlayerColor::White, "c3", context_2,
                   &["b1", "d1", "e2", "a4", "e4", "b5", "d5"],
        );
        test_board(board_2.clone(), PlayerColor::White, "d4", context_2,
                   &[],
        );
        test_board(board_2.clone(), PlayerColor::White, "e5", context_2,
                   &["d6", "e6"],
        );

        // en passant
        test_board(
            Board::from_fen_string("k7/8/8/8/8/4Pp2/8/K7").unwrap(),
            PlayerColor::Black, "f3", Some(MoveContext {
                castling_rights: Default::default(),
                en_passant_target: Some(BoardPosition::try_from("e2").unwrap()),
            }),
            &["e2", "f2"],
        );
        test_board(
            Board::from_fen_string("8/8/8/8/8/3RPpk1/8/K7").unwrap(),
            PlayerColor::Black, "f3", Some(MoveContext {
                castling_rights: Default::default(),
                en_passant_target: Some(BoardPosition::try_from("e2").unwrap()),
            }),
            &["f2"],
        );
        test_board(
            Board::from_fen_string("8/8/8/8/8/4Ppk1/6R1/K7").unwrap(),
            PlayerColor::Black, "f3", Some(MoveContext {
                castling_rights: Default::default(),
                en_passant_target: Some(BoardPosition::try_from("e2").unwrap()),
            }),
            &["g2"],
        );

        // normal castling
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R3K3").unwrap(),
            PlayerColor::White, "e1", Some(MoveContext {
                castling_rights: CastlingRights {
                    queenside: false,
                    kingside: false,
                },
                en_passant_target: None,
            }),
            &["d1", "d2", "e2", "f1", "f2"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R3K3").unwrap(),
            PlayerColor::White, "e1", Some(MoveContext {
                castling_rights: CastlingRights {
                    queenside: true,
                    kingside: false,
                },
                en_passant_target: None,
            }),
            &["c1", "d1", "d2", "e2", "f1", "f2"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R3K3").unwrap(),
            PlayerColor::White, "e1", Some(MoveContext {
                castling_rights: CastlingRights {
                    queenside: false,
                    kingside: true,
                },
                en_passant_target: None,
            }),
            &["d1", "d2", "e2", "f1", "f2"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R3K3").unwrap(),
            PlayerColor::White, "e1", Some(MoveContext {
                castling_rights: CastlingRights {
                    queenside: true,
                    kingside: true,
                },
                en_passant_target: None,
            }),
            &["c1", "d1", "d2", "e2", "f1", "f2"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R2QK2R").unwrap(),
            PlayerColor::White, "e1", None,
            &["d2", "e2", "f1", "f2", "g1"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R1B1K2R").unwrap(),
            PlayerColor::White, "e1", None,
            &["d1", "d2", "e2", "f1", "f2", "g1"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/RN2K2R").unwrap(),
            PlayerColor::White, "e1", None,
            &["d1", "d2", "e2", "f1", "f2", "g1"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R3KB1R").unwrap(),
            PlayerColor::White, "e1", None,
            &["c1", "d1", "d2", "e2", "f2"],
        );
        test_board(
            Board::from_fen_string("4k3/8/8/8/8/8/8/R3K1NR").unwrap(),
            PlayerColor::White, "e1", None,
            &["c1", "d1", "d2", "e2", "f1", "f2"],
        );

        // castle through check
        test_board(
            Board::from_fen_string("r3k3/8/8/8/8/8/8/K4R2").unwrap(),
            PlayerColor::Black, "e8", None,
            &["d8", "d7", "e7", "c8"],
        );
        test_board(
            Board::from_fen_string("r3k3/8/8/8/8/8/8/K3R3").unwrap(),
            PlayerColor::Black, "e8", None,
            &["d8", "d7", "f8", "f7"],
        );
        test_board(
            Board::from_fen_string("r3k3/8/8/8/8/8/8/K2R4").unwrap(),
            PlayerColor::Black, "e8", None,
            &["e7", "f7", "f8"],
        );
        test_board(
            Board::from_fen_string("r3k3/8/8/8/8/8/8/K1R5").unwrap(),
            PlayerColor::Black, "e8", None,
            &["d8", "d7", "e7", "f7", "f8"],
        );
        test_board(
            Board::from_fen_string("r3k3/8/8/8/8/8/8/KR6").unwrap(),
            PlayerColor::Black, "e8", None,
            &["d8", "d7", "e7", "f7", "f8", "c8"],
        );
    }

    #[test]
    fn do_move_test() {
        fn test_board(board_before: &str, board_after: &str, active_player: PlayerColor, from: &str,
                      to: &str, captured_piece_expected: Option<char>,
                      en_passant_target: Option<&str>, promotion: Option<PromotionType>)
        {
            let before = Board::from_fen_string(board_before).unwrap();
            let mut board = before.clone();
            let expected = Board::from_fen_string(board_after).unwrap();
            let piece_movement = PieceMovement {
                from: BoardPosition::try_from(from).unwrap(),
                to: BoardPosition::try_from(to).unwrap(),
            };
            let en_passant_target = en_passant_target.map(|s| BoardPosition::try_from(s).unwrap());
            let move_result = do_move(
                &mut board,
                active_player,
                ChessMove { piece_movement, promotion },
                MoveContext { castling_rights: CastlingRights::default(), en_passant_target }
            ).unwrap();
            let captured_piece = move_result.captured_piece;
            assert_eq!(
                board, expected,
                "from: {}, to: {},\nbefore: {},\nexpected: {},\ngot: {}",
                from,
                to,
                before,
                expected,
                board,
            );
            assert_eq!(
                captured_piece,
                captured_piece_expected.map(|s| Piece::from_char(s).unwrap()),
            );
        }

        test_board(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR",
            PlayerColor::White, "e2", "e4", None, None, None);
        test_board(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR",
            "rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR",
            PlayerColor::White, "e4", "d5", Some('p'), None, None);

        // en passant
        test_board(
            "r3k1nr/pppq1ppp/2n5/3pP3/3Pp3/2N5/PPPQ1PPP/R3KB1R",
            "r3k1nr/pppq1ppp/2nP4/8/3Pp3/2N5/PPPQ1PPP/R3KB1R",
            PlayerColor::White, "e5", "d6", Some('p'), Some("d6"), None);

        // promotion
        test_board(
            "8/k5P1/8/8/8/8/8/K7",
            "6N1/k7/8/8/8/8/8/K7",
            PlayerColor::White, "g7", "g8", None, None, Some(PromotionType::Knight));
        test_board(
            "8/k5P1/8/8/8/8/8/K7",
            "6B1/k7/8/8/8/8/8/K7",
            PlayerColor::White, "g7", "g8", None, None, Some(PromotionType::Bishop));
        test_board(
            "8/k5P1/8/8/8/8/8/K7",
            "6R1/k7/8/8/8/8/8/K7",
            PlayerColor::White, "g7", "g8", None, None, Some(PromotionType::Rook));
        test_board(
            "8/k5P1/8/8/8/8/8/K7",
            "6Q1/k7/8/8/8/8/8/K7",
            PlayerColor::White, "g7", "g8", None, None, Some(PromotionType::Queen));

        // castling
        test_board(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1",
            PlayerColor::White, "e1", "g1", None, None, None);
        test_board(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/2KR1BNR",
            PlayerColor::White, "e1", "c1", None, None, None);
        test_board(
            "rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "rnbq1rk1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            PlayerColor::Black, "e8", "g8", None, None, None);
        test_board(
            "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "2kr1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            PlayerColor::Black, "e8", "c8", None, None, None);
    }
}
