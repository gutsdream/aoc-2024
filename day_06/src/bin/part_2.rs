use day_06::Puzzle;
use std::fs;
use std::str::FromStr;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let puzzle = Puzzle::from_str(&input).unwrap();

    println!("Part 2: {}", puzzle.potential_loop_opportunities());

    Ok(())
}
