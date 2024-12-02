use std::cmp::Ordering;
use std::env::current_dir;
use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let safe_levels = get_safe_level_report_count(input);

    println!("Part 1: {}", safe_levels);

    Ok(())
}

fn get_safe_level_report_count(input: String) -> usize {
    let level_reports = day_02::get_level_reports(input);

    let safe_levels = level_reports
        .into_iter()
        .map(|x| {
            x.windows(2)
                .map(|pair| (pair[0].cmp(&pair[1]), i32::abs(pair[0] - pair[1])))
                .collect::<Vec<(Ordering, i32)>>()
        })
        .filter(|x|
            x.iter().all(|(ord, _)| ord == &Ordering::Less) ||
                x.iter().all(|(ord, _)| ord == &Ordering::Greater))
        .filter(|x| x.iter().all(|(_, diff)| diff >= &1 && diff <= &3))
        .collect::<Vec<_>>()
        .len();
    safe_levels
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
