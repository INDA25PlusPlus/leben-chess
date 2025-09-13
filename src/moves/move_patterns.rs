use crate::board::board_pos::{BoardLine, CaptureType};

pub const WHITE_PAWN_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (0, 1), max_length: 1, capture_type: CaptureType::MoveOnly },
    BoardLine { offset: (1, 1), max_length: 1, capture_type: CaptureType::CaptureOnly },
    BoardLine { offset: (-1, 1), max_length: 1, capture_type: CaptureType::CaptureOnly },
];

pub static BLACK_PAWN_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (0, -1), max_length: 1, capture_type: CaptureType::MoveOnly },
    BoardLine { offset: (1, -1), max_length: 1, capture_type: CaptureType::CaptureOnly },
    BoardLine { offset: (-1, -1), max_length: 1, capture_type: CaptureType::CaptureOnly },
];

pub static ROOK_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, -1), max_length: 7, capture_type: CaptureType::Normal },
];

pub static KNIGHT_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-2, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-2, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -2), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (2, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (2, 1), max_length: 1, capture_type: CaptureType::Normal },
];

pub static BISHOP_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -1), max_length: 7, capture_type: CaptureType::Normal },
];

pub static QUEEN_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 0), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, -1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -1), max_length: 7, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -1), max_length: 7, capture_type: CaptureType::Normal },
];

pub static KING_BOARD_LINES: &[BoardLine] = &[
    BoardLine { offset: (1, 0), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 0), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (0, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, 1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (-1, -1), max_length: 1, capture_type: CaptureType::Normal },
    BoardLine { offset: (1, -1), max_length: 1, capture_type: CaptureType::Normal },
];
