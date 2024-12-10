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

fn day5(c: &mut Criterion) {
    c.bench_function("day5 part1", |b| b.iter(||
        day05::Day05
            .part1(black_box(include_str!("../inputs/05.txt")))
    ));
    c.bench_function("day5 part2", |b| b.iter(||
        day05::Day05
            .part2(black_box(include_str!("../inputs/05.txt")))
    ));
}

fn day6(c: &mut Criterion) {
    c.bench_function("day6 part1", |b| b.iter(||
        day06::Day06
            .part1(black_box(include_str!("../inputs/06.txt")))
    ));
    c.bench_function("day6 part2", |b| b.iter(||
        day06::Day06
            .part2(black_box(include_str!("../inputs/06.txt")))
    ));
}

fn day7(c: &mut Criterion) {
    c.bench_function("day7 part1", |b| b.iter(||
        day07::Day07
            .part1(black_box(include_str!("../inputs/07.txt")))
    ));
    c.bench_function("day7 part2", |b| b.iter(||
        day07::Day07
            .part2(black_box(include_str!("../inputs/07.txt")))
    ));
}

fn day8(c: &mut Criterion) {
    c.bench_function("day8 part1", |b| b.iter(||
        day08::Day08
            .part1(black_box(include_str!("../inputs/08.txt")))
    ));
    c.bench_function("day8 part2", |b| b.iter(||
        day08::Day08
            .part2(black_box(include_str!("../inputs/08.txt")))
    ));
}

fn day9(c: &mut Criterion) {
    c.bench_function("day9 part1", |b| b.iter(||
        day09::Day09
            .part1(black_box(include_str!("../inputs/09.txt")))
    ));
    c.bench_function("day9 part2", |b| b.iter(||
        day09::Day09
            .part2(black_box(include_str!("../inputs/09.txt")))
    ));
}

fn day10(c: &mut Criterion) {
    c.bench_function("day10 part1", |b| b.iter(||
        day10::Day10
            .part1(black_box(include_str!("../inputs/10.txt")))
    ));
    // c.bench_function("day10 part2", |b| b.iter(||
    //     day10::Day10
    //         .part2(black_box(include_str!("../inputs/10.txt")))
    // ));
}

criterion_group!(benches, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10);
criterion_main!(benches);
