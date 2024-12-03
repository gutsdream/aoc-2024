use std::{cmp, fs};
fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let result = solve(input);

    println!("Part 2: {}", result?);

    Ok(())
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

fn solve(input: String) -> eyre::Result<i32> {

    let mut mul_switches : Vec<MulSwitch> = vec![];
    let do_indices  = input.match_indices("do()").into_iter();
    let do_not_indices = input.match_indices("don't()").into_iter();

    mul_switches.append(&mut do_indices.map(|(index, _)|MulSwitch::enabled(index)).collect::<Vec<MulSwitch>>());
    mul_switches.append(&mut do_not_indices.map(|(index, _)|MulSwitch::disabled(index)).collect::<Vec<MulSwitch>>());

    // Reverses list
    mul_switches.sort_by(|x, y| y.index.cmp(&x.index));

    let max_range = input.len();
    let regex = regex::Regex::new(r"(\d{1,3}),(\d{1,3})\)")?;

    Ok(input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            if let Some(switch) =  mul_switches.iter().filter(|x| x.index < i).next(){
                if switch.enabled == false {
                    return None;
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_multiply_valid_statements() -> eyre::Result<()> {
        // Given
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        // When
        let result = solve(input.to_string())?;

        // Then
        assert_eq!(result, 48);

        Ok(())
    }
}