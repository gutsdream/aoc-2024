use std::cmp::{max, min};
use eyre::Context;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::str::FromStr;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Button {
    point: Point,
    cost: i64,
}

impl Button {
    fn a(x: i64, y: i64) -> Button {
        Button {
            point: Point { x, y },
            cost: 3,
        }
    }

    fn b(x: i64, y: i64) -> Button {
        Button {
            point: Point { x, y },
            cost: 1,
        }
    }

    fn diff(&self, other: &Self) -> Point {
        Point {
            x: i64::abs(self.point.x - other.point.x),
            y: i64::abs(self.point.y - other.point.y),
        }
    }
}

#[derive(Debug, Clone)]
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
                    .collect::<Vec<i64>>();

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
    pub fn part_1(&self) -> i64 {
        self.machines
            .par_iter()
            .filter_map(|machine| {
                let mut zero_to_two_patterns = (0..100)
                    .rev()
                    .filter_map(|b_count| {
                        let prize_without_x = machine.prize.x - b_count * machine.b.point.x;
                        let prize_without_y = machine.prize.y - b_count * machine.b.point.y;

                        match prize_without_x % machine.a.point.x == 0_i64 && prize_without_y % machine.a.point.y == 0_i64 {
                            true => {
                                let a_count_x = prize_without_x / machine.a.point.x;
                                let a_count_y = prize_without_y / machine.a.point.y;

                                Some((a_count_x, a_count_y, b_count))
                            }
                            false => {None}
                        }
                    })
                    .take(2);

                match zero_to_two_patterns.next() {
                    Some(first) => match zero_to_two_patterns.next() {
                        Some(second) => {
                            let mut costs : Vec<i64> = Vec::new();
                            let a_x_diff = second.0 - first.0;
                            let a_y_diff = second.1 - first.1;
                            let b_diff = first.2 - second.2;


                            let mut a_x = first.0;
                            let mut a_y = first.1;
                            let mut b = first.2;

                            while b >= 0{
                                if a_x == a_y{
                                    costs.push(machine.a.cost * a_x + machine.b.cost * b)
                                }

                                a_x += a_x_diff;
                                a_y += a_y_diff;
                                b -= b_diff
                            }

                            costs.into_iter().min()
                        }
                        None => {
                            match first.0 == first.1 {
                                true => {
                                    Some(machine.a.cost * first.0 + machine.b.cost * first.2)
                                }
                                false => {None}
                            }
                        }
                    },
                    None => None,
                }
            })
            .sum()
    }

    pub fn part_2(&self) -> i64 {
        let increment = 10000000000000_i64;
        let mut machines = self.machines.clone();

        machines.iter_mut().for_each(|mut machine| {
            machine.prize.x += increment;
            machine.prize.y += increment;
        });

        machines
            .par_iter()
            .filter_map(|machine| {
                let prize_x_by_b = machine.prize.x / machine.b.point.x; // 381
                let prize_y_by_b = machine.prize.y / machine.b.point.y; // 80

                let ceiling = max(prize_x_by_b as i64, prize_y_by_b as i64);

                let mut zero_to_two_patterns = (0..ceiling)
                    .rev()
                    .filter_map(|b_count| {
                        let prize_without_x = machine.prize.x - b_count * machine.b.point.x;
                        let prize_without_y = machine.prize.y - b_count * machine.b.point.y;

                        match prize_without_x % machine.a.point.x == 0_i64 && prize_without_y % machine.a.point.y == 0_i64 {
                            true => {
                                let a_count_x = prize_without_x / machine.a.point.x;
                                let a_count_y = prize_without_y / machine.a.point.y;

                                Some((a_count_x, a_count_y, b_count))
                            }
                            false => { None }
                        }
                    })
                    .take(2);

                match zero_to_two_patterns.next() {
                    Some(first) => match zero_to_two_patterns.next() {
                        Some(second) => {
                            let mut costs: Vec<i64> = Vec::new();
                            let a_x_diff = second.0 - first.0;
                            let a_y_diff = second.1 - first.1;
                            let b_diff = first.2 - second.2;


                            let mut a_x = first.0;
                            let mut a_y = first.1;
                            let mut b = first.2;

                            while b >= 0 {
                                if a_x == a_y {
                                    costs.push(machine.a.cost * a_x + machine.b.cost * b)
                                }

                                a_x += a_x_diff;
                                a_y += a_y_diff;
                                b -= b_diff
                            }

                            costs.into_iter().min()
                        }
                        None => {
                            match first.0 == first.1 {
                                true => {
                                    Some(machine.a.cost * first.0 + machine.b.cost * first.2)
                                }
                                false => { None }
                            }
                        }
                    },
                    None => None,
                }
            })
            .sum()
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
    #[ignore] // this is currently way too unoptimized
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(1, sum);
    }
}
