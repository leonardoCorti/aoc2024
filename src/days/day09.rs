use std::fmt::Display;

use super::Day;

pub struct Day09;

impl Day for Day09 {
    fn part1(&self, input: &str) -> String {
        let mut disk: Disk = input.into();
        // println!("{}", disk);
        disk.compact();
        // println!("{}", disk);
        return disk.checksum().to_string();
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

#[derive(PartialEq, Eq)]
enum DiskPosition {
    Empty,
    Block(i32),
}

struct Disk {
    map: Vec<DiskPosition>
}

impl Disk {
    fn checksum(&self) -> i64 {
        let result: i64 = self.map.iter().enumerate().map(|(i,val)| {
            match val {
                DiskPosition::Empty => return 0,
                DiskPosition::Block(id) => return (i as i64)*(*id as i64),
            }
        }).fold(0, |acc, b| {
                return acc + b;
            });
        return result ;
    }

    fn compact(&mut self) {
        let mut j = self.map.len();
        j = self.find_last_block(j);
        for i in 0..self.map.len() {
            // println!("{}", self);
            let current_value = &self.map[i];
            if *current_value == DiskPosition::Empty {
                self.map.swap(i, j);
                j = self.find_last_block(j);
            }
            if i == j {
                return;
            }
        }
    }

    fn find_last_block(&self, j: usize) -> usize {
        let i = if j==self.map.len() {
            j
        } else {
            let a = j+1;
            a
        };

        for index in (0..i).rev() {
            if self.map[index]!= DiskPosition::Empty {
                return index;
            }
        }
        return 0;
    }
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut map: Vec<DiskPosition> = Vec::with_capacity(value.len());

        let mut id: i32 = 0;

        let mut is_file = false;
        for character in value.chars() {
            is_file = !is_file;
            if !character.is_numeric() {
                break;
            }
            let quantity: i32 = character.to_string().parse().unwrap();
            for _ in 0..quantity {
                if is_file {
                    map.push(DiskPosition::Block(id));
                } else {
                    map.push(DiskPosition::Empty);
                }
            }                
            if is_file {
                id += 1;
            }
        }
        
        return Self { map };
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for val in self.map.iter() {
            match val {
                DiskPosition::Empty => {
                    write!(f,".")?;
                },
                DiskPosition::Block(id) => {
                    write!(f," ")?;
                    write!(f,"{}",id)?;
                    write!(f," ")?;
                },
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_test() {
        let day = Day09;
        assert_eq!(day.part1(INPUT), "1928");
    }

    #[test]
    fn part2_test() {
        let day = Day09;
        assert_eq!(day.part2(INPUT), "6");
    }
}
