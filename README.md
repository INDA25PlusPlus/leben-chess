# leben-chess

Chess game library written in Rust implementing all standard chess rules, including castling, en passant and pawn promotion.

Till dig som ska använda biblioteket i nästa veckas läxa: Skriv till mig om du hittar någon bugg så försöker jag fixa den 😊.

## Features

- Querying legal moves
- Automatic checkmate and stalemate detection
- Resignation and draw by agreement

### To do

- Move history
- Time control
- Track and list captured pieces
- Fifty move rule draw
- Three move repetition draw
- Convert game state to/from FEN string

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
leben-chess = { git = "https://github.com/INDA25PlusPlus/leben-chess.git", tag = "0.1.1" }
```

### Example

```rust
use leben_chess::board::Board;
use leben_chess::board::board_pos::BoardPosition;
use leben_chess::board::piece::PlayerColor;
use leben_chess::chess::{ChessError, ChessGame};
use leben_chess::moves::{ChessMove, PieceMovement};

fn main() -> Result<(), ChessError> {
    let mut game = ChessGame::new(Board::default_board());
    game.do_move(ChessMove {
        piece_movement: PieceMovement {
            from: BoardPosition::try_from("d2").unwrap(),
            to: BoardPosition::try_from("d4").unwrap()
        },
        promotion: None,
    })?;

    println!("{}", game.game_status());
    println!("{}", game.board());

    let bitmap = game.available_moves(BoardPosition::try_from("d7").unwrap());
    println!("{}", bitmap);

    game.resign()?;

    Ok(())
}
```

See the demo CLI program for more usages.

### Documentation

Run `cargo doc --open` to generate and open docs.

The main type exposed by the library is the `ChessGame` type, that contains all internal game logic. Call `ChessGame::available_moves` to retrieve a bitmap of all legal moves for a given piece, and `ChessGame::do_move` to perform a given move. Use `ChessGame::game_status` to query the current game status.

The library uses a number of utility types to interface with the `ChessGame` type. Mainly the `Board` type, which represents the internal board state. Use `Board::get_piece` to get a `Piece` object representing the piece at a given square.

## Demo program

This crate includes a small demo CLI program that can be run with `cargo run --example cli_demo`. Here's a list of commands that can be used in the demo:
- `<from><to>` - Performs the move that moves a piece from `<from>` to `<to>`. Example: `d2d4`.
- `<from><to>=<type>` - Performs the move that moves a pawn from `<from>` to `<to>` and promotes it to `<type>`. Example: `b7b8=q`.
- `@<square>` - Lists all squares that the piece at `<square>` can move to in a table. Example: `@b1`.
- `!resign` - Ends the game by resignation.
- `!draw` - Draws the game by agreement.
- `!set <fen-string>` - Reset the game, setting the board position to the specified FEN board position string. Example: `!set 1k4r1/3r4/8/8/8/8/4r2K/8`.
