use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let distance = day_01::calculate_total_distance(input.to_string());

    println!("Part 1: {}", distance);

    Ok(())
}