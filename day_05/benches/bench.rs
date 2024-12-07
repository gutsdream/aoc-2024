use std::fs;
use divan::AllocProfiler;
use day_05::Puzzle;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = divan::black_box(fs::read_to_string("./input.txt").unwrap());
    Puzzle::from(input.as_str()).sum_of_correct_updates();
}

#[divan::bench]
fn part2() {
    let input = divan::black_box(fs::read_to_string("./input.txt").unwrap());
    Puzzle::from(input.as_str()).sum_of_incorrect_updates();
}