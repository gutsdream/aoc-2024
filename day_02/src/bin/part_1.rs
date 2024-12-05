use std::fs;
fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let safe_levels = day_02::get_safe_level_report_count(input);

    println!("Part 1: {}", safe_levels);

    Ok(())
}