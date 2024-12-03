use std::{cmp, fs};

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = solve(input);

    println!("Part 1: {}", result?);

    Ok(())
}

fn solve(input: String) -> eyre::Result<i32> {
    let max_range = input.len();
    let regex = regex::Regex::new(r"(\d{1,3}),(\d{1,3})\)")?;
    Ok(input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            let mul_index = i + 4;
            let maximum_slice_range = cmp::min(mul_index + 3 + 1 + 3 + 1, max_range);
            let slice = &input[(mul_index)..maximum_slice_range];

            let captures = regex.captures(slice)?;
            let x : i32 = captures.get(1)?.as_str().parse().ok()?;
            let y : i32 = captures.get(2)?.as_str().parse().ok()?;

            Some( x * y )
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_multiply_valid_statements() -> eyre::Result<()> {
        // Given
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        // When
        let result = solve(input.to_string())?;

        // Then
        assert_eq!(result, 161);

        Ok(())
    }
}