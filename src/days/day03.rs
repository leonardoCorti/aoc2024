use std::cmp::min;

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
        let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        let do_re = Regex::new("do()").unwrap();
        let dont_re = Regex::new("don't()").unwrap();
        let do_positions: Vec<usize> = do_re.find_iter(input)
            .map(|e| e.start()).collect();
        let dont_positions: Vec<usize> = dont_re.find_iter(input)
            .map(|e| e.start()).collect();
        let mut accumulator= 0;
        for a_match in mul_re.find_iter(input) {
            // println!("{:?}", a_match);
            let numbers: Vec<u32> = a_match.as_str().replace("mul(", "").replace(")", "")
                .split(",")
                .map(|e| e.parse::<u32>().unwrap())
                .collect();
            let position = a_match.start(); 
            //defaults to do()
            if position < min(*do_positions.first().unwrap(), *dont_positions.first().unwrap()) {
                accumulator += numbers[0]*numbers[1];
            } else {
                //check boundaries
                let do_before: Vec<&usize> = do_positions.iter().filter(|e| **e<position).collect();
                let dont_before: Vec<&usize> = dont_positions.iter().filter(|e| **e<position).collect();
                if do_before.last().unwrap() > dont_before.last().unwrap() {
                    accumulator += numbers[0]*numbers[1];
                } else {
                    //do nothing
                }

            }
        }
        return accumulator.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_test() {
        let day = Day03;
        assert_eq!(day.part1(INPUT), "161");
    }

    #[test]
    fn part2_test() {
        let day = Day03;
        assert_eq!(day.part2(INPUT2), "48");
    }
}
