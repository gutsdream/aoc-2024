use day_04::{search, DIRECTIONS};
use eyre::ContextCompat;
use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = solve(input);

    println!("Part 1: {}", result?);

    Ok(())
}


fn solve(input: String) -> eyre::Result<i32> {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            let char = line.chars().nth(x).context("Char should exist in length")?;
            if char == 'X' {
                for direction in DIRECTIONS.iter() {
                    let search = search(x, y, &lines, &direction, 3);
                    if search == "MAS"{
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_x() -> eyre::Result<()> {
        // Given
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        // When
        let result = solve(input.to_string())?;

        // Then
        assert_eq!(result, 18);

        Ok(())
    }
}
