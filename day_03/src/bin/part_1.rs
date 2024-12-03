use std::{cmp, fs};

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = solve(input);

    println!("Part 1: {}", result?);

    Ok(())
}

fn solve(input: String) -> eyre::Result<i32> {
    day_03::solve(input, &|_| false)
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