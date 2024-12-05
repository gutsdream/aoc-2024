pub struct Puzzle{
    rules: Vec<Vec<usize>>,
    updates: Vec<Vec<usize>>
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let (rules, updates): (Vec<&str>, Vec<&str>) = input
            .lines()
            .filter(|&x| x.len() > 2)
            .partition(|&x| x.contains('|'));

        let rules = rules.into_iter()
            .filter_map(|x|Some(x.split('|')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect())
            ).collect();

        let updates = updates.into_iter()
            .map(|x| x.split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect())
            .collect();

        Puzzle{
            rules,
            updates
        }
    }
}

impl Puzzle{
    pub fn sum_of_correct_updates(&self) -> usize{
        self.updates.iter()
            .filter(|x| self.is_valid(x))
            .map(|x| Puzzle::get_middle_page(x))
            .sum()
    }

    pub fn sum_of_incorrect_updates(&self) -> usize{
        self.updates.iter()
            .filter(|x| !self.is_valid(x))
            .map(|x| self.correct_update(x.clone()))
            .map(|x| Puzzle::get_middle_page(&x))
            .sum()
    }

    // fn get_incorrect_updates(&self) -> Vec<Vec<usize>>{
    //     self.updates.iter()
    //         .filter(|x| !self.is_valid(x))
    //         .collect()
    // }

    fn get_middle_page(update: &Vec<usize>) -> usize{
        let len = update.len();
        let index = match len % 2{
            0 => len / 2 - 1,
            _ => len / 2
        };

        update[index]
    }

    fn is_valid(&self, update: &Vec<usize>) -> bool{
        let mut avoided_pages: Vec<usize> = vec![];
        for n in (0..update.len()).rev(){
            if avoided_pages.contains(&update[n]){
                return false;
            }

            let mut avoid_for_update: Vec<usize> = self.rules.iter()
                .filter(|x| &x[0] == &update[n])
                .map(|x| x[1])
                .collect();

            avoided_pages.append(&mut avoid_for_update);
        }

        true
    }

    pub fn correct_update(&self, mut update: Vec<usize>) -> Vec<usize>{
        let applicable_rules : Vec<&Vec<usize>> = self.rules.iter()
            .filter(|rule| update.contains(&rule[0]))
            .filter(|rule| update.contains(&rule[1]))
            .collect();

        let broken_rules : Vec<&Vec<usize>> = applicable_rules
            .into_iter()
            .filter(|rule| update.iter().position(|page| page == &rule[0]) > update.iter().position(|page| page == &rule[1]))
            .collect();

        if(broken_rules.len() == 0){
            return update
        }

        let left = update.iter().position(|page| page == &broken_rules[0][0]).unwrap();
        let right = update.iter().position(|page| page == &broken_rules[0][1]).unwrap();
        update.swap(left, right);

        // dbg!(&update, &broken_rules);

        self.correct_update(update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT : &str = "47|53
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