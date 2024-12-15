use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn quadrants(&self) -> Vec<(Point,Point)> {
        let midpoint_x = self.x / 2;
        let midpoint_y = self.y / 2;

        vec![
            (Self::new(0, 0), Self::new(midpoint_x - 1, midpoint_y -1)),
            (Self::new(0, midpoint_y + 1), Self::new(midpoint_x - 1, self.y)),
            (Self::new(midpoint_x + 1, 0), Self::new(self.x, midpoint_y - 1)),
            (Self::new(midpoint_x + 1, midpoint_y + 1), Self::new(self.x, self.y)),
        ]
    }
}

#[derive(Debug, Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn move_within_dimensions(self, dimensions: &Point) -> Self {
        let mut x = self.position.x + self.velocity.x; // 5 + 2 = 7. 7 - 6
        let mut y = self.position.y + self.velocity.y; // 1 - 3 = -2. -2 + 10 = 8

        if x > dimensions.x {
            x = x - dimensions.x - 1;
        }

        if y > dimensions.y {
            y = y - dimensions.y - 1;
        }

        if x < 0 {
            x = x + dimensions.x + 1;
        }

        if y < 0 {
            y = y + dimensions.y + 1;
        }

        Robot {
            position: Point { x, y },
            velocity: self.velocity,
        }
    }
}

pub struct Puzzle {
    robots: Vec<Robot>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let robots = input
            .lines()
            .filter_map(|line| {
                let mut nums = line
                    .split(|x| x == '=' || x == ',' || x == ' ')
                    .filter_map(|p| p.parse::<i32>().ok());

                Some(Robot {
                    position: Point {
                        x: nums.next()?,
                        y: nums.next()?,
                    },
                    velocity: Point {
                        x: nums.next()?,
                        y: nums.next()?,
                    },
                })
            })
            .collect::<Vec<_>>();

        Ok(Puzzle { robots })
    }
}

impl Puzzle {
    pub fn part_1(&self, dimensions: Point) -> u32 {
        let robots_after_100_secs = self
            .robots
            .clone()
            .into_iter()
            .map(|robot| (0..100).fold(robot, |robot, i| {
                robot.move_within_dimensions(&dimensions)
            }))
            .collect::<Vec<_>>();

        let quadrants = dimensions.quadrants();

        quadrants.iter().map(|(floor, ceiling)|{
            robots_after_100_secs.iter()
                .filter(|robot| robot.position.x >= floor.x && robot.position.x <= ceiling.x &&
                    robot.position.y >= floor.y && robot.position.y <= ceiling.y)
                .count() as u32
        }).product()
    }

    fn snapshot(dimensions: &Point, robots: &Vec<Robot>) {
        let lines = (0..dimensions.y + 1)
            .map(|y| (0..dimensions.x + 1)
                .map(|x| {
                    let robots_at_position = robots
                        .iter()
                        .filter(|robot| robot.position.x == x && robot.position.y == y).count();
                    match robots_at_position {
                        0 => ".".to_string(),
                        _ => { robots_at_position.to_string() }
                    }
                }).collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<_>>();

        lines.iter().for_each(|line| println!("{}", line));
    }

    pub fn part_2(&self, dimensions: Point) -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    const SMOL: &str = "p=2,4 v=2,-3";

    #[test]
    fn should_solve_part_1() {
        // Given
        let dimensions = Point::new(10, 6);
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1(dimensions);

        // Then
        assert_eq!(12, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let dimensions = Point::new(6, 10);
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2(dimensions);

        // Then
        assert_eq!(1, sum);
    }
}
