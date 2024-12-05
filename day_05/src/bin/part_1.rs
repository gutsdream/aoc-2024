use day_04::part_1;
use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = part_1::solve(input);

    println!("Part 1: {}", result?);

    Ok(())
}