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
        let disk: Disk = input.into();
        let mut disk = disk.convert_v2();
        disk.compact();
        let disk = disk.convert_v1();
        return disk.checksum().to_string();
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum DiskPosition {
    Empty,
    Block(i32),
}

#[derive(Debug)]
struct Disk {
    map: Vec<DiskPosition>
}

#[derive(Debug)]
struct DiskV2 {
    map: Vec<(DiskPosition, i32)>,
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

    fn convert_v2(&self) -> DiskV2 {
        let mut map: Vec<(DiskPosition,i32)> = Vec::new();

        let mut previous: Option<DiskPosition> = Some(self.map[0].clone());
        let mut count = 0;
        for val in self.map.iter() {
            if Some(val) == previous.as_ref() {
                count += 1;
            } else {
                map.push((previous.unwrap(),count));
                previous = Some(val.clone());
                count = 1;
            }
        }
        map.push((previous.unwrap(),count));

        return DiskV2 { map };
    }

}

impl DiskV2 {
    fn compact(&mut self) {
        loop {
            let mut changed = false;
            for i in (0..self.map.len()).rev() {
                let element = self.map[i].clone();
                if element.0 == DiskPosition::Empty {
                    continue;
                }
                // println!("analyzing {i} {element:?}");
                for j in 0..i {
                    let second_element = self.map[j].clone();

                    if second_element.0 != DiskPosition::Empty {
                        continue;
                    }

                    if element.1 < second_element.1 {
                        // println!("could insert {i},{:?} {j},{:?}", element, second_element);
                        self.map[i] = (DiskPosition::Empty,element.1);
                        self.map[j] = (DiskPosition::Empty, second_element.1-element.1);
                        self.map.insert(j, (element.0.clone(),element.1));
                        // println!("done");
                        changed = true;
                        break;
                    } else if element.1 == second_element.1 {
                        // println!("could swap {i},{:?} {j},{:?}", element, second_element);
                        self.map.swap(i, j);
                        changed = true;
                        break;
                    }
                }

                if changed {
                    break;
                }
            }

            if !changed {
                break;
            }
            // println!("{}", self);
        }
    }

    fn convert_v1(&self) -> Disk {
        let mut map: Vec<DiskPosition> = Vec::new();
        for (val, count) in self.map.iter() {
            for _ in 0..*count {
                map.push(val.clone());
            }
        }
        return Disk { map };
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

impl Display for DiskV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (val, count) in self.map.iter() {
            match val {
                DiskPosition::Empty => {
                    for _ in 0..*count {
                        write!(f,".")?;
                    }
                },
                DiskPosition::Block(id) => {
                    for _ in 0..*count {
                        write!(f," ")?;
                        write!(f,"{}", id)?;
                        write!(f," ")?;
                    }
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
        assert_eq!(day.part2(INPUT), "2858");
    }
}
