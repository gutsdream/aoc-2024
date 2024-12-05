use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Puzzle {
    rules: HashMap<usize,Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let (rules_vec, updates): (Vec<&str>, Vec<&str>) = input
            .lines()
            .filter(|&x| x.len() > 2)
            .partition(|&x| x.contains('|'));

        let mut rules = HashMap::<usize, Vec<usize>>::new();
        rules_vec
            .into_iter()
            .map(|pair| pair.split('|')
                .filter_map(|page| page.parse::<usize>().ok())
                .collect::<Vec<usize>>()
            )
            .for_each(|x| rules.entry(x[0]).or_default().push(x[1]));

        let updates = updates
            .into_iter()
            .map(|x| {
                x.split(',')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect()
            })
            .collect();

        Puzzle { rules, updates }
    }
}

impl Puzzle {
    pub fn sum_of_correct_updates(&self) -> usize {
        self.updates
            .iter()
            .filter(|x| self.is_valid(x))
            .map(|x| Self::get_middle_page(x))
            .sum()
    }

    pub fn sum_of_incorrect_updates(&self) -> usize {
        self.updates
            .iter()
            .filter(|x| !self.is_valid(x))
            .map(|x| self.correct_update(x.clone()))
            .map(|x| Self::get_middle_page(&x))
            .sum()
    }

    fn get_middle_page(update: &Vec<usize>) -> usize {
        let len = update.len();
        let index = match len % 2 {
            0 => len / 2 - 1,
            _ => len / 2,
        };

        update[index]
    }

    fn is_valid(&self, update: &Vec<usize>) -> bool {
        let empty = Vec::new();
        let mut update_sorted = update.clone();
        update_sorted.is_sorted_by(|a, b| self.rules.get(a).unwrap_or(&empty).iter().contains(&b))
    }

    pub fn correct_update(&self, mut update: Vec<usize>) -> Vec<usize> {
        let empty = Vec::new();
        let mut update_sorted = update.clone();
        update_sorted.sort_by(|a, b| match self.rules.get(a).unwrap_or(&empty).iter().contains(&b) {
            true => {Ordering::Greater}
            false => {Ordering::Less}
        });

        update_sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn should_solve_sum_of_correct_updates() {
        // Given
        let puzzle = Puzzle::from(INPUT);

        // When
        let sum = puzzle.sum_of_correct_updates();

        // Then
        assert_eq!(143, sum);
    }

    #[test]
    fn should_solve_sum_of_incorrect_updates() {
        // Given
        let puzzle = Puzzle::from(INPUT);

        // When
        let sum = puzzle.sum_of_incorrect_updates();

        // Then
        assert_eq!(123, sum);
    }
}
