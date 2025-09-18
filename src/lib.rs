//! Chess game library written in Rust implementing all standard chess rules, including castling, en
//! passant and pawn promotion.
//!
//! # Example
//!
//! ```rust
//! use leben_chess::board::Board;
//! use leben_chess::board::board_pos::BoardPosition;
//! use leben_chess::board::piece::PlayerColor;
//! use leben_chess::chess::{ChessError, ChessGame};
//! use leben_chess::moves::{ChessMove, PieceMovement};
//!
//! fn main() -> Result<(), ChessError> {
//!     let mut game = ChessGame::new(Board::default_board());
//!     game.do_move(ChessMove {
//!         piece_movement: PieceMovement {
//!             from: BoardPosition::try_from("d2").unwrap(),
//!             to: BoardPosition::try_from("d4").unwrap()
//!         },
//!         promotion: None,
//!     })?;
//!
//!     println!("{}", game.game_status());
//!     println!("{}", game.board());
//!
//!     let bitmap = game.available_moves(BoardPosition::try_from("d7").unwrap());
//!     println!("{}", bitmap);
//!
//!     game.resign()?;
//!
//!     Ok(())
//! }
//! ```

pub mod board;
pub mod chess;
pub mod moves;
pub mod util;
