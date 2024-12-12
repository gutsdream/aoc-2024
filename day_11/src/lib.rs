use std::collections::HashMap;
use std::str::FromStr;

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let stones = input.split(' ').filter_map(|x| x.parse().ok()).collect();

        Ok(Puzzle { stones })
    }
}

pub struct Puzzle {
    stones: Vec<u64>,
}

impl Puzzle {
    pub fn part_1(&self) -> u64 {
        self.count_for_blinks(25)
    }

    pub fn part_2(&self) -> u64 {
        self.count_for_blinks(75)
    }

    fn count_for_blinks(&self, depth: u8) -> u64 {
        let mut cache: HashMap<u64, u64> = HashMap::new();
        self.stones
            .clone()
            .into_iter()
            .for_each(|x| {*cache.entry(x).or_insert(0) += 1});

        (0..depth).for_each(|current_depth| {
            let mut cache_iteration : HashMap<u64, u64>= HashMap::new();

            cache
                .iter()
                .for_each(|(stone, count)| {
                    Self::blink(&mut cache_iteration, stone, count);
                });

            cache = cache_iteration;
        });

        cache.values().sum()
    }

    fn blink(cache_iteration: &mut HashMap<u64, u64>, stone: &u64, count: &u64) {
        let length = stone.checked_ilog10().unwrap_or(0) + 1;

        if length % 2 == 0 {
            let half_length = 10_u64.pow(length / 2);

            *cache_iteration.entry(stone / half_length).or_insert(0) += count;
            *cache_iteration.entry(stone % half_length).or_insert(0) += count
        } else {
            match stone == &0 {
                true => { *cache_iteration.entry(1).or_insert(0) += count }
                false => { *cache_iteration.entry(stone * 2024).or_insert(0) += count }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(55312, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(65601038650482, sum);
    }
}
