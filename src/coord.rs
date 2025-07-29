use crossterm::event:: {KeyCode, KeyEvent};

#[derive(Copy, Clone, Debug , PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn key_to_direction(event: KeyEvent ) -> Option<Direction> {
    match event.code {
        KeyCode::Char('w') | KeyCode::Up |  KeyCode::Char(',') => Some(Direction::Up),
        KeyCode::Char('s') | KeyCode::Down | KeyCode::Char('o') => Some(Direction::Down),
        KeyCode::Char('a') | KeyCode::Left => Some(Direction::Left),
        KeyCode::Char('d') | KeyCode::Right | KeyCode::Char('e') => Some(Direction::Right),
        _ => None,

    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn to_the(&self, direction: Direction) -> Self{
        match direction  {
            Direction::Up => Self::new(self.row.saturating_sub(1), self.col),
            Direction::Down => Self::new(self.row + 1, self.col),
            Direction::Left => Self::new(self.row, self.col.saturating_sub(1)),
            Direction::Right => Self::new(self.row, self.col + 1),
        }
    }

    pub fn to(&self, target:Self) -> Self {
        if *self == target {
            return *self;

        }

        let col_diff = self.col as isize - target.col as isize;
        let row_diff = self.row as isize - target.row as isize;
        if col_diff.abs() > row_diff.abs() {
            if col_diff < 0 {
                self.to_the(Direction::Right)
            } else {
                self.to_the(Direction::Left)
            }
        } else {
            if row_diff <  0{
                self.to_the(Direction::Down)
            } else {
                self.to_the(Direction::Up)
            }
        }
    }
}