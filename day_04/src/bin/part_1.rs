use crate::Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};
use eyre::ContextCompat;
use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = solve(input);

    println!("Part 1: {}", result?);

    Ok(())
}

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

const DIRECTIONS: [Direction; 8] = [
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
];

impl Direction {
    fn traversal(&self) -> (i32, i32) {
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

fn solve(input: String) -> eyre::Result<i32> {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            let char = line.chars().nth(x).context("Char should exist in length")?;
            if char == 'X' {
                for direction in DIRECTIONS.iter() {
                    let search = search(x, y, &lines, &direction, 3);
                    if search == "MAS"{
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

fn search(x: usize, y: usize, lines: &Vec<String>, direction: &Direction, depth: i32) -> String{
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_x() -> eyre::Result<()> {
        // Given
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        // When
        let result = solve(input.to_string())?;

        // Then
        assert_eq!(result, 18);

        Ok(())
    }
}
