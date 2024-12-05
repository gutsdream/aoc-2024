use std::fs;
use day_04::part_2;
fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = part_2::solve(input);

    println!("Part 2: {}", result?);

    Ok(())
}