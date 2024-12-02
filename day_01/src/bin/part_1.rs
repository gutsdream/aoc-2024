use std::fs;
use std::iter::zip;
use day_01::get_location_id_pairs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("../input.txt")?;

    let (left, right) = get_location_id_pairs(input.lines()
        .collect());

    let distance = calculate_total_distance(left, right);

    println!("Part 1: {}", distance);

    Ok(())
}

fn calculate_total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    zip(left, right)
        .map(|(x, y)| i32::abs(x - y))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_total_distance() {
        // given
        let mut left = vec![
            3,
            4,
            2,
            1,
            3,
            3
        ];

        let mut right = vec![
            4,
            3,
            5,
            3,
            9,
            3
        ];

        left.sort();
        right.sort();

        // when
        let result = calculate_total_distance(left, right);

        // then
        assert_eq!(result, 11);
    }
}