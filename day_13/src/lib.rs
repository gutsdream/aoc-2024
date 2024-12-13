use eyre::Context;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Button {
    point: Point,
    cost: i32,
}

impl Button {
    fn a(x: i32, y: i32) -> Button {
        Button {
            point: Point { x, y },
            cost: 3,
        }
    }

    fn b(x: i32, y: i32) -> Button {
        Button {
            point: Point { x, y },
            cost: 1,
        }
    }

    fn diff(&self, other: &Self) -> Point {
        Point {
            x: i32::abs(self.point.x - other.point.x),
            y: i32::abs(self.point.y - other.point.y),
        }
    }
}

#[derive(Debug)]
struct ClawMachine {
    a: Button,
    b: Button,
    prize: Point,
}

pub struct Puzzle {
    pub machines: Vec<ClawMachine>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"X.(\d*)|Y.(\d*)").expect("Hardcode regex should be valid");
        let machines = input
            .split("\n\n")
            .filter_map(|machine_config| {
                let captures = regex
                    .captures_iter(machine_config)
                    .filter_map(|capture| {
                        capture
                            .get(1)
                            .or_else(|| capture.get(2))
                            .map(|x| x.as_str())
                    })
                    .filter_map(|x| x.parse().ok())
                    .collect::<Vec<i32>>();

                let button_a = Button::a(captures.get(0)?.clone(), captures.get(1)?.clone());
                let button_b = Button::b(captures.get(2)?.clone(), captures.get(3)?.clone());
                let prize = Point {
                    x: captures.get(4)?.clone(),
                    y: captures.get(5)?.clone(),
                };

                Some(ClawMachine {
                    a: button_a,
                    b: button_b,
                    prize,
                })
            })
            .collect::<Vec<ClawMachine>>();

        Ok(Puzzle { machines })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32 {
        let kind_of_primes = [
            0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
            79, 83, 89, 97,
        ];

        let prime_combinations = iproduct!(
            kind_of_primes.clone().into_iter(),
            kind_of_primes.clone().into_iter()
        )
        .collect::<Vec<_>>();

        let variations = iproduct!(
            (0..101).into_iter(),
            (0..101).into_iter()
        ).collect::<Vec<_>>();

        self
            .machines
            .iter()
            .filter_map(|machine| {
                variations
                    .iter()
                    .filter(|x| x != &&(0, 0))
                    .filter(|(a_count, b_count)| {
                        machine.a.point.x * a_count + machine.b.point.x * b_count == machine.prize.x &&
                            machine.a.point.y * a_count + machine.b.point.y * b_count == machine.prize.y
                    })
                    .map(|(a_count, b_count)| {
                        (machine.a.cost * a_count + machine.b.cost * b_count) as u32
                    }).min()
                    // .filter_map(|(a_count, b_count)| {
                    //     Self::get_multiplier_for_combination(machine, a_count, b_count)
                    //         .and_then(|multiplier| Some(((a_count.clone(), b_count.clone()), multiplier)))
                    // })
                    // .filter_map(|((a_count, b_count), multiplier)|{
                    //     let a_multiplier = a_count * multiplier;
                    //     let b_multiplier = b_count * multiplier;
                    //
                    //     if(a_multiplier > 100 || b_multiplier > 100) {
                    //         return None;
                    //     }
                    //     let a_creds = machine.a.cost * a_count * multiplier;
                    //     let b_creds = machine.b.cost * b_count * multiplier;
                    //
                    //     Some((a_creds + b_creds) as u32)
                    // }).min()
            })
            .sum()
    }

    fn get_multiplier_for_combination(machine: &ClawMachine, a_count: &i32, b_count: &i32) -> Option<(i32)> {
        let unique_x_combination =
            machine.a.point.x * a_count + machine.b.point.x * b_count;
        let unique_y_combination =
            machine.a.point.y * a_count + machine.b.point.y * b_count;

        match machine.prize.x % unique_x_combination == 0
            && machine.prize.y % unique_y_combination == 0
        {
            true => {
                Some(machine.prize.x / unique_x_combination)
            }
            false => None,
        }
    }

    pub fn part_2(&self) -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(480, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(1, sum);
    }
}
