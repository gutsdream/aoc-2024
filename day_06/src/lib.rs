use std::collections::HashSet;
use crate::Direction::{East, North, South, West};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

impl Direction {
    pub fn traversal(&self) -> (isize, isize) {
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }

    pub fn rotate(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Debug, Eq, Hash, Clone)]
struct Point(usize, usize);

#[derive(Debug)]
struct Guard {
    point: Point,
    direction: Direction,
}

pub enum WalkResult{
    WalkedTo(Point),
    ObstructedAt(Point)
}

impl Guard {
    fn new(point: Point, direction: Direction) -> Self {
        Guard { point, direction }
    }

    fn try_walk(&mut self, map: &Map) -> WalkResult{
        let new_point = self.point.gen_point_for_direction(&self.direction).unwrap();

        match map.point_is_obstructed(&new_point){
            true => {
                WalkResult::ObstructedAt(new_point)
            }
            false => {
                self.point = new_point.clone();
                WalkResult::WalkedTo(new_point)
            }
        }
    }

    fn turn(&mut self) {
        self.direction = self.direction.rotate();
    }
}

#[derive(Debug)]
struct Map {
    points: Vec<Vec<char>>,
}

impl Map {
    fn point_is_obstructed(&self, point: &Point) -> bool {
        let char = self.points[point.1][point.0];
        char == '#'
    }

    fn at_map_boundary(&self, point: &Point) -> bool {
        let vertical_max = self.points.len() - 1;
        let horizontal_max = self.points.first().map(|x| x.len() -1).unwrap_or(0);

        point.0 < 1 ||
            point.0 >= horizontal_max ||
            point.1 < 1 ||
            point.1 >= vertical_max
    }
}

impl Point {
    fn gen_point_for_direction(&self, direction: &Direction) -> Option<Point> {
        let traversal = direction.traversal();
        let x: usize = (self.0 as i32 + (traversal.0 as i32)).try_into().ok()?;
        let y: usize = (self.1 as i32 + (traversal.1 as i32)).try_into().ok()?;

        Some(Point(x, y))
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug)]
pub struct Puzzle {
    map: Map,
    guard: Guard,
}

pub enum PuzzleParsingError {
    CouldNotFindGuard,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let guard = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '.' => None,
                        '^' => Some(Guard::new(Point(x, y), North)),
                        '>' => Some(Guard::new(Point(x, y), East)),
                        'v' => Some(Guard::new(Point(x, y), South)),
                        '<' => Some(Guard::new(Point(x, y), West)),
                        _ => None,
                    })
                    .collect::<Vec<Guard>>()
            })
            .flatten()
            .next()
            .unwrap();

        let map = Map {
            points: input.lines().map(|x| x.chars().collect()).collect(),
        };

        Ok(Puzzle { guard, map })
    }
}

impl Puzzle {
    pub fn distinct_positions_visited(&mut self) -> usize {
        let mut points_visited = HashSet::new();
        points_visited.insert(self.guard.point.clone());
        loop{
            match self.guard.try_walk(&self.map) {
                WalkResult::WalkedTo(point) => {
                    if self.map.at_map_boundary(&point) {
                        points_visited.insert(point);

                        break;
                    }

                    points_visited.insert(point);
                }
                WalkResult::ObstructedAt(_) => {
                    self.guard.turn();
                }
            }
        }

        points_visited.iter().count()
    }

    pub fn potential_loop_opportunities(&mut self) -> u32 {
        let mut loop_opportunities = 0;

        let mut points_visited = HashSet::new();
        let mut obstructions_encountered : Vec<(Point, Direction)> = Vec::new();
        points_visited.insert((self.guard.point.clone(), self.guard.direction.clone()));
        loop{
            match self.guard.try_walk(&self.map) {
                WalkResult::WalkedTo(point) => {
                    let rotated = self.guard.direction.rotate();
                    if let Some(next_obstruction_if_rotated) = self.next_obstruction(&point, &rotated) {
                        if obstructions_encountered.iter()
                            .any(|x| x.0 == next_obstruction_if_rotated && x.1 == rotated) {
                            loop_opportunities += 1;
                        }
                    }

                    if self.map.at_map_boundary(&point) {
                        points_visited.insert((point.clone(), self.guard.direction.clone()));

                        break;
                    }

                    points_visited.insert((point.clone(), self.guard.direction.clone()));

                }
                WalkResult::ObstructedAt(point) => {
                    obstructions_encountered.push((point, self.guard.direction.clone()));
                    self.guard.turn();
                }
            }
        }

        loop_opportunities
    }

    pub fn next_obstruction(&self, current: &Point, direction: &Direction) -> Option<Point> {
        if let Some(point) = current.gen_point_for_direction(&direction) {
            if self.map.point_is_obstructed(&point){
                return Some(point);
            }

            if self.map.at_map_boundary(&point){
                return None;
            }

            return self.next_obstruction(&point, direction);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn should_solve_part_1() {
        // Given
        let mut puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.distinct_positions_visited();

        // Then
        assert_eq!(41, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let mut puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.potential_loop_opportunities();

        // Then
        assert_eq!(6, sum);
    }
}
