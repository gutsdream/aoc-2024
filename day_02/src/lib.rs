use std::cmp::Ordering;

pub fn get_safe_level_report_count(input: String) -> usize {
    let level_reports = get_level_reports(input);

    let safe_levels = level_reports
        .into_iter()
        .filter(|x| is_report_safe(x))
        .collect::<Vec<_>>()
        .len();

    safe_levels
}

pub fn get_dampened_safe_report_count(input: String) -> usize {
    let level_reports = get_level_reports(input);

    let safe_levels = level_reports
        .into_iter()
        .filter(|x| is_report_safe(x) || can_report_be_dampened(x))
        .collect::<Vec<_>>()
        .len();

    safe_levels
}

fn get_level_reports(input: String) -> Vec<Vec<i32>>{
    input.lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|y| y.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let initial_order = report[0].cmp(&report[1]);

    let comparisons : Vec<(Ordering, i32)> = report.windows(2)
        .map(|pair| (pair[0].cmp(&pair[1]), i32::abs(pair[0] - pair[1])))
        .collect();

    let ordering_matches = comparisons.iter().all(|(ord, _)| ord == &initial_order);

    let differences_within_range = comparisons.iter().all(| (_, diff)| diff >= &1 && diff <= &3);

    ordering_matches && differences_within_range
}

pub fn can_report_be_dampened(report: &Vec<i32>) -> bool {
    for n in 0..report.len() {
        let subset : Vec<i32> = report.iter()
            .enumerate()
            .filter(|(i, _)| i!= &n)
            .map(|(_, v)| v.clone())
            .collect();

        if is_report_safe(&subset){
            return true;
        }
    }

    false
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
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];

        // when
        let output = get_level_reports(input);

        // then
        assert_eq!(output, expected);
    }

    #[test]
    fn should_get_safe_level_report_count() {
        // given
        let input = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

        // when
        let safe_levels = get_safe_level_report_count(input.to_string());

        // then
        assert_eq!(safe_levels, 2);
    }

    #[test]
    fn should_get_dampened_safe_report_count() {
        // given
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

        // when
        let safe_levels = get_dampened_safe_report_count(input.to_string());

        // then
        assert_eq!(safe_levels, 4);
    }
}
