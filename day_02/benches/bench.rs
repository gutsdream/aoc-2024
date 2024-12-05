use std::fs;
use divan::AllocProfiler;
use day_02::{get_dampened_safe_report_count, get_safe_level_report_count};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    get_safe_level_report_count(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    );
}

#[divan::bench]
fn part2() {
    get_dampened_safe_report_count(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    );
}