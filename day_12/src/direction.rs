use crate::direction::Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const STRAIGHT: [Direction; 4] = [North, East, South, West];
pub const DIAGONAL: [Direction; 4] = [NorthEast, SouthEast, SouthWest, NorthWest];

impl Direction {
    pub fn traversal(&self) -> (isize, isize) {
        match self {
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
        }
    }

    pub fn get_diagonal(vertical: &Self, horizontal: &Self) -> Option<Self>{
        match vertical {
            North => {match horizontal {
                East => Some(NorthEast),
                West => Some(NorthWest),
                _ => None
            }}
            South => {match horizontal {
                East => Some(SouthEast),
                West => Some(SouthWest),
                _ => None
            }}
            _ => None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn gen_in_direction(&self, direction: &Direction) -> Option<Self> {
        let traversal = direction.traversal();
        let x: usize = (self.x as isize + traversal.0).try_into().ok()?;
        let y: usize = (self.y as isize + traversal.1).try_into().ok()?;

        Some(Point { x, y })
    }
}