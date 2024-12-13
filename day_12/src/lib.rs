mod direction;
mod region;

use crate::direction::STRAIGHT;
use crate::region::Area;
use direction::Point;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

pub struct Puzzle {
    regions: Vec<HashSet<Point>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut regions = Arc::new(Mutex::new(Vec::new()));

        let mut grid: HashMap<char, HashSet<Point>> = HashMap::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                grid.entry(c).or_default().insert(Point { x, y });
            })
        });

        fn build_region_points(
            points: &mut HashSet<Point>,
            point: Point,
            contacting_points: &mut HashSet<Point>,
        ) {
            if points.remove(&point) {
                STRAIGHT
                    .iter()
                    .filter_map(|direction| point.gen_in_direction(direction))
                    .for_each(|point| {
                        if points.contains(&point) {
                                build_region_points(points, point, contacting_points)
                        }
                    });

                contacting_points.insert(point);
            }
        }

        grid.into_par_iter().for_each(|(char, mut points)| {
            points.clone().into_iter().for_each(|point| {
                if points.contains(&point) {
                    let mut contacting_points = HashSet::new();
                    build_region_points(&mut points, point.clone(), &mut contacting_points);
                    if contacting_points.len() > 0 {
                        let regions = regions.clone();
                        if let Some(mut regions) = regions.lock().ok() {
                            regions.push(contacting_points);
                        };
                    }
                }
            })
        });

        Ok(Puzzle { regions: Arc::try_unwrap(regions).unwrap().into_inner().unwrap() })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32 {
        self.regions
            .par_iter()
            .map(|region| region.area() * region.perimeter())
            .sum()
    }

    pub fn part_2(&self) -> u32 {
        self.regions
            .par_iter()
            .map(|region| region.area() * region.sides())
            .sum()
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
