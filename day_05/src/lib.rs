pub struct Puzzle {
    rules: Vec<Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let (rules, updates): (Vec<&str>, Vec<&str>) = input
            .lines()
            .filter(|&x| x.len() > 2)
            .partition(|&x| x.contains('|'));

        let rules = rules
            .into_iter()
            .filter_map(|x| {
                Some(
                    x.split('|')
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect(),
                )
            })
            .collect();

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
        let applicable_rules: Vec<&Vec<usize>> = self.applicable_rules(&update);

        applicable_rules
            .iter()
            .all(|x| Self::is_rule_met(&update, x))
    }

    pub fn correct_update(&self, mut update: Vec<usize>) -> Vec<usize> {
        let mut corrected = vec![];
        let mut applicable_rules: Vec<&Vec<usize>> = self.applicable_rules(&update);
        let length = update.len();

        while corrected.len() < length {

            // dbg!(&applicable_rules);
            let absent_rhs: usize = update
                .iter()
                .find(|x| !applicable_rules.iter().any(|rule| &&rule[1] == x))
                .map(|x| x.clone())
                .unwrap();

            // Remove any rules related to the sorted page
            applicable_rules.retain(|rule| rule[0] != absent_rhs);
            update.retain(|x| x != &absent_rhs);
            corrected.push(absent_rhs);
        }

        corrected
    }

    fn applicable_rules(&self, update: &Vec<usize>) -> Vec<&Vec<usize>> {
        self.rules
            .iter()
            .filter(|rule| update.contains(&rule[0]))
            .filter(|rule| update.contains(&rule[1]))
            .collect()
    }

    fn is_rule_met(update: &Vec<usize>, rule: &Vec<usize>) -> bool {
        update.iter().position(|page| page == &rule[0])
            < update.iter().position(|page| page == &rule[1])
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
