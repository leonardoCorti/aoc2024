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

fn day3(c: &mut Criterion) {
    c.bench_function("day3 part1", |b| b.iter(||
        day03::Day03
            .part1(black_box(include_str!("../inputs/03.txt")))
    ));
    c.bench_function("day3 part2", |b| b.iter(||
        day03::Day03
            .part2(black_box(include_str!("../inputs/03.txt")))
    ));
}

fn day4(c: &mut Criterion) {
    c.bench_function("day4 part1", |b| b.iter(||
        day04::Day04
            .part1(black_box(include_str!("../inputs/04.txt")))
    ));
    c.bench_function("day4 part2", |b| b.iter(||
        day04::Day04
            .part2(black_box(include_str!("../inputs/04.txt")))
    ));
}

criterion_group!(benches, day1, day2, day3, day4);
criterion_main!(benches);
