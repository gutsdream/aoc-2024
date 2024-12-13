use crate::direction::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};
use crate::direction::{Direction, Point, DIAGONAL, STRAIGHT};
use std::collections::HashSet;

pub trait Area {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
    fn sides(&self) -> u32;
    fn touches_plant(&self, point: &Point, direction: &Direction) -> bool;
}

impl Area for HashSet<Point> {
    fn area(&self) -> u32 {
        let area = self.len() as u32;
        area
    }

    fn perimeter(&self) -> u32 {
        let perimeter = self
            .iter()
            .map(|point| {
                STRAIGHT
                    .iter()
                    .map(|direction| match point.gen_in_direction(direction) {
                        None => 1,
                        Some(gen_point) => match self.contains(&gen_point) {
                            true => 0,
                            false => 1,
                        },
                    })
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .sum();

        perimeter
    }

    fn sides(&self) -> u32 {
        let sides: u32 = self
            .iter()
            .map(|point| {
                let vertical = [North, South]
                    .iter()
                    .filter(|direction| self.touches_plant(point, direction))
                    .collect::<Vec<_>>();

                let horizontal = [East, West]
                    .iter()
                    .filter(|direction| self.touches_plant(point, direction))
                    .collect::<Vec<_>>();

                // Either this is an external or internal corner, and the direction has changed
                let corner_count = match (
                    vertical.len(),
                    horizontal.len(),
                ) {
                    (0, 0) => 4_u32,
                    (0, 1) => 2,
                    (1, 0) => 2,
                    (0, 2) => 0,
                    (2, 0) => 0,
                    (2, 2) => {
                        {
                            [NorthEast, SouthEast, SouthWest, NorthWest]
                                .iter()
                                .filter(|direction| !self.touches_plant(point, direction))
                                .count() as u32
                        }
                    },
                    (1, 1) => {
                        let direction = Direction::get_diagonal(vertical[0], horizontal[0]).unwrap();
                        match self.touches_plant(&point, &direction) {
                            true => {1}
                            false => {2}
                        }
                    }
                    (2, 1) => {
                        vertical.iter().filter(|vert| {
                            let direction = Direction::get_diagonal(vert, horizontal[0]).unwrap();
                            !self.touches_plant(point, &direction)
                        }).count() as u32
                    },
                    (1, 2) => {
                        horizontal.iter().filter(|hori| {
                            let direction = Direction::get_diagonal(vertical[0], hori).unwrap();
                            !self.touches_plant(point, &direction)
                        }).count() as u32
                    },
                    _ => 0,
                };

                corner_count
            })
            .sum();

        sides as u32
    }

    fn touches_plant(&self, point: &Point, direction: &Direction) -> bool {
        match point.gen_in_direction(direction) {
            None => false,
            Some(gen_point) => match self.contains(&gen_point) {
                true => true,
                false => false,
            },
        }
    }
}
