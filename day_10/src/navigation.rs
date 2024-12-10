use itertools::Itertools;
use crate::direction::{Direction, DIRECTIONS};

pub type Map = Vec<Vec<u32>>;

const START: usize = 0;
const END: usize = 9;

pub trait Navigate {
    fn get_trailhead_score(&self, position: MapPosition) -> u32;
    fn get_trailhead_rating(&self, position: MapPosition) -> u32;
    fn get_next_trail_steps(&self, position: &MapPosition) -> Vec<MapPosition>;
    fn at_point(&self, point: Point) -> Option<MapPosition>;
}

impl Navigate for Map {
    fn get_trailhead_score(&self, position: MapPosition) -> u32 {
        let final_positions = (0..9).fold(vec![position], |positions, _| {
            positions.iter()
                .map(|position| self.get_next_trail_steps(position))
                .flatten()
                .collect()
        });

        final_positions.iter().unique().count() as u32
    }

    fn get_trailhead_rating(&self, position: MapPosition) -> u32 {
        let final_positions = (0..9).fold(vec![position], |positions, _| {
            positions.iter()
                .map(|position| self.get_next_trail_steps(position))
                .flatten()
                .collect()
        });

        final_positions.iter().count() as u32
    }

    fn get_next_trail_steps(&self, position: &MapPosition) -> Vec<MapPosition> {
        DIRECTIONS.iter()
            .filter_map(|direction| position.point.gen_in_direction(direction))
            .filter_map(|point| self.at_point(point))
            .filter(|next_position| next_position.height.checked_sub(position.height) == Some(1))
            .collect()
    }

    fn at_point(&self, point: Point) -> Option<MapPosition> {
        self.get(point.y)
            .and_then(|row| row.get(point.x))
            .and_then(|height| Some(MapPosition{ point, height: height.clone() }))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MapPosition {
    pub point: Point,
    pub height: u32,
}

impl Point {
    fn gen_in_direction(&self, direction: &Direction) -> Option<Self> {
        let traversal = direction.traversal();
        let x: usize = (self.x as isize + traversal.0).try_into().ok()?;
        let y: usize = (self.y as isize + traversal.1).try_into().ok()?;

        Some(Point { x, y })
    }
}