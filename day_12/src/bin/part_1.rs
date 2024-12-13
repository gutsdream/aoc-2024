use std::fs;
use std::str::FromStr;
use day_12::Puzzle;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let puzzle = Puzzle::from_str(&input).unwrap();

    println!("Part 1: {}", puzzle.part_1());

    Ok(())
}