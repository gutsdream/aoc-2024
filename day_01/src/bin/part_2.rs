use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let similarity_score = day_01::calculate_similarity_score(input.to_string());

    println!("Part 2: {}", similarity_score);

    Ok(())
}
