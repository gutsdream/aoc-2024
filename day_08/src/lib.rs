use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

struct PointGenerator {
    maximum_x: usize,
    maximum_y: usize,
}

impl PointGenerator {
    fn with_thresholds(maximum_x: usize, maximum_y: usize) -> PointGenerator {
        PointGenerator {
            maximum_x,
            maximum_y,
        }
    }

    fn resonant_antinodes(&self, a: &Point, b: &Point) -> Vec<Point> {
        match self.antinode(a, b).is_some() {
            true => {
                let mut antinodes: Vec<Point> = vec![a.clone(), b.clone()];
                self.antinodes_till_inner(a, b, &mut antinodes);
                antinodes
            }
            false => Vec::new(),
        }
    }

    fn antinodes_till_inner(&self, a: &Point, b: &Point, antinodes: &mut Vec<Point>) {
        if let Some(c) = self.antinode(a, b) {
            self.antinodes_till_inner(b, &c, antinodes);
            antinodes.push(c);
        }
    }

    fn antinode(&self, a: &Point, b: &Point) -> Option<Point> {
        if a == b {
            return None;
        }

        let leapfrogged_x: usize = (b.x as isize + (b.x as isize - a.x as isize))
            .try_into()
            .ok()?;
        let leapfrogged_y: usize = (b.y as isize + (b.y as isize - a.y as isize))
            .try_into()
            .ok()?;

        self.generate_point(leapfrogged_x, leapfrogged_y)
    }

    fn generate_point(&self, x: usize, y: usize) -> Option<Point> {
        match x <= self.maximum_x {
            true => match y <= self.maximum_y {
                true => Some(Point::new(x, y)),
                false => None,
            },
            false => None,
        }
    }
}

pub struct Puzzle {
    node_map: HashMap<char, Vec<Point>>,
    point_generator: PointGenerator,
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let maximum_x = input
            .lines()
            .next()
            .ok_or("No lines found in input".to_string())?
            .len()
            - 1;

        let maximum_y = input.lines().count() - 1;

        let nodes = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c != &'.')
                    .map(|(x, char)| (char, Point::new(x, y)))
                    .collect::<Vec<(char, Point)>>()
            })
            .flatten()
            .into_group_map();

        let point_generator = PointGenerator::with_thresholds(maximum_x, maximum_y);

        Ok(Puzzle {
            node_map: nodes,
            point_generator,
        })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> usize {
        let antinodes = self
            .node_map
            .iter()
            .map(|(_, nodes)| {
                nodes
                    .iter()
                    .map(|a| {
                        nodes
                            .iter()
                            .filter_map(|b| self.point_generator.antinode(a, b))
                            .collect::<Vec<Point>>()
                    })
                    .flatten()
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .unique()
            .collect::<Vec<Point>>();

        antinodes.iter().count()
    }

    pub fn part_2(&self) -> usize {
        let resonant_antinodes = self
            .node_map
            .iter()
            .map(|(_, nodes)| {
                nodes
                    .iter()
                    .map(|a| {
                        nodes
                            .iter()
                            .map(|b| self.point_generator.resonant_antinodes(a, b))
                            .flatten()
                            .collect::<Vec<Point>>()
                    })
                    .flatten()
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .unique()
            .collect::<Vec<Point>>();

        resonant_antinodes.iter().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(14, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let x: usize = 3;
        let y: usize = 4;
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(34, sum);
    }
}
