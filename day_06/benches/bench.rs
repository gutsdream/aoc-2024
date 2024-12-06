use std::fs;
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = divan::black_box(fs::read_to_string("./input.txt").unwrap());
    let puzzle = Puzzle::from(input.as_str());
    puzzle.part_1();
}

#[divan::bench]
fn part2() {
    let input = divan::black_box(fs::read_to_string("./input.txt").unwrap());
    let puzzle = Puzzle::from(input.as_str());
    puzzle.part_2();
}