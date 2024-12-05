use eyre::ContextCompat;
use crate::Direction::{NorthEast, NorthWest, SouthEast, SouthWest};
use crate::search;

pub fn solve(input: String) -> eyre::Result<i32> {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect::<Vec<_>>();

    let mut count = 0;

    let cross_matches = [
        ["M", "S", "S", "M"],
        ["M", "M", "S", "S"],
        ["S", "M", "M", "S"],
        ["S", "S", "M", "M"]
    ];

    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            let char = line.chars().nth(x).context("Char should exist in length")?;
            if char == 'A' {
                let nw = search(x, y, &lines, &NorthWest, 1);
                let ne = search(x, y, &lines, &NorthEast, 1);
                let se = search(x, y, &lines, &SouthEast, 1);
                let sw = search(x, y, &lines, &SouthWest, 1);

                let maybe_cross_match = [nw.as_str(), ne.as_str(), se.as_str(), sw.as_str()];

                if cross_matches.contains(&maybe_cross_match){
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::part_2::solve;
    #[test]
    fn should_solve_part_2() -> eyre::Result<()> {
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
        assert_eq!(result, 9);

        Ok(())
    }
}