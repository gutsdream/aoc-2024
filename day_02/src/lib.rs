pub fn get_level_reports(input: String) -> Vec<Vec<i32>>{
    input.lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|y| y.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_level_reports() {
        // given
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9".to_string();

        let expected = vec![
            vec![7,6,4,2,1],
            vec![1,2,7,8,9],
            vec![9,7,6,2,1],
            vec![1,3,2,4,5],
            vec![8,6,4,4,1],
            vec![1,3,6,7,9]
        ];

        // when
        let output = get_level_reports(input);

        // then
        assert_eq!(output, expected);
    }
}