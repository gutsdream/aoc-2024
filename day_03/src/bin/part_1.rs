use std::fs;
use day_03::part_1;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = part_1(input);

    println!("Part 1: {}", result?);

    Ok(())
}