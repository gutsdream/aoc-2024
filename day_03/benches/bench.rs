use std::fs;
use divan::AllocProfiler;
use day_03::{part_1, part_2};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    ).unwrap();
}

#[divan::bench]
fn part2() {
    part_2(
        divan::black_box(fs::read_to_string("./input.txt").unwrap())
    ).unwrap();
}