use std::fs;
use divan::AllocProfiler;
use day_04::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1::solve(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    ).unwrap();
}

#[divan::bench]
fn part2() {
    part_2::solve(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    ).unwrap();
}