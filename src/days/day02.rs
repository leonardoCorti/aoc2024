use super::Day;

pub struct Day02;

impl Day for Day02 {
    fn part1(&self, input: &str) -> String {
        return input.lines()
            .filter(|l| is_safe(l))
            .count()
            .to_string();
    }

    fn part2(&self, input: &str) -> String {
        return input.lines()
            .filter(|l| {
                if is_safe(l) {
                    // println!("was safe naturally {l}");
                    return true;
                } else {
                    return tolerate_one_level_bruteforce(l);
                }
            })
            .count()
            .to_string();
    }
}

fn tolerate_one_level_bruteforce(l: &str) -> bool {
    let numbers: Vec<i32> = l.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    for i in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(i);
        let new_numbers_string = new_numbers.iter()
            .map(|l| l.to_string()).collect::<Vec<String>>().join(" ");
        if is_safe(&new_numbers_string) {
            // println!("was safe {new_numbers_string}");
            return true;
        }
    }
    return false;
}

#[derive(Debug)]
pub enum Order {
    Incresing,
    Decreasing,
}

fn is_safe(l: &str) -> bool {
    let numbers: Vec<i32> = l.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let mut order: Option<Order>= None;

    for (a, b) in numbers.iter().zip(numbers.iter().skip(1)) {
        if (b-a).abs() > 3 || (b-a) == 0 {
            return false;
        }
        match order {
            Some(ref ordering) => {
                match ordering {
                    Order::Incresing => {
                        if b<a {
                            return false;
                        }
                    },
                    Order::Decreasing => {
                        if b>a {
                            return false;
                        }
                    },
                }

            },
            None => {
                if a>b {
                    order = Some(Order::Decreasing);
                } else {
                    order = Some(Order::Incresing);
                }
            },
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    #[test]
    fn part1_test() {
        let day = Day02;
        assert_eq!(day.part1(INPUT), "2");
    }

    #[test]
    fn part2_test() {
        let day = Day02;
        assert_eq!(day.part2(INPUT), "4");
    }
}
