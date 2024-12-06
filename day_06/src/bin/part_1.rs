use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let puzzle = Puzzle::from(input.as_str());

    println!("Part 1: {}", puzzle.part_1());

    Ok(())
}