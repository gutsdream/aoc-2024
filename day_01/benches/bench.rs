use std::fs;
use divan::AllocProfiler;
use day_01::{calculate_similarity_score, calculate_total_distance};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    calculate_total_distance(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    );
}

#[divan::bench]
fn part2() {
    calculate_similarity_score(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    );
}