use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();
        for line in input.lines(){
            let two_numbers: Vec<&str> = line.split_whitespace().collect();
            let first_number: i32 = two_numbers.first()
                .expect("couldn't find first number").parse().unwrap();
            let second_number: i32 = two_numbers.last()
                .expect("couldn't find second number").parse().unwrap();
            left_numbers.push(first_number);
            right_numbers.push(second_number);
        }

        left_numbers.sort(); 
        right_numbers.sort(); 

        let mut accumulator: i32 = 0; 
        for (index,number_left) in left_numbers.iter().enumerate() {
            let number_right = right_numbers.get(index)
                .expect("couldn't find corrispondent number in right array");
            let distance = number_right - number_left;
            accumulator += distance.abs();
            
        }

        return accumulator.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();
        for line in input.lines(){
            let two_numbers: Vec<&str> = line.split_whitespace().collect();
            let first_number: i32 = two_numbers.first()
                .expect("couldn't find first number").parse().unwrap();
            let second_number: i32 = two_numbers.last()
                .expect("couldn't find second number").parse().unwrap();
            left_numbers.push(first_number);
            right_numbers.push(second_number);
        }

        let similarity: i32 = left_numbers.iter().map(|e| {
            let similarity: usize = right_numbers.iter().filter(|n| *n==e).count();
            return *e * (similarity as i32);
        }).sum();
        return similarity.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn part1_test() {
        let day = Day01;
        assert_eq!(day.part1(INPUT), "11");
    }

    #[test]
    fn part2_test() {
        let day = Day01;
        assert_eq!(day.part2(INPUT), "31");
    }
}
