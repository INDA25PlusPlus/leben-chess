use crate::board::Board;
use crate::board::board_pos::BoardPosition;
use crate::board::piece::PlayerColor;
use crate::moves::CastlingRights;

#[derive(Copy, Clone, Debug)]
pub enum DrawReason {
    Stalemate,
    DrawByAgreement,
}

#[derive(Copy, Clone, Debug)]
pub enum WinReason {
    Checkmate,
    Resignation,
}

#[derive(Copy, Clone, Debug)]
pub enum GameStatus {
    NotYetStarted,
    Normal,
    Draw(DrawReason),
    Win(PlayerColor, WinReason),
}

#[derive(Clone, Debug)]
pub struct ChessGame {
    game_status: GameStatus,
    active_player: PlayerColor,

    board: Board,
    castling_rights: (CastlingRights, CastlingRights),
    en_passant_target: Option<BoardPosition>,
}
