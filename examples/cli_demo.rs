use leben_chess::board::Board;
use leben_chess::board::board_pos::BoardPosition;
use leben_chess::board::piece::{Piece, PlayerColor};
use leben_chess::chess::{ChessGame, GameStatus};
use leben_chess::moves::{ChessMove, PieceMovement, PromotionType};

fn get_promotion_type(string: &str) -> Result<Option<PromotionType>, ()> {
    if string.len() == 0 {
        return Ok(None)
    } else if string.len() != 2 {
        return Err(())
    }
    let mut iter = string.chars();
    if iter.next() != Some('.') {
        return Err(())
    }
    if let Some(piece_char) = iter.next() {
        if let Some(piece) = Piece::from_char(piece_char) {
            if let Ok(promotion_type) = PromotionType::try_from(piece.piece_type) {
                return Ok(Some(promotion_type));
            }
        }
    }
    Err(())
}

fn main() {
    let mut game = ChessGame::new(Board::default_board());
    while matches!(game.game_status(), GameStatus::Normal | GameStatus::NotYetStarted) {
        let player = match game.active_player() {
            PlayerColor::White => "White",
            PlayerColor::Black => "Black",
        };
        println!("-----------------{}\n-----------------\n{} to play:", game.board(), player);
        let mut s = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut s) {
            continue;
        }
        let s = s.trim();
        match s {
            "!resign" => {
                let _ = game.resign();
            }
            "!draw" => {
                let _ = game.draw_by_agreement();
            }
            s => {
                if s.starts_with("!set ") {
                    if let Some(new_board) = Board::from_fen_string(&s[5..]) {
                        game = ChessGame::new(new_board);
                    }
                    continue;
                }
                if s.len() < 4 {
                    if s.starts_with("@") && s.len() == 3 {
                        let pos = BoardPosition::try_from(&s[1..3]);
                        if let Ok(pos) = pos {
                            println!("{}", game.available_moves(pos));
                        }
                    }
                    continue;
                }
                let from = match BoardPosition::try_from(&s[0..2]) {
                    Ok(pos) => pos,
                    Err(_) => continue,
                };
                let to = match BoardPosition::try_from(&s[2..4]) {
                    Ok(pos) => pos,
                    Err(_) => continue,
                };
                let promotion = match get_promotion_type(&s[4..]) {
                    Ok(promotion_type) => promotion_type,
                    Err(_) => continue,
                };
                let result = game.do_move(ChessMove {
                    piece_movement: PieceMovement { from, to }, promotion
                });
                if let Err(err) = result {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
    println!("{}\n{}", game.board(), game.game_status());
}
