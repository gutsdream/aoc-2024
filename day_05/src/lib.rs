use crate::Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};

pub mod part_1;
pub mod part_2;

#[derive(Debug)]
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

pub const DIRECTIONS: [Direction; 8] = [
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
];

impl Direction {
    pub fn traversal(&self) -> (i32, i32) {
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
}

pub fn search(x: usize, y: usize, lines: &Vec<String>, direction: &Direction, depth: i32) -> String{
    let word = (0..depth)
        .enumerate()
        .filter_map(|(i, _)| {
            let i = i + 1;
            let traversal = direction.traversal();
            let next_y : usize = (y as i32 + (traversal.1 * i as i32)).try_into().ok()?;
            let next_x : usize = (x as i32 + (traversal.0 * i as i32)).try_into().ok()?;

            let next_line = lines.iter().nth(next_y)?;
            let next_char = next_line.chars().nth(next_x)?;

            Some(next_char)
        })
        .collect::<String>();

    word
}