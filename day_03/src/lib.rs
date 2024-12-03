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