use crate::board::board_pos::{BoardLine, CaptureType};
use crate::board::piece::PieceType;

pub const WHITE_PAWN_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (0, 1), max_length: 1, capture_type: CaptureType::MoveOnly },
    BoardLine { offset: (1, 1), max_length: 1, capture_type: CaptureType::CaptureOnly },
    BoardLine { offset: (-1, 1), max_length: 1, capture_type: CaptureType::CaptureOnly },
];

pub const BLACK_PAWN_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (0, -1), max_length: 1, capture_type: CaptureType::MoveOnly },
    BoardLine { offset: (1, -1), max_length: 1, capture_type: CaptureType::CaptureOnly },
    BoardLine { offset: (-1, -1), max_length: 1, capture_type: CaptureType::CaptureOnly },
];

pub const ROOK_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, -1), max_length: 7, capture_type: CaptureType::Normal },
];

pub const KNIGHT_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-2, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-2, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (2, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (2, 1), max_length: 1, capture_type: CaptureType::Normal },
];

pub const BISHOP_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -1), max_length: 7, capture_type: CaptureType::Normal },
];

pub const QUEEN_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, -1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -1), max_length: 7, capture_type: CaptureType::Normal },
];

pub const KING_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 0), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 0), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -1), max_length: 1, capture_type: CaptureType::Normal },
];

pub const WHITE_KING_CHECK_BOARD_LINES: &[(PieceType, &[BoardLine])] = &[
    (PieceType::Pawn, BLACK_PAWN_BOARD_LINES),
    (PieceType::Rook, ROOK_BOARD_LINES),
    (PieceType::Knight, KNIGHT_BOARD_LINES),
    (PieceType::Bishop, BISHOP_BOARD_LINES),
    (PieceType::Queen, QUEEN_BOARD_LINES),
    (PieceType::King, KING_BOARD_LINES),
];

pub const BLACK_KING_CHECK_BOARD_LINES: &[(PieceType, &[BoardLine])] = &[
    (PieceType::Pawn, WHITE_PAWN_BOARD_LINES),
    (PieceType::Rook, ROOK_BOARD_LINES),
    (PieceType::Knight, KNIGHT_BOARD_LINES),
    (PieceType::Bishop, BISHOP_BOARD_LINES),
    (PieceType::Queen, QUEEN_BOARD_LINES),
    (PieceType::King, KING_BOARD_LINES),
];
