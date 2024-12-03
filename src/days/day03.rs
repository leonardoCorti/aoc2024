use regex::Regex;

use super::Day;

pub struct Day03;

impl Day for Day03 {
    fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        let mut accumulator= 0;
        for a_match in re.find_iter(input) {
            // println!("{:?}", a_match);
            let numbers: Vec<u32> = a_match.as_str().replace("mul(", "").replace(")", "")
                .split(",")
                .map(|e| e.parse::<u32>().unwrap())
                .collect();
            accumulator += numbers[0]*numbers[1];
        }
        return accumulator.to_string();
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_test() {
        let day = Day03;
        assert_eq!(day.part1(INPUT), "161");
    }

    #[test]
    fn part2_test() {
        let day = Day03;
        assert_eq!(day.part2(INPUT), "6");
    }
}
