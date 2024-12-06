use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;

pub struct Puzzle {
}

impl FromStr for Puzzle {
    fn from(input: &str) -> Puzzle {
        Puzzle { }
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32{
        1
    }

    pub fn part_2(&self) -> u32{
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from(INPUT);

        // When
        let sum = puzzle.sum_of_correct_updates();

        // Then
        assert_eq!(1, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from(INPUT);

        // When
        let sum = puzzle.sum_of_incorrect_updates();

        // Then
        assert_eq!(1, sum);
    }
}
