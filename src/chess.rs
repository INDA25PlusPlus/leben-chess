use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::board::Board;
use crate::board::board_pos::BoardPosition;
use crate::board::piece::PlayerColor;
use crate::moves;
use crate::moves::{CastlingRights, ChessMove, MoveContext, MoveResult};
use crate::moves::util::BoardBitmap;

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

impl Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            GameStatus::NotYetStarted => "Game not yet started",
            GameStatus::Normal => "Normal play",
            GameStatus::Draw(DrawReason::Stalemate) => "Draw by stalemate",
            GameStatus::Draw(DrawReason::DrawByAgreement) => "Draw by agreement",
            GameStatus::Win(PlayerColor::White, WinReason::Checkmate)
                => "White won by checkmate",
            GameStatus::Win(PlayerColor::White, WinReason::Resignation)
                => "White won by resignation",
            GameStatus::Win(PlayerColor::Black, WinReason::Checkmate)
                => "Black won by checkmate",
            GameStatus::Win(PlayerColor::Black, WinReason::Resignation)
                => "Black won by resignation",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
pub struct ChessGame {
    game_status: GameStatus,
    active_player: PlayerColor,

    board: Board,
    available_moves: [[BoardBitmap; 8]; 8],
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
        let mut game = ChessGame {
            game_status: GameStatus::NotYetStarted,
            active_player: PlayerColor::White,
            board: starting_board,
            available_moves: [[BoardBitmap::all_zeros(); 8]; 8],
            castling_rights: (CastlingRights::default(), CastlingRights::default()),
            en_passant_target: None,
        };
        game.recalculate_available_moves();
        game
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

    pub fn resign(&mut self) -> Result<(), ChessError> {
        match self.game_status {
            GameStatus::Normal => {
                self.game_status = GameStatus::Win(self.active_player.other_player(),
                                                   WinReason::Resignation);
                Ok(())
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

    fn recalculate_available_moves(&mut self) {
        for file in 0..8 {
            for rank in 0..8 {
                let pos = BoardPosition::try_from((file, rank)).unwrap();
                let move_context = self.move_context();
                let bitmap = moves::get_available_moves(&mut self.board, self.active_player, pos,
                                                        move_context);
                self.available_moves[file as usize][rank as usize] = bitmap;
            }
        }
    }

    pub fn available_moves(&mut self, pos: BoardPosition) -> BoardBitmap {
        self.available_moves[pos.file.get() as usize][pos.rank.get() as usize]
    }

    fn after_move(&mut self, move_result: MoveResult) {
        // determine en passant target
        self.en_passant_target = move_result.new_en_passant_target;

        // modify castling rights
        if move_result.removes_queenside_castling_rights {
            match self.active_player {
                PlayerColor::White => self.castling_rights.0.queenside = false,
                PlayerColor::Black => self.castling_rights.1.queenside = false,
            }
        }
        if move_result.removes_kingside_castling_rights {
            match self.active_player {
                PlayerColor::White => self.castling_rights.0.kingside = false,
                PlayerColor::Black => self.castling_rights.1.kingside = false,
            }
        }

        // change active player
        self.active_player = self.active_player.other_player();

        // recalculate available moves
        self.recalculate_available_moves();

        // determine game status
        let has_available_moves = self.available_moves.iter()
            .flatten()
            .any(|bitset| !bitset.is_all_zeros());
        if !has_available_moves {
            let check = moves::is_in_check(&self.board, self.active_player);
            if check {
                self.game_status = GameStatus::Win(self.active_player.other_player(),
                                                   WinReason::Checkmate);
            } else {
                self.game_status = GameStatus::Draw(DrawReason::Stalemate);
            }
        }
    }

    pub fn do_move(&mut self, chess_move: ChessMove) -> Result<(), ChessError> {
        match self.game_status {
            GameStatus::Normal => {}
            GameStatus::NotYetStarted => self.game_status = GameStatus::Normal,
            GameStatus::Draw(..) | GameStatus::Win(..) => return Err(ChessError::GameAlreadyEnded),
        }
        let available_moves = self.available_moves(chess_move.piece_movement.from);
        if !available_moves.get(chess_move.piece_movement.to) {
            return Err(ChessError::IllegalMove);
        }
        let move_context = self.move_context();
        let move_result = moves::do_move(&mut self.board, self.active_player, chess_move,
                                         move_context)?;
        self.after_move(move_result);
        Ok(())
    }
}
