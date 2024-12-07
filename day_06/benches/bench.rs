use day_06::Puzzle;
use std::fs;
use std::str::FromStr;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = divan::black_box(fs::read_to_string("./input.txt")).unwrap();
    let puzzle = Puzzle::from_str(input.as_str()).unwrap();
    puzzle.distinct_positions_visited();
}

#[divan::bench]
fn part2() {
    let input = divan::black_box(fs::read_to_string("./input.txt").unwrap());
    let puzzle = Puzzle::from_str(input.as_str()).unwrap();
    puzzle.potential_loop_opportunities();
}
