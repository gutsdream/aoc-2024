use std::str::FromStr;

pub struct Puzzle {
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32{
        1
    }

    pub fn part_2(&self) -> u32{
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(1, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(1, sum);
    }
}
