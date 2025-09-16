use std::fmt::{Display, Formatter};
use crate::util::U3;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BoardPosition {
    pub file: U3,
    pub rank: U3
}

impl Into<(u8, u8)> for BoardPosition {
    fn into(self) -> (u8, u8) {
        (self.file.into(), self.rank.into())
    }
}

impl TryFrom<(u8, u8)> for BoardPosition {
    type Error = ();
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Ok(BoardPosition { file: value.0.try_into()?, rank: value.1.try_into()? })
    }
}

impl TryFrom<&str> for BoardPosition {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.as_bytes();
        if value.len() != 2 { return Err(()); }
        let file = match value[0] {
            b'a' | b'A' => 0,
            b'b' | b'B' => 1,
            b'c' | b'C' => 2,
            b'd' | b'D' => 3,
            b'e' | b'E' => 4,
            b'f' | b'F' => 5,
            b'g' | b'G' => 6,
            b'h' | b'H' => 7,
            _ => return Err(()),
        };
        let rank = if let Some(rank) = (value[1] as char).to_digit(10)
            { rank } else { return Err(()); };
        let rank = if rank > 0 { rank - 1 } else { return Err(()); };
        BoardPosition::try_from((file, rank as u8))
    }
}

impl Display for BoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let files = b"abcdefgh";
        let ranks = b"12345678";
        let file = files[self.file.get() as usize] as char;
        let rank = ranks[self.rank.get() as usize] as char;
        write!(f, "{}{}", file, rank)
    }
}

impl BoardPosition {
    pub fn add(&self, offset: (i8, i8)) -> Option<BoardPosition> {
        let file = self.file.get() as i8 + offset.0;
        let rank = self.rank.get() as i8 + offset.1;
        if file < 0 || rank < 0 {
            None
        } else {
            BoardPosition::try_from((file as u8, rank as u8)).ok()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum CaptureType {
    Normal,
    MoveOnly,
    CaptureOnly,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct TargetSquare {
    pub position: BoardPosition,
    pub capture_type: CaptureType,
}

#[derive(Clone, Debug)]
pub(crate) struct BoardLine {
    pub offset: (i8, i8),
    pub max_length: usize,
    pub capture_type: CaptureType,
}

#[derive(Clone, Debug)]
pub(crate) struct BoardLineIterator<'a> {
    origin: BoardPosition,
    lines: &'a [BoardLine],
    current_index: usize,
    current_line_length: usize,
}

impl<'a> Iterator for BoardLineIterator<'a> {
    type Item = TargetSquare;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current_line) = self.lines.get(self.current_index) {
            if self.current_line_length >= current_line.max_length {
                self.current_index += 1;
                self.current_line_length = 0;
                continue;
            }
            let pos = match self.origin.add((
                current_line.offset.0 * (self.current_line_length + 1) as i8,
                current_line.offset.1 * (self.current_line_length + 1) as i8,
            )) {
                Some(pos) => pos,
                None => {
                    // if already outside board, no other squares on this line can be inside the
                    // board. skip to next line
                    self.current_index += 1;
                    self.current_line_length = 0;
                    continue;
                },
            };
            self.current_line_length += 1;
            return Some(TargetSquare {
                position: pos,
                capture_type: current_line.capture_type,
            });
        }
        None
    }
}

impl<'a> BoardLineIterator<'a> {
    pub fn new(origin: BoardPosition, lines: &'a [BoardLine]) -> BoardLineIterator<'a> {
        BoardLineIterator {
            origin,
            lines,
            current_index: 0,
            current_line_length: 0,
        }
    }

    pub fn skip_line(&mut self) {
        self.current_index += 1;
        self.current_line_length = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::moves::util::BoardBitmap;
    use super::*;

    #[test]
    fn board_pos_math() {
        let a = BoardPosition::try_from((2, 1)).unwrap();
        let b = BoardPosition::try_from((1, 7)).unwrap();

        assert_eq!(a.add((-2, 0)), Some(BoardPosition::try_from((0, 1)).unwrap()));
        assert_eq!(b.add((6,0)), Some(BoardPosition::try_from((7, 7)).unwrap()));
        assert_eq!(b.add((3, 2)), None);
    }

    #[test]
    fn target_square_iterator() {
        let iterator = BoardLineIterator::new(
            BoardPosition::try_from((6, 3)).unwrap(),
            &[
                BoardLine {
                    offset: (0, 1),
                    max_length: 3,
                    capture_type: CaptureType::Normal,
                },
                BoardLine {
                    offset: (1, 1),
                    max_length: 3,
                    capture_type: CaptureType::Normal,
                },
            ]
        );
        let mut bitset = BoardBitmap::all_zeros();
        iterator.for_each(|p| bitset.set(p.position, true));
        let mut expected_bitset = BoardBitmap::all_zeros();
        expected_bitset.set(BoardPosition::try_from((6, 4)).unwrap(), true);
        expected_bitset.set(BoardPosition::try_from((6, 5)).unwrap(), true);
        expected_bitset.set(BoardPosition::try_from((6, 6)).unwrap(), true);
        expected_bitset.set(BoardPosition::try_from((7, 4)).unwrap(), true);
        assert_eq!(bitset, expected_bitset, "Left:  {}\nRight: {}", bitset, expected_bitset);
    }
}
