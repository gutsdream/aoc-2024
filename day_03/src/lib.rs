use std::cmp;

pub fn solve(input: String, disabled: &dyn Fn(usize) -> bool) -> eyre::Result<i32> {
    let max_range = input.len();
    let regex = regex::Regex::new(r"(\d{1,3}),(\d{1,3})\)")?;
    Ok(input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            if disabled(i){
                return None;
            }

            let mul_index = i + 4;
            let maximum_slice_range = cmp::min(mul_index + 3 + 1 + 3 + 1, max_range);
            let slice = &input[(mul_index)..maximum_slice_range];

            let captures = regex.captures(slice)?;
            let x : i32 = captures.get(1)?.as_str().parse().ok()?;
            let y : i32 = captures.get(2)?.as_str().parse().ok()?;

            Some( x * y )
        })
        .sum())
}

pub fn part_1(input: String) -> eyre::Result<i32> {
    solve(input, &|_| false)
}

#[derive(Debug)]
struct MulSwitch{
    index: usize,
    enabled: bool,
}

impl MulSwitch{
    pub fn enabled(index: usize) -> Self{
        MulSwitch{index, enabled: true}
    }

    pub fn disabled(index: usize) -> Self{
        MulSwitch{index, enabled: false}
    }
}

pub fn part_2(input: String) -> eyre::Result<i32> {
    let mut mul_switches : Vec<MulSwitch> = vec![];
    let do_indices  = input.match_indices("do()").into_iter();
    let do_not_indices = input.match_indices("don't()").into_iter();

    mul_switches.append(&mut do_indices.map(|(index, _)|MulSwitch::enabled(index)).collect::<Vec<MulSwitch>>());
    mul_switches.append(&mut do_not_indices.map(|(index, _)|MulSwitch::disabled(index)).collect::<Vec<MulSwitch>>());

    // Reverses list
    mul_switches.sort_by(|x, y| y.index.cmp(&x.index));

    solve(input, &|i| {
        if let Some(switch) =  mul_switches.iter().filter(|x| x.index < i).next(){
            return !switch.enabled
        }

        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_multiply_valid_statements() -> eyre::Result<()> {
        // Given
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        // When
        let result = part_1(input.to_string())?;

        // Then
        assert_eq!(result, 161);

        Ok(())
    }

    #[test]
    fn should_multiply_valid_statements_after_do_switch() -> eyre::Result<()> {
        // Given
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        // When
        let result = part_2(input.to_string())?;

        // Then
        assert_eq!(result, 48);

        Ok(())
    }
}