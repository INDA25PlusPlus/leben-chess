//! Functions and types for instantiating and playing a chess game. This module represents a more
//! high-level view of the chess rules than the [moves](crate::moves) module, enforcing chess rules.
//! The main type used for interfacing with the library API is the [ChessGame] type, which
//! represents the state of a game. Its main methods are detailed below:
//! - [board](ChessGame::board): Returns the [Board] object which represents the current board
//!   state.
//! - [available_moves](ChessGame::available_moves): Returns the set of all legal moves for a piece
//!   on a given square.
//! - [do_move](ChessGame::do_move): Performs a move, if it is legal. See [ChessError].
//! - [game_status](ChessGame::game_status): Returns the current [status](GameStatus) of the game.
//! - [active_player](ChessGame::active_player): Returns which player's turn it is.
//!
//! Also see [ChessGame::new] for creating a new [ChessGame] object.

use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::board::Board;
use crate::board::board_pos::BoardPosition;
use crate::board::piece::PlayerColor;
use crate::moves;
use crate::moves::{CastlingRights, ChessMove, MoveContext, MoveResult};
use crate::moves::util::BoardBitmap;

/// A valid reason for a chess game to end in a draw.
#[derive(Copy, Clone, Debug)]
pub enum DrawReason {
    Stalemate,
    DrawByAgreement,
}

/// A valid reason for a chess game to end in a win for either player.
#[derive(Copy, Clone, Debug)]
pub enum WinReason {
    Checkmate,
    Resignation,
}

/// The status of a given chess game.
#[derive(Copy, Clone, Debug)]
pub enum GameStatus {
    /// No player has made a move yet.
    NotYetStarted,
    /// The game is in normal play.
    Normal,
    /// The game has ended in draw.
    Draw(DrawReason),
    /// The game has ended in win for one of the players.
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

/// Represents a chess game played according to the standard chess rules. See
/// [the module documentation](self) for more information.
#[derive(Clone, Debug)]
pub struct ChessGame {
    game_status: GameStatus,
    active_player: PlayerColor,

    board: Board,
    available_moves: [[BoardBitmap; 8]; 8],
    castling_rights: (CastlingRights, CastlingRights),
    en_passant_target: Option<BoardPosition>,
}

/// An error caused by attempting to perform an illegal move or other invalid operation on a
/// [ChessGame] object.
#[derive(Error, Debug)]
pub enum ChessError {
    /// The game has not been started yet.
    #[error("game not started")]
    GameNotStarted,
    /// The game has already ended.
    #[error("game has already ended")]
    GameAlreadyEnded,
    /// An illegal move was attempted.
    #[error("illegal move")]
    IllegalMove,
    /// A move involving moving the other player's piece was attempted.
    #[error("it is the other player's turn")]
    WrongTurn,
    /// `None` was passed as promotion type, when the move was in fact a promotion move. See
    /// [do_move](ChessGame::do_move).
    #[error("missing promotion type")]
    MissingPromotionType,
    /// `Some(PromotionType` was passed, when the move was in fact not a promotion move. See
    /// [do_move](ChessGame::do_move).
    #[error("expected `None` as promotion type: move is not a promotion move")]
    UnexpectedPromotionType,
}

impl ChessGame {
    /// returns: A new [ChessGame] object with the given starting board configuration.
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

    /// returns: The current game status. See [GameStatus].
    pub fn game_status(&self) -> &GameStatus {
        &self.game_status
    }

    /// returns: Whose turn it is.
    pub fn active_player(&self) -> PlayerColor {
        self.active_player
    }

    /// returns: A [Board] object representing the current board state.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Ends the game by draw by agreement.
    ///
    /// returns: `Ok(())` if the game was successfully drawn.
    ///          [GameNotStarted](ChessError::GameNotStarted) if neither player has made a move yet
    ///          (the game may not be drawn at this point).
    ///          [GameAlreadyEnded](ChessError::GameAlreadyEnded) if the game is already ended by
    ///          draw or win.
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

    /// Ends the game by the active player resigning. A player may only resign on their turn.
    ///
    /// returns: `Ok(())` if the player successfully resigned.
    ///          [GameNotStarted](ChessError::GameNotStarted) if neither player has made a move yet
    ///          (the game may not be resigned at this point).
    ///          [GameAlreadyEnded](ChessError::GameAlreadyEnded) if the game is already ended by
    ///          draw or win.
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

    /// returns: Whether there is a piece on the given square that belongs to the active player.
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

    /// returns: A [BoardBitmap] representing the set of legal moves for the piece on a given
    /// square. Returns an empty bitmap ([BoardBitmap::all_zeros]) if there is no piece on the
    /// provided square, or if the piece has no legal moves.
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

    /// Performs a given chess move, if legal. Note that the [promotion](ChessMove) member of
    /// `chess_move` has to be set to `Some(PromotionType)` if the move involves a pawn promotion,
    /// and has to be set to `None` otherwise. A move involves a pawn promotion if and only if:
    /// - The piece being moves is a [pawn](crate::board::piece::PieceType), and
    /// - The piece is moved to its highest rank (rank 1 for white, and rank 7 for black)
    ///
    /// If the move is performed successfully, a set of actions are performed afterward:
    /// - En passant target is updated
    /// - Castling rights are updated (that is, removed if the king or a rook is moved)
    /// - The turn is given to the other player
    /// - The cache of available moves for each piece is updated
    /// - The game status is updated (checks for checkmate/stalemate)
    ///
    /// returns: `Ok(())` if the move was performed successfully, and `Err(ChessError)` otherwise.
    ///          See [ChessError].
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
