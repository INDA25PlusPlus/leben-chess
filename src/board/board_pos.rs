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
        let mut board_line = None;
        while let Some(next) = self.lines.get(self.current_index) {
            if next.max_length > self.current_line_length {
                board_line = Some(next);
                break;
            }
            self.current_index += 1;
            self.current_line_length = 0;
        }
        let board_line = board_line?;
        let pos = self.origin.add((
              board_line.offset.0 * (self.current_line_length + 1) as i8,
              board_line.offset.1 * (self.current_line_length + 1) as i8,
        ))?;
        self.current_line_length += 1;
        Some(TargetSquare {
            position: pos,
            capture_type: board_line.capture_type,
        })
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
        let positions = iterator.for_each(|p| bitset.set(p.position, true));
        let mut expected_bitset = BoardBitmap::all_zeros();
        expected_bitset.set(BoardPosition::try_from((6, 4)).unwrap(), true);
        expected_bitset.set(BoardPosition::try_from((6, 5)).unwrap(), true);
        expected_bitset.set(BoardPosition::try_from((6, 6)).unwrap(), true);
        expected_bitset.set(BoardPosition::try_from((7, 4)).unwrap(), true);
        assert_eq!(bitset, expected_bitset, "Left:  {}\nRight: {}", bitset, expected_bitset);
    }
}
