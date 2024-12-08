use std::{collections::{HashMap, HashSet}, fmt::Display};

use super::Day;

pub struct Day08;

impl Day for Day08 {
    fn part1(&self, input: &str) -> String {
        let map: Map = Map::from_str(input);
        let mut taken_positions: HashSet<(i32,i32)> = HashSet::new();
        let mut antinodes: HashSet<(i32,i32)> = HashSet::new();

        for (_, values) in &map.antennas {
            for value in values {
                taken_positions.insert(*value);
            }
        }

        for (_, values) in &map.antennas {
            for i in 0..values.len() {
                for j in (i+1)..values.len() {
                    let first_antenna = values[i];
                    let (x1,y1) = first_antenna;
                    let second_antenna = values[j];
                    let (x2,y2) = second_antenna;
                    let d = ((x2-x1),(y2-y1));
                    let an1 = ((x1-d.0),(y1-d.1));
                    let an2 = ((x2+d.0),(y2+d.1));
                    if (an1.0 < map.lenght) && (an1.1 < map.lenght)
                    && (an1.0 >= 0) && (an1.1 >= 0)
                    {
                        antinodes.insert(an1);
                    }

                    if (an2.0 < map.lenght) && (an2.1 < map.lenght)
                    && (an2.0 >= 0) && (an2.1 >= 0)
                    {
                        antinodes.insert(an2);
                    }

                }
            }
        }
        return antinodes.iter().count().to_string();
    }

    fn part2(&self, input: &str) -> String {
        let map: Map = Map::from_str(input);
        let mut taken_positions: HashSet<(i32,i32)> = HashSet::new();
        let mut antinodes: HashSet<(i32,i32)> = HashSet::new();

        for (_, values) in &map.antennas {
            for value in values {
                taken_positions.insert(*value);
            }
        }
        
        for (_, values) in &map.antennas {
            for i in 0..values.len() {
                for j in (i+1)..values.len() {
                    let first_antenna = values[i];
                    let (x1,y1) = first_antenna;
                    let second_antenna = values[j];
                    let (x2,y2) = second_antenna;
                    antinodes.insert(first_antenna);
                    antinodes.insert(second_antenna);
                    let d = ((x2-x1),(y2-y1));

                    let mut multiplier = 1;

                    loop {
                        let an1 = ((x1-(d.0*multiplier)),(y1-(d.1*multiplier)));
                        if (an1.0 < map.lenght) && (an1.1 < map.lenght)
                        && (an1.0 >= 0) && (an1.1 >= 0)
                        {
                            antinodes.insert(an1);
                        } else {
                            break;
                        }
                        multiplier += 1;
                    }
                    multiplier = 1;
                    
                    loop {
                        let an2 = ((x2+(d.0*multiplier)),(y2+(d.1*multiplier)));
                        if (an2.0 < map.lenght) && (an2.1 < map.lenght)
                        && (an2.0 >= 0) && (an2.1 >= 0)
                        {
                            antinodes.insert(an2);
                        } else {
                            break;
                        }
                        multiplier += 1;
                    }
                }
            }
        }
        return antinodes.iter().count().to_string();
    }
}

struct Map {
    antennas: HashMap<char,Vec<(i32,i32)>>,
    height: i32,
    lenght: i32,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.height {
            let mut row: Vec<char> = Vec::new();
            for _ in 0..self.lenght {
                row.push('.');
            }
            map.push(row);
        }
        for (freq, positions ) in self.antennas.iter() {
            for (posx,posy) in positions {
                map[*posx as usize][*posy as usize] = *freq;
            }
        }
        for row in map.iter() {
            for character in row {
                write!(f,"{}",character)?;
            }
            write!(f,"\n")?;
        }
        return Ok(());
    }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut antennas: HashMap<char, Vec<(i32,i32)>> = HashMap::new();
        let lenght= input.lines().next().unwrap().chars().count() as i32;
        let height= input.lines().count() as i32;
        for (i, row) in input.lines().enumerate() {
            for (j, character) in row.chars().enumerate() {
                match character {
                    '.' => {},
                    other => {
                        match antennas.get_mut(&other) {
                            Some(list) => {
                                list.push((i as i32,j as i32));
                            },
                            None => {
                                antennas.insert(other, vec![(i as i32, j as i32)]);
                            },
                        }
                    }

                }
            }
        }
        return Map{ antennas, height, lenght};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_test() {
        let day = Day08;
        assert_eq!(day.part1(INPUT), "14");
    }

    #[test]
    fn part2_test() {
        let day = Day08;
        assert_eq!(day.part2(INPUT), "34");
    }
}
