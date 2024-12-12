use std::collections::HashMap;

use rayon::prelude::*;

use super::Day;

pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: &str) -> String {
        let stones: Vec<u64> = input.split_whitespace()
            .map(|e| e.parse::<u64>().unwrap()).collect();

        let count: u64 = stones.par_iter()
            .map(|e| {
                let mut memo = HashMap::new();
                count_blink(*e, 0, 25, &mut memo)
            })
            .sum();

        return count.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let stones: Vec<u64> = input.split_whitespace()
            .map(|e| e.parse::<u64>().unwrap()).collect();

        let mut memo = HashMap::new();
        let count: u64 = stones.iter()
            .map(|e| {
                let result = count_blink(*e, 0, 75, &mut memo);
                return result;
            })
            .sum();

        return count.to_string();
    }
}

//v3
fn count_blink(
    stone: u64,
    current_blink: i32,
    max: i32,
    memo: &mut HashMap<(u64,i32), u64>,
) -> u64 {
    if current_blink == max {
        return 1;
    }

    if let Some(&cached) = memo.get(&(stone,current_blink)) {
        return cached;
    }

    let result = if stone == 0 {
        count_blink(1, current_blink+1, max, memo)
    } else {

        let digits = stone.to_string();
        if digits.len() % 2 == 0 {
            let midpoint = digits.len() / 2;
            let left_part: u64 = digits[..midpoint].parse().unwrap();
            let right_part: u64 = digits[midpoint..].parse().unwrap();
            count_blink(left_part, current_blink+1, max,memo) +
            count_blink(right_part, current_blink+1, max,memo)
        }  else {
            count_blink(stone*2024, current_blink+1, max,memo)
        }
    };

    memo.insert((stone,current_blink), result);
    return result;
}

// v2
// fn blink_with_memory(stones: &mut Vec<u64>, memory: &mut HashMap<u64,Vec<u64>>) {
//     let mut new_stones: Vec<u64> = Vec::new();
//     for number in stones.iter() {
//         let blinked_number = apply_rules_with_memory(*number, memory);
//         for new_number in blinked_number.iter() {
//             new_stones.push(*new_number);
//         }
//     }
//     *stones = new_stones;
// }
//
// fn apply_rules_with_memory(number: u64, memory: &mut HashMap<u64,Vec<u64>>) -> Vec<u64> {
//     match memory.get(&number) {
//         Some(result) => return result.to_vec(),
//         None => {},
//     }
//
//     if number == 0 {
//         let vec = vec![1];
//         memory.insert(number, vec.clone());
//         return vec;
//     }
//     
//     let digits = number.to_string();
//     if digits.len() % 2 == 0 {
//         let midpoint = digits.len() / 2;
//         let left_part: u64 = digits[..midpoint].parse().unwrap();
//         let right_part: u64 = digits[midpoint..].parse().unwrap();
//         let v = vec![left_part, right_part];
//         memory.insert(number, v.clone());
//         return v;
//     }
//
//     let v = vec![number * 2024];
//     memory.insert(number, v.clone());
//     return v;
// }

// v1
// fn blink(stones: &mut Vec<u64>) {
//     let mut new_stones: Vec<u64> = Vec::new();
//     for number in stones.iter() {
//         let blinked_number = apply_rules(*number);
//         for new_number in blinked_number.iter() {
//             new_stones.push(*new_number);
//         }
//     }
//     *stones = new_stones;
// }
//
// fn apply_rules(number: u64) -> Vec<u64> {
//     if number == 0 {
//         return vec![1];
//     }
//     
//     let digits = number.to_string();
//     if digits.len() % 2 == 0 {
//         let midpoint = digits.len() / 2;
//         let left_part: u64 = digits[..midpoint].parse().unwrap();
//         let right_part: u64 = digits[midpoint..].parse().unwrap();
//         return vec![left_part, right_part];
//     }
//
//     return vec![number * 2024];
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="125 17";

    #[test]
    fn part1_test() {
        let day = Day11;
        assert_eq!(day.part1(INPUT), "55312");
    }

    #[test]
    fn part2_test() {
        let day = Day11;
        assert_eq!(day.part2(INPUT), "6");
    }
}
