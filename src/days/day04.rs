use std::error::Error;

use super::Day;

pub struct Day04;

impl Day for Day04 {
    fn part1(&self, input: &str) -> String {
        let words: Vec<Vec<char>> = input.lines()
            .map(|e|e.chars().collect::<Vec<char>>())
            .collect();

        let x_positions = get_positions(&words, 'X');

        let mut accumulator = 0;
        for (x,y) in x_positions {
            accumulator += count_xmas_all_directions((x,y),&words);
        }
        return accumulator.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let words: Vec<Vec<char>> = input.lines()
            .map(|e|e.chars().collect::<Vec<char>>())
            .collect();

        let a_positions = get_positions(&words,'A');

        let mut accumulator = 0;
        for (x,y) in a_positions {
            if is_x_mas((x,y),&words) {
                accumulator += 1;
            }
        }
        return accumulator.to_string();
    }
}

fn is_x_mas(coordinates: (usize, usize), words: &Vec<Vec<char>>) -> bool {
    let mut four_letters: Vec<(i32,i32,char)> = Vec::with_capacity(4);
    let directions: Vec<(i32,i32)> = vec![
        (1,-1),
        (1,1),
        (-1,-1),
        (-1,1),
    ];
    for (x_delta,y_delta) in directions {
        let x = coordinates.0 as i32 + x_delta;
        let y = coordinates.1 as i32 + y_delta;
        if x < 0 || y < 0 || x as usize >= words.len() || y as usize >= words[0].len() {
            // println!("out of bounds");
            return false;
        }
        let character = words.get(x as usize).unwrap().get(y as usize).unwrap();
        four_letters.push((x,y,*character));
    }
    // print!("four letters are{:?}", four_letters);
    if four_letters.iter().filter(|c| c.2=='M').count()==2 &&
    four_letters.iter().filter(|c| c.2=='S').count()==2 {
        let ms: Vec<&(i32,i32,char)> = four_letters.iter().filter(|c| c.2=='M').collect();
        let first_m = ms.first().unwrap();
        let second_m = ms.last().unwrap();
        if (first_m.0 - coordinates.0 as i32 == -(second_m.0 - coordinates.0 as i32)) &&
           (first_m.1 - coordinates.1 as i32 == -(second_m.1 - coordinates.1 as i32)) {
            // println!("malformed x-mas");
            return false;
        }
        // println!("correct");
        return true;
    }
        // println!("false");
    return false;
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


fn get_positions(words: &Vec<Vec<char>>, character: char) -> Vec<(usize,usize)> {
    let mut result: Vec<(usize,usize)> = Vec::new();
    for (i_index,i) in words.iter().enumerate() {
        for (j_index, j) in i.iter().enumerate() {
            if *j == character {
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

    #[test]
    fn part1_test() {
        let day = Day04;
        assert_eq!(day.part1(INPUT), "18");
    }

    #[test]
    fn part2_test() {
        let day = Day04;
        assert_eq!(day.part2(INPUT), "9");
    }
}
