use std::cmp::Ordering;
use std::fs;

fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let safe_levels = get_dampened_safe_report_count(input);

    println!("Part 2: {}", safe_levels);

    Ok(())
}

#[derive(Debug)]
struct Comparison {
    ordering: Ordering,
    diff: i32
}

impl Comparison {
    fn value(&self)-> i32{
        match self.ordering {
            Ordering::Less => {self.diff * -1}
            Ordering::Equal => {0}
            Ordering::Greater => {self.diff}
        }
    }
    fn simplify(&self, other: &Comparison) -> Comparison {
        let combined_value = self.value() + other.value();

        let ordering = combined_value.cmp(&0);

        Comparison{
            ordering,
            diff: i32::abs(combined_value)
        }
    }
}

fn get_dampened_safe_report_count(input: String) -> usize {
    let level_reports = day_02::get_level_reports(input);

    let safe_levels = level_reports
        .into_iter()
        .filter(|x| is_report_safe(x))
        .collect::<Vec<_>>()
        .len();

    safe_levels
}

pub fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut comparisons: Vec<Comparison> = report.windows(2)
        .map(|pair| Comparison{
            ordering: pair[0].cmp(&pair[1]),
            diff: i32::abs(pair[0] - pair[1])
        })
        .collect();

    comparisons.sort_by(|x, y| x.ordering.cmp(&y.ordering));

    let established_order = comparisons[comparisons.len() / 2].ordering;

    // dbg!(&comparisons);
    // dbg!(&established_order);

    if comparisons_meet_requirements(&established_order, &comparisons) {
        return true;
    }

    let (index, comparison) = comparisons.iter()
        .enumerate()
        .find(|(i, x)| x.ordering != established_order || x.diff <= 0 || x. diff > 3)
        .expect("Since requirements weren't met we should have at least one offending comparison");

    // If at either end of the scale, drop the offending comparison and evaluate
    if index == 0 || index == comparisons.len() - 1 {
        // println!("hello");

        return comparisons_meet_requirements(&established_order, &comparisons
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i != &index)
            .map(|(_, v)| v)
            .collect());
    }

    println!("Comparisons, before and after");
    dbg!("Comparisons", &comparisons);
    // else, if neighboured by other comparisons, simplify with the following comparison
    comparisons[index] = comparisons[index].simplify(&comparisons[index+1]);

    let dampened_comparisons = &comparisons
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i != &(index + 1)) // Need to remove the other comparison that was simplified
        .map(|(_, v)| v)
        .collect();

    dbg!("Dampened Comparisons", &dampened_comparisons);

    comparisons_meet_requirements(&established_order, &dampened_comparisons)
}

fn comparisons_meet_requirements(established_order: &Ordering, comparisons: &Vec<Comparison>) -> bool {
    let ordering_matches = comparisons.iter().all(|x| &x.ordering == established_order);
    let differences_within_range = comparisons.iter().all(| x| x.diff >= 1 && x.diff <= 3);

    ordering_matches && differences_within_range
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Greater, Less};
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

    #[test]
    fn should_combine_comparisons() {
        // Given
        let comparison_a = Comparison{
            ordering: Greater,
            diff: 1
        };

        let comparison_b = Comparison{
            ordering: Less,
            diff: 3
        };

        let expected = Comparison{
            ordering: Less,
            diff: 2
        };

        // When
        let result = comparison_a.simplify(&comparison_b);

        // Then
        assert_eq!(result.ordering, expected.ordering);
        assert_eq!(result.diff, expected.diff);
    }
}

