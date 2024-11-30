use super::Day;

pub struct DayXX;

impl Day for DayXX {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "simpletestempty123";

    #[test]
    fn part1_test() {
        let day = DayXX;
        assert_eq!(day.part1(INPUT), "18");
    }

    #[test]
    fn part2_test() {
        let day = DayXX;
        assert_eq!(day.part2(INPUT), "6");
    }
}
