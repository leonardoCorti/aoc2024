use std::error::Error;

use super::Day;

pub struct Day04;

impl Day for Day04 {
    fn part1(&self, input: &str) -> String {
        let words: Vec<Vec<char>> = input.lines()
            .map(|e|e.chars().collect::<Vec<char>>())
            .collect();

        let x_positions = get_x_positions(&words);

        let mut accumulator = 0;
        for (x,y) in x_positions {
            accumulator += count_xmas_all_directions((x,y),&words);
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

fn count_xmas_all_directions(coordinates: (usize, usize), words: &Vec<Vec<char>>) -> i32 {
    let mut accumulator = 0;
    let directions: Vec<(i32,i32)> = vec![
        (1,0),
        (-1,0),
        (0,-1),
        (0,1),
        (1,-1),
        (-1,-1),
        (1,1),
        (-1,1),
    ];
    for direction in directions {
        if search_xmas(coordinates, words, direction).is_ok() {
            accumulator +=1;
        }
    }
    return accumulator;
}

fn search_xmas(
    coordinates: (usize, usize),
    words: &Vec<Vec<char>>,
    direction: (i32,i32)
) -> Result<(),Box<dyn Error>> {
    let xmas: Vec<char> = vec!['X','M','A','S'];
    let mut x = coordinates.0 as i32;
    let mut y = coordinates.1 as i32;
    for &character in &xmas {
        if x < 0 || y < 0 || x as usize >= words.len() || y as usize >= words[0].len() {
            return Err(Box::from("Out of bounds"));
        }

        if words[x as usize][y as usize] != character {
            return Err(Box::from("Character mismatch"));
        }

        x += direction.0;
        y += direction.1;
    }
    return Ok(());
}


fn get_x_positions(words: &Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let mut result: Vec<(usize,usize)> = Vec::new();
    for (i_index,i) in words.iter().enumerate() {
        for (j_index, j) in i.iter().enumerate() {
            if *j == 'X' {
                result.push((i_index,j_index));
            }        
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    const INPUT2: &str = 
".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

    #[test]
    fn part1_test() {
        let day = Day04;
        assert_eq!(day.part1(INPUT), "18");
    }

    #[test]
    fn part2_test() {
        let day = Day04;
        assert_eq!(day.part2(INPUT2), "9");
    }
}
