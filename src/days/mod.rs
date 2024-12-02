pub mod day01;
pub mod day02;

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
    }
}
