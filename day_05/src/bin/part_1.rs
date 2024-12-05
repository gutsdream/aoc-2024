use std::fs;
use day_05::Puzzle;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let puzzle = Puzzle::from(input.as_str());

    println!("Part 1: {}", puzzle.sum_of_correct_updates());

    Ok(())
}