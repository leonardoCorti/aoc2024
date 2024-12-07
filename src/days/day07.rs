use rayon::prelude::*;

use super::Day;

pub struct Day07;

impl Day for Day07 {
    fn part1(&self, input: &str) -> String {
        let equations: Vec<Equation> = input.lines()
            .map(|e| e.into()).collect();
        // equations.iter().for_each(|e| println!("{e:?}"));
        let result: i64 = equations.iter()
            .filter(|e| e.exists_solution())
                .map(|e| e.result)
                .sum();
        return result.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let equations: Vec<Equation> = input.lines()
            .map(|e| e.into()).collect();
        let result: i64 = equations.iter()
            .filter(|e| e.exists_solution_concat())
                .map(|e| e.result)
                .sum();
        return result.to_string();
    }
}

#[derive(Debug)]
enum Operators {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug)]
struct Equation {
    result: i64,
    values: Vec<i64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut two_parts = value.split(": ");
        let result: i64 = two_parts.next().unwrap().parse().unwrap();
        let values: Vec<i64> = two_parts.next().unwrap()
            .split(" ").map(|e| e.parse().unwrap())
            .collect();
        return Equation { result, values }
    }
}

impl Equation {
    fn exists_solution(&self) -> bool {
        let num_values = self.values.len();
        if num_values < 2 {
            return false;
        }

        let total_combinations = 2_usize.pow((num_values - 1) as u32);

        return (0..total_combinations).into_par_iter().any(|i| {
            let mut ops = Vec::new();
            let mut n = i;

            for _ in 0..(num_values - 1) {
                match n % 2 {
                    0 => ops.push(Operators::Add),
                    1 => ops.push(Operators::Multiply),
                    _ => unreachable!(),
                }
                n /= 2;
            }

            let mut result = self.values[0];
            for (i, op) in ops.iter().enumerate() {
                match op {
                    Operators::Add => result += self.values[i + 1],
                    Operators::Multiply => result *= self.values[i + 1],
                    Operators::Concat => {
                        unreachable!()
                    }
                }
            }

            result == self.result
        });
    }

    fn exists_solution_concat(&self) -> bool {
        let num_values = self.values.len();
        if num_values < 2 {
            return false;
        }

        let total_combinations = 3_usize.pow((num_values - 1) as u32);

        return (0..total_combinations).into_par_iter().any(|i| {
            let mut ops = Vec::new();
            let mut n = i;

            for _ in 0..(num_values - 1) {
                match n % 3 {
                    0 => ops.push(Operators::Add),
                    1 => ops.push(Operators::Multiply),
                    2 => ops.push(Operators::Concat),
                    _ => unreachable!(),
                }
                n /= 3;
            }

            let mut result = self.values[0];
            for (i, op) in ops.iter().enumerate() {
                match op {
                    Operators::Add => result += self.values[i + 1],
                    Operators::Multiply => result *= self.values[i + 1],
                    Operators::Concat => {
                        let concatenated = format!("{}{}", result, self.values[i + 1]);
                        result = concatenated.parse::<i64>().unwrap();
                    }
                }
            }

            result == self.result
        });
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn part1_test() {
        let day = Day07;
        assert_eq!(day.part1(INPUT), "3749");
    }

    #[test]
    fn part2_test() {
        let day = Day07;
        assert_eq!(day.part2(INPUT), "11387");
    }
}
