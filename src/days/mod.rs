pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

use std::collections::HashMap;

pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

macro_rules! register_days {
    ($($day_num:literal => $day_mod:ident::$day_struct:ident),* $(,)?) => {{
        let mut days: HashMap<u32, Box<dyn Day>> = HashMap::new();
        $(
            days.insert($day_num, Box::new($day_mod::$day_struct));
        )*
        days
    }};
}

pub fn get_days() -> HashMap<u32, Box<dyn Day>> {
    register_days! {
        1 => day01::Day01,
        2 => day02::Day02,
        3 => day03::Day03,
        4 => day04::Day04,
        5 => day05::Day05,
        6 => day06::Day06,
        7 => day07::Day07,
        8 => day08::Day08,
        9 => day09::Day09,
        10 => day10::Day10,
        11 => day11::Day11,
    }
}
