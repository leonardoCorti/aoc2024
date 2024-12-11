use std::collections::HashSet;

use super::Day;

pub struct Day10;

const DIRECTIONS: [(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];

impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        let map = Map::new(input);
        // println!("{:?}", map.map);
        let result: u64 = map.trailheads.iter()
            .map(|e| map.get_score(*e))
            .sum();
        return result.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let map = Map::new(input);
        // println!("{:?}", map.map);
        let result: u64 = map.trailheads.iter()
            .map(|e| map.get_score_v2(*e))
            .sum();
        return result.to_string();
    }
}

struct Map {
    map: Vec<Vec<u8>>,
    trailheads: Vec<(usize,usize)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut trailheads: Vec<(usize,usize)> = Vec::new();

        for (i,line) in input.lines().enumerate() {
            let mut line_vec: Vec<u8> = Vec::new();
            for (j,pos) in line.chars().enumerate() {
                let pos_value: u8 = pos.to_string().parse().unwrap();
                if pos_value == 0 {
                    trailheads.push((i,j));
                }
                line_vec.push(pos_value);
            }
            map.push(line_vec);
        }

        return Map { map, trailheads };
    }

    fn get_score(&self, trailhead: (usize,usize)) -> u64 {
        let mut visited: HashSet<(usize,usize)> = HashSet::new();
        let score = self.travel(trailhead, &mut visited);
        // print!("the trailhead {trailhead:?} has score ");
        // println!("{score}");
        return score;
    }

    fn travel(&self, position: (usize,usize), visited: &mut  HashSet<(usize,usize)>) -> u64 {
        let mut result: u64 = 0;
        let position_value = self.get_position(position);
        if position_value == 9 {
            if visited.contains(&position) {
                return 0;
            } else {
                visited.insert(position);
                return 1;
            }
        }

        for direction in DIRECTIONS {
            let new_position: (i32,i32) = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
            if self.is_outside_bounds(new_position) {
                continue;
            } else {
                let new_position: (usize,usize) = (new_position.0 as usize, new_position.1 as usize);
                let new_position_value = self.get_position(new_position);

                if position_value+1 == new_position_value {
                    // println!("old is {:?} {:?}, new is {:?} {:?}",
                        // position, position_value, new_position, new_position_value);
                    result += self.travel(new_position, visited);
                }
            }
            // println!("now I am at {position:?}, {result}");
        }
        return result;
    }

    fn get_score_v2(&self, trailhead: (usize,usize)) -> u64 {
        let score = self.travel_v2(trailhead);
        // print!("the trailhead {trailhead:?} has score ");
        // println!("{score}");
        return score;
    }

    fn travel_v2(&self, position: (usize,usize)) -> u64 {
        let mut result: u64 = 0;
        let position_value = self.get_position(position);
        if position_value == 9 {
            return 1;
        }

        for direction in DIRECTIONS {
            let new_position: (i32,i32) = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
            if self.is_outside_bounds(new_position) {
                continue;
            } else {
                let new_position: (usize,usize) = (new_position.0 as usize, new_position.1 as usize);
                let new_position_value = self.get_position(new_position);

                if position_value+1 == new_position_value {
                    // println!("old is {:?} {:?}, new is {:?} {:?}",
                        // position, position_value, new_position, new_position_value);
                    result += self.travel_v2(new_position);
                }
            }
            // println!("now I am at {position:?}, {result}");
        }
        return result;
    }

    fn is_outside_bounds(&self, new_position: (i32, i32)) -> bool {
        new_position.0 < 0 || new_position.1 < 0 ||new_position.0 >= self.map.len().try_into().unwrap() || new_position.1 >= self.map.len().try_into().unwrap()
    }

    fn get_position(&self, position: (usize,usize)) -> u8 {
        return self.map[position.0][position.1];
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_test() {
        let day = Day10;
        assert_eq!(day.part1(INPUT), "36");
    }

    #[test]
    fn part2_test() {
        let day = Day10;
        assert_eq!(day.part2(INPUT), "81");
    }
}
