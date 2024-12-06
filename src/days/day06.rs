use core::panic;
use std::fmt::Display;

use super::Day;

pub struct Day06;

impl Day for Day06 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::from_string(input);
        while !map.is_guard_finished() {
            map.move_guard();
        }
        return map.count_visited().to_string();
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

#[derive(Eq,PartialEq)]
enum Position {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Eq,PartialEq,Clone,Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for (i32,i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1,0),
            Direction::Down => (1,0),
            Direction::Right => (0,1),
            Direction::Left => (0,-1),
        }
    }
}

struct Map {
    map: Vec<Vec<Position>>,
    guard_position: (i32,i32),
    guard_direction: Direction,
}

impl Map {
    fn from_string(input: &str) -> Self {
        let mut map = Vec::new();
        let mut guard_position = (0, 0);
        let mut guard_direction = Direction::Up;

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => row.push(Position::Empty),
                    '#' => row.push(Position::Obstacle),
                    '^' => {
                        row.push(Position::Empty);
                        guard_position = (y as i32, x as i32);
                        guard_direction = Direction::Up;
                    }
                    _ => panic!("Invalid character in map string: {}", ch),
                }
            }
            map.push(row);
        }

        Self {
            map,
            guard_position,
            guard_direction,
        }
    }

    fn is_guard_finished(&self) -> bool {
        let (x,y) = self.guard_position;
        if x==0 {
            return true;
        }
        else if y==0{
            return true;
        }
        else if x==(self.map.len() as i32-1){
            return true;
        }
        else if y==(self.map[0].len() as i32-1){
            return true;
        } else {
            return false;
        }
    }

    fn count_visited(&self) -> u32 {
        self.map.iter()
            .map(|e| {
                e.iter()
                    .filter(|e| **e==Position::Visited)
                    .count() as u32
            }).sum()      
    }

    fn move_guard(&mut self) {
        let (mut x, mut y) = self.guard_position;
        let (delta_x, delta_y): (i32, i32) = self.guard_direction.into();

        loop {
            let next_row = x + delta_x;
            let next_col = y + delta_y;

            if next_row < 0
                || next_row >= self.map.len() as i32
                || next_col < 0
                || next_col >= self.map[0].len() as i32
            {
                break;
            }

            match self.map[next_row as usize][next_col as usize] {
                Position::Obstacle => break,
                Position::Empty => {
                    self.map[x as usize][y as usize] = Position::Visited;

                    x = next_row;
                    y = next_col;
                }
                Position::Visited => {
                    self.map[x as usize][y as usize] = Position::Visited;
                    x = next_row;
                    y = next_col;
                }
            }
        }

        self.guard_position = (x, y);

        if self.map[x as usize][y as usize] == Position::Empty {
            self.map[x as usize][y as usize] = Position::Visited;
        }
        self.guard_direction = match self.guard_direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

}

impl Display for Map{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.iter() {
            for pos in line.iter() {
                match pos {
                    Position::Empty => write!(f,".")?,
                    Position::Obstacle => write!(f,"#")?,
                    Position::Visited => write!(f,"X")?,
                }
            }
            write!(f,"\n")?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_test() {
        let day = Day06;
        assert_eq!(day.part1(INPUT), "41");
    }

    #[test]
    fn part2_test() {
        let day = Day06;
        assert_eq!(day.part2(INPUT), "6");
    }
}
