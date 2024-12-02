use std::collections::HashMap;
use day_01::get_location_id_pairs;

fn main() {
    let input = include_str!("../input.txt");

    let (left, right) = get_location_id_pairs(input.lines()
        .collect());

    let similarity_score = calculate_similarity_score(left, right);

    println!("Part 2: {}", similarity_score);
}

fn calculate_similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32{
    let mut hashmap = HashMap::new();
    right.into_iter().for_each(|x| {
        hashmap.entry(x).and_modify(|x| *x += 1).or_insert(1);
    });

    left.into_iter()
        .map(|x| x * hashmap.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_similarity_score() {
        // given
        let mut left = vec![
            3,
            4,
            2,
            1,
            3,
            3
        ];

        let mut right = vec![
            4,
            3,
            5,
            3,
            9,
            3
        ];

        left.sort();
        right.sort();

        // when
        let result = calculate_similarity_score(left, right);

        // then
        assert_eq!(result, 31);
    }
}