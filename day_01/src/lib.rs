pub fn get_location_id_pairs(pairs: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    pairs.into_iter().for_each(|x| {
        let pair: Vec::<i32> = x.split(' ')
            .filter_map(|y| y.parse::<i32>().ok())
            .collect();

        if pair.len() == 2 {
            left.push(pair[0]);
            right.push(pair[1]);
        }
    });

    left.sort();
    right.sort();

    (left, right)
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