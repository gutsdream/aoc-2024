mod direction;
mod region;

use crate::direction::STRAIGHT;
use direction::Point;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::region::Area;

pub struct Puzzle {
    regions: Vec<HashSet<Point>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid: HashMap<Point, char> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| (Point { x, y }, c))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        let mut regions : Vec<HashSet<Point>> = Vec::new();

        fn build_region_points(
            grid: &mut HashMap<Point, char>,
            point: Point,
            points: &mut HashSet<Point>,
        ) {
            if let Some(char) = grid.remove(&point) {

                STRAIGHT
                    .iter()
                    .filter_map(|direction| point.gen_in_direction(direction))
                    .for_each(|point| {
                        if let Some(c) = grid.get(&point) {
                            if c == &char {
                                build_region_points(grid, point, points)
                            }
                        }
                    });

                points.insert(point);
            }
        }

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each( |(x, c)| {
                    let mut points = HashSet::new();
                    build_region_points(&mut grid,Point{x, y}, &mut points);
                    if points.len() > 0 {
                        regions.push(points);
                    }
                })
        });

        Ok(Puzzle {
            regions
        })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32 {
        self.regions.iter().map(|region| region.area() * region.perimeter()).sum()
    }

    pub fn part_2(&self) -> u32 {
        self.regions.iter().map(|region| region.area() * region.sides()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(1930, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(1206, sum);
    }
}
