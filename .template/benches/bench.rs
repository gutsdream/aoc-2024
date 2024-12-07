use std::fs;
use divan::AllocProfiler;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = divan::black_box(fs::read_to_string("./input.txt")).unwrap();
    let puzzle = Puzzle::from_str(input.as_str()).unwrap();
    puzzle.part_1();
}

#[divan::bench]
fn part2() {
    let input = divan::black_box(fs::read_to_string("./input.txt").unwrap());
    let puzzle = Puzzle::from_str(input.as_str()).unwrap();
    puzzle.part_2();
}