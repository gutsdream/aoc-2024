use crate::Direction::{East, North, South, West};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

impl Direction {
    pub fn traversal(&self) -> (isize, isize) {
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}
