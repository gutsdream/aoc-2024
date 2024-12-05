use std::{fs};
use day_03::part_2;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = part_2(input);

    println!("Part 2: {}", result?);

    Ok(())
}