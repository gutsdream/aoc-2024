use std::fs;
use std::str::FromStr;
use day_14::{Point, Puzzle};

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let puzzle = Puzzle::from_str(&input).unwrap();

    println!("Part 2: {}", puzzle.part_2(Point::new(100, 102)));

    Ok(())
}