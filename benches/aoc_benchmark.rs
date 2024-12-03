use std::hint::black_box;
use aoc2024::days::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    c.bench_function("day1 part1", |b| b.iter(||
        day01::Day01
            .part1(black_box(include_str!("../inputs/01.txt")))
    ));
    c.bench_function("day1 part2", |b| b.iter(||
        day01::Day01
            .part2(black_box(include_str!("../inputs/01.txt")))
    ));
}

fn day2(c: &mut Criterion) {
    c.bench_function("day2 part1", |b| b.iter(||
        day02::Day02
            .part1(black_box(include_str!("../inputs/02.txt")))
    ));
    c.bench_function("day2 part2", |b| b.iter(||
        day02::Day02
            .part2(black_box(include_str!("../inputs/02.txt")))
    ));
}

criterion_group!(benches, day1, day2);
criterion_main!(benches);
