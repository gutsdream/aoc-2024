use direction::Direction;
use navigation::Map;
use std::str::FromStr;
use crate::navigation::{MapPosition, Navigate, Point};

mod direction;
mod navigation;

pub struct Puzzle {
    map: Map,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
            )
            .collect();

        Ok(Puzzle { map })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32 {
        let starting_positions = self.get_starting_positions();

        starting_positions
            .iter()
            .map(|position| self.map.get_trailhead_score(position.clone()))
            .sum()
    }

    pub fn part_2(&self) -> u32 {
        let starting_positions = self.get_starting_positions();

        starting_positions
            .iter()
            .map(|position| self.map.get_trailhead_rating(position.clone()))
            .sum()
    }

    fn get_starting_positions(&self) -> Vec<MapPosition> {
        self.map.iter()
            .enumerate()
            .map(|(y, row)| row.iter()
                .enumerate()
                .filter_map(|(x, height)| match height == &0 {
                    true => {
                        Some(MapPosition {
                            point: Point { x, y },
                            height: height.clone()
                        })
                    }
                    false => { None }
                }).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const SMALL :&str = "0123
1234
8765
9876";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(36, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(81, sum);
    }
}
