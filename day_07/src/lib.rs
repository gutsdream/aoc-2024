use crate::Operator::{Divide, IsConcatenable, Subtract};
use rayon::prelude::*;
use std::iter::once;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operator {
    Subtract,
    Divide,
    IsConcatenable,
}

impl Operator {
    fn apply(&self, a: &i64, b: &i64) -> Option<i64> {
        match self {
            Subtract => Some(a - b),
            Divide => match a % b == 0 {
                true => Some(a / b),
                false => None,
            },
            IsConcatenable => {
                let length_b = b.checked_ilog10().unwrap_or(0) + 1;
                let b_divisor = 10_i64.pow(length_b);
                let a_without_b = a - b;
                Divide.apply(&a_without_b, &b_divisor)
            }
        }
    }
}

#[derive(Debug)]
struct ReversedEquation {
    inputs: Vec<i64>,
}

impl ReversedEquation {
    fn new(all_inputs: Vec<i64>) -> Option<ReversedEquation> {
        let mut iter = all_inputs.into_iter();
        let result = iter.next()?;
        let inputs = iter.chain(once(result)).rev().collect::<Vec<_>>();

        Some(ReversedEquation { inputs })
    }

    fn has_successful_variation(&self, applicable_operators: &Vec<Operator>) -> bool {
        if let Some(initial) = self.inputs.get(0) {
            return applicable_operators.iter().any(|x| {
                self.any_valid_calculation_routes(initial.clone(), 1, x, applicable_operators)
            });
        }

        false
    }

    fn any_valid_calculation_routes(
        &self,
        acc: i64,
        index: usize,
        operator: &Operator,
        applicable_operators: &Vec<Operator>,
    ) -> bool {
        if let Some(next) = self.inputs.get(index) {
            return match operator.apply(&acc, &next) {
                None => false,
                Some(result) => match result == 0 {
                    true => true,
                    false => applicable_operators.iter().any(|x| {
                        self.any_valid_calculation_routes(
                            result,
                            index + 1,
                            x,
                            applicable_operators,
                        )
                    }),
                },
            };
        }

        false
    }
}

pub struct Puzzle {
    equations: Vec<ReversedEquation>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let equations = input
            .lines()
            .map(|x| {
                x.split([':', ' '])
                    .filter_map(|x| x.parse::<i64>().ok())
                    .collect()
            })
            .filter_map(|x| ReversedEquation::new(x))
            .collect::<Vec<_>>();

        Ok(Puzzle { equations })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> i64 {
        self.equations
            .par_iter()
            .filter(|x| x.has_successful_variation(&vec![Subtract, Divide]))
            .map(|x| x.inputs[0])
            .sum()
    }

    pub fn part_2(&self) -> i64 {
        self.equations
            .par_iter()
            .filter(|x| x.has_successful_variation(&vec![Subtract, Divide, IsConcatenable]))
            .map(|x| x.inputs[0])
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(3749, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(11387, sum);
    }
}
