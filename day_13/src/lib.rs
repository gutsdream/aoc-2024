use eyre::Context;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{min, Ordering};
use std::str::FromStr;
use float_cmp::approx_eq;

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

enum Direction{
    Up,
    Down
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
            .iter()
            .filter_map(|machine| {
                Self::prize_cost(machine, Some(100))
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
            .iter()
            .filter_map(|machine| {
                Self::prize_cost(machine, None)
            })
            .sum()
    }

    fn prize_cost(machine: &ClawMachine, ceiling: Option<i64>) -> Option<i64> {
        let prize_x_by_b = machine.prize.x / machine.b.point.x;
        let prize_y_by_b = machine.prize.y / machine.b.point.y;

        let mut floor = 0;

        let mut ceiling = match ceiling {
            None => {
                min(prize_x_by_b + 1, prize_y_by_b + 1)
            }
            Some(ceiling) => {
                min(min(prize_x_by_b + 1, prize_y_by_b + 1), ceiling)
            }
        };

        let prize_without_x = machine.prize.x - ceiling * machine.b.point.x;
        let prize_without_y = machine.prize.y - ceiling * machine.b.point.y;

        let mut mid_point = (ceiling + floor) / 2;
        let mut a_count_x = prize_without_x as f64 / machine.a.point.x as f64;
        let mut a_count_y = prize_without_y as f64 / machine.a.point.y as f64;

        while ceiling - floor > 1
        {
            let (new_a_count_x, new_a_count_y) = Self::get_a_pair(machine, mid_point);

            if approx_eq!(f64, new_a_count_x, new_a_count_y, ulps = 2) {
                let a_multiplier = new_a_count_x.round() as i64;
                let x = a_multiplier * machine.a.point.x + mid_point * machine.b.point.x;
                let y = a_multiplier * machine.a.point.y + mid_point * machine.b.point.y;

                if machine.prize.x == x && machine.prize.y == y {
                    return Some(new_a_count_x.round() as i64 * machine.a.cost + mid_point * machine.b.cost);
                }
            }

            let up_one_pair = Self::get_a_pair(machine, mid_point + 1);
            let up_one_diff = Self::pair_diff(up_one_pair);

            let down_one_pair = Self::get_a_pair(machine, mid_point - 1);
            let down_one_diff = Self::pair_diff(down_one_pair);

            match up_one_diff.partial_cmp(&down_one_diff) {
                Some(Ordering::Less) => { floor = mid_point }
                Some(Ordering::Greater) => { ceiling = mid_point }
                _ => { return None }
            }

            mid_point = (ceiling + floor) / 2;
            a_count_x = new_a_count_x;
            a_count_y = new_a_count_y;
        }

        None
    }

    fn pair_diff(a_count: (f64, f64)) -> f64 {
        f64::abs(a_count.0 - a_count.1)
    }

    fn get_a_pair(machine: &ClawMachine, mut mid_point: i64) -> (f64, f64) {
        let prize_without_x = machine.prize.x - mid_point * machine.b.point.x;
        let prize_without_y = machine.prize.y - mid_point * machine.b.point.y;

        let new_a_count_x = prize_without_x as f64 / machine.a.point.x as f64;
        let new_a_count_y = prize_without_y as f64 / machine.a.point.y as f64;
        (new_a_count_x, new_a_count_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
        "Button A: X+94, Y+34
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
        assert_eq!(875318608908, sum);
    }
}
