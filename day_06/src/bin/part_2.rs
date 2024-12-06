use std::fs;
use day_05::Puzzle;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = Puzzle::from(input.as_str()).sum_of_incorrect_updates();

    println!("Part 2: {}", result);

    Ok(())
}