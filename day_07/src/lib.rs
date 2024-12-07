use crate::Operator::{Add, Concatenate, Multiply};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Add => a + b,
            Multiply => a * b,
            Concatenate => {
                let length_b = b.checked_ilog10().unwrap_or(0) + 1;
                let a_raised = a * 10_u64.pow(length_b);
                a_raised + b
            }
        }
    }
}

#[derive(Debug)]
struct UnoperatedEquation {
    result: u64,
    inputs: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Operation {
    a: u64,
    b: u64,
    operator: Operator,
}

trait Calculate {
    fn calculate(&self) -> u64;
}

impl Calculate for Vec<Operation> {
    fn calculate(&self) -> u64 {
        let first = self[0].a;
        self.iter().fold(first, |acc, x| x.operator.apply(acc, x.b))
    }
}

impl UnoperatedEquation {
    fn new(all_inputs: Vec<u64>) -> Option<UnoperatedEquation> {
        let mut iter = all_inputs.into_iter();
        let result = iter.next()?;
        let inputs = iter.collect();

        Some(UnoperatedEquation { result, inputs })
    }

    fn has_successful_variation(&self, operators: &Vec<Operator>) -> bool {
        let pair_variations = self
            .inputs
            .windows(2)
            .map(|window| {
                operators
                    .clone()
                    .into_iter()
                    .map(|op| Operation {
                        a: window[0],
                        b: window[1],
                        operator: op,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let potential_equations = Self::gen_potential_equations(pair_variations);

        let results = potential_equations
            .iter()
            .map(|x| x.calculate())
            .collect::<Vec<_>>();

        results.iter().any(|x| x == &self.result)
    }

    fn gen_potential_equations(pair_variations: Vec<Vec<Operation>>) -> Vec<Vec<Operation>> {
        let initial: Vec<Vec<Operation>> = pair_variations[0]
            .clone()
            .into_iter()
            .map(|x| vec![x])
            .collect();

        pair_variations
            .into_iter()
            .skip(1)
            .fold(initial, |acc, pair| {
                pair.iter()
                    .enumerate()
                    .map(|(i, _)| {
                        acc.clone()
                            .into_iter()
                            .map(|mut x| {
                                x.push(pair[i].clone());
                                x
                            })
                            .collect::<Vec<Vec<Operation>>>()
                    })
                    .flatten()
                    .collect()
            })
    }
}

pub struct Puzzle {
    equations: Vec<UnoperatedEquation>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let equations = input
            .lines()
            .map(|x| {
                x.split([':', ' '])
                    .filter_map(|x| x.parse::<u64>().ok())
                    .collect()
            })
            .filter_map(|x| UnoperatedEquation::new(x))
            .collect::<Vec<_>>();

        Ok(Puzzle { equations })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u64 {
        self.equations
            .iter()
            .filter(|x| x.has_successful_variation(&vec![Add, Multiply]))
            .map(|x| x.result)
            .sum()
    }

    pub fn part_2(&self) -> u64 {
        self.equations
            .iter()
            .filter(|x| x.has_successful_variation(&vec![Add, Multiply, Concatenate]))
            .map(|x| x.result)
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
