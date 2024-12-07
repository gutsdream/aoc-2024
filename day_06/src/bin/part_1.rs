use std::fs;
use std::str::FromStr;
use day_06::Puzzle;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let mut puzzle = Puzzle::from_str(&input).unwrap();

    println!("Part 1: {}", puzzle.distinct_positions_visited());

    Ok(())
}