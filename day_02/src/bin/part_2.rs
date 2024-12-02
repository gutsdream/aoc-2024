use std::fs;
use day_02::{is_report_safe};

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let safe_levels = get_dampened_safe_report_count(input);

    println!("Part 1: {}", safe_levels);

    Ok(())
}

fn get_dampened_safe_report_count(input: String) -> usize {
    let level_reports = day_02::get_level_reports(input);

    let safe_levels = level_reports
        .into_iter()
        .filter(|x| is_report_safe(x) || can_report_be_dampened(x))
        .collect::<Vec<_>>()
        .len();

    safe_levels
}

fn can_report_be_dampened(report: &Vec<i32>) -> bool {
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

