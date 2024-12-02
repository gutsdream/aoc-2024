pub fn get_location_id_pairs(pairs: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
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

    #[test]
    fn should_get_sorted_location_id_pairs() {
        // given
        let mut pairs = vec![
            "2 1",
            "3 4",
            "5 2",
            "2 4",
        ];

        let expected_left = [2, 2, 3, 5];
        let expected_right = [1, 2, 4, 4];

        // when
        let (left, right) = get_location_id_pairs(pairs);

        // then
        assert_eq!(left, expected_left);
        assert_eq!(right, expected_right);
    }
}