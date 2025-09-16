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

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("game not started")]
    GameNotStarted,
    #[error("game has already ended")]
    GameAlreadyEnded,
    #[error("illegal move")]
    IllegalMove,
    #[error("it is the other player's turn")]
    WrongTurn,
    #[error("missing promotion type")]
    MissingPromotionType,
    #[error("expected `None` as promotion type: move is not a promotion move")]
    UnexpectedPromotionType,
}

impl ChessGame {
    pub fn new(starting_board: Board) -> ChessGame {
        ChessGame {
            game_status: GameStatus::NotYetStarted,
            active_player: PlayerColor::White,
            board: starting_board,
            castling_rights: (CastlingRights::default(), CastlingRights::default()),
            en_passant_target: None,
        }
    }

    pub fn game_status(&self) -> &GameStatus {
        &self.game_status
    }

    pub fn active_player(&self) -> PlayerColor {
        self.active_player
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn draw_by_agreement(&mut self) -> Result<(), ChessError> {
        match self.game_status {
            GameStatus::Normal => {
                self.game_status = GameStatus::Draw(DrawReason::DrawByAgreement);
                Ok(())
            }
            GameStatus::NotYetStarted => Err(ChessError::GameNotStarted),
            GameStatus::Draw(..) | GameStatus::Win(..) => Err(ChessError::GameAlreadyEnded),
        }
    }

    pub fn resign(&mut self, player: PlayerColor) -> Result<(), ChessError> {
        match self.game_status {
            GameStatus::Normal => {
                if self.active_player == player {
                    self.game_status = GameStatus::Win(player.other_player(),
                                                       WinReason::Resignation);
                    Ok(())
                } else {
                    Err(ChessError::WrongTurn)
                }
            }
            GameStatus::NotYetStarted => Err(ChessError::GameNotStarted),
            GameStatus::Draw(..) | GameStatus::Win(..) => Err(ChessError::GameAlreadyEnded),
        }
    }

    pub fn active_piece(&self, pos: BoardPosition) -> bool {
        if let Some(piece) = self.board.get_piece(pos) {
            self.active_player == piece.player
        } else {
            false
        }
    }

    fn castling_rights(&self, player: PlayerColor) -> CastlingRights {
        match player {
            PlayerColor::White => self.castling_rights.0,
            PlayerColor::Black => self.castling_rights.1,
        }
    }

    fn move_context(&self) -> MoveContext {
        MoveContext {
            castling_rights: self.castling_rights(self.active_player),
            en_passant_target: self.en_passant_target,
        }
    }
}
