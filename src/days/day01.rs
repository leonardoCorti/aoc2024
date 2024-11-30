use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part1(&self, input: &str) -> String {
        return input.chars().count().to_string();
    }

    fn part2(&self, input: &str) -> String {
        return input.chars()
            .map(|e| e.to_digit(10))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .fold(0, |a,b| a+b )
            .to_string();
    }
}
