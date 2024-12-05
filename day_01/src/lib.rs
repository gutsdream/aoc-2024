use std::iter::zip;
use std::collections::HashMap;

pub fn calculate_total_distance(input: String) -> i32 {
    let (left, right) = get_location_id_pairs(input.lines().collect());

    zip(left, right)
        .map(|(x, y)| i32::abs(x - y))
        .sum()
}

pub fn calculate_similarity_score(input: String) -> i32{
    let (left, right) = get_location_id_pairs(input.lines().collect());

    let mut hashmap = HashMap::new();
    right.into_iter().for_each(|x| {
        hashmap.entry(x).and_modify(|x| *x += 1).or_insert(1);
    });

    left.into_iter()
        .map(|x| x * hashmap.get(&x).unwrap_or(&0))
        .sum()
}

fn get_location_id_pairs(pairs: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) : (Vec<i32>, Vec<i32>) = pairs.into_iter().filter_map(|x| {
        extract_numeric_pair(x)
    }).collect();

    left.sort();
    right.sort();

    (left, right)
}

fn extract_numeric_pair(x: &str) -> Option<(i32, i32)> {
    let pair: Vec::<i32> = x.split(' ')
        .filter_map(|y| y.parse::<i32>().ok())
        .collect();

    match pair.len() == 2 {
        true => Some((pair[0], pair[1])),
        false => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT :&str = "3 4
4 3
2 5
1 3
3 9
3 3";

    #[test]
    fn should_calculate_total_distance() {
        // when
        let result = calculate_total_distance(INPUT.to_string());

        // then
        assert_eq!(result, 11);
    }

    #[test]
    fn should_calculate_similarity_score() {
        // when
        let result = calculate_similarity_score(INPUT.to_string());

        // then
        assert_eq!(result, 31);
    }
}