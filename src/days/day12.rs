use std::collections::HashSet;

use tracing::{debug, error, info, instrument, trace, warn};
use tracing_test::traced_test;

use std::cmp::min;

use super::Day;

pub struct Day12;

const DIRECTIONS: [(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];

impl Day for Day12 {
    fn part1(&self, input: &str) -> String {
        let garden: Vec<Vec<char>> = input.lines()
            .map(|e| e.chars().collect())
            .collect();

        let regions = find_regions(&garden);
        debug!("regions are: ");
        for region in regions.iter() {
            debug!("{}", region.print(&garden));
        }
        debug!("there are {} regions", regions.len() );
        let result: u32 = regions.iter().map(|e| e.area*e.perimeter).sum();
        return result.to_string();
    }
    //should finish with an iter of regions, map to multiply the perimitere and area, the sum it 

    fn part2(&self, input: &str) -> String {
        return input.chars()
            .map(|e| e.to_digit(10))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .fold(0, |a,b| a+b )
            .to_string();
    }
}

#[tracing::instrument(skip(input))]
fn find_regions(input: &Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<(usize,usize)> = HashSet::new();
    for i in 0..input.len() {
        for j in 0..input.len() {
            if !visited.contains(&(i,j)) {
                debug!("found new region");
                let mut coordinates: Vec<(usize,usize)> = Vec::new();
                expand_region(input,(i,j), &mut visited, &mut coordinates);
                let region = Region::new(&coordinates);
                regions.push(region);
            }
        }
    }
    return regions;
}

#[tracing::instrument(skip(input,visited,container))]
fn expand_region(
    input: &Vec<Vec<char>>,
    point: (usize,usize),
    visited: &mut HashSet<(usize,usize)>,
    container: &mut Vec<(usize,usize)>
) {
    trace!("now entering {:?}", point);
    visited.insert(point);
    container.push(point);

    let max = input.len();
    let x = point.0;
    let y = point.1;
    let neighbors = [
        (if x>0 {x-1} else {0}, y),
        (min(x + 1,max-1), y),
        (x, if y>0 {y-1} else {0}),
        (x, min(y + 1,max-1))
    ];
    let current_char_value = input[x][y];

    for neighbor in neighbors {
        if !visited.contains(&neighbor) {
            let neighbor_value = input[neighbor.0][neighbor.1];
            if current_char_value == neighbor_value {
                expand_region(input, neighbor, visited, container);
            }
        }
    }
}



#[derive(Debug)]
struct Region {
    coordinates: Vec<(usize,usize)>,
    area: u32,
    perimeter: u32,
}

impl Region {
    fn new(input: &[(usize,usize)]) -> Region {
        let area = input.len() as u32;
        let mut perimeter = 0;

        let coord_set: HashSet<_> = input.iter().cloned().collect();
        for &(x, y) in input {
            let neighbors = [
                (if x>0 {x-1} else {0}, y),
                (x + 1, y),
                (x, if y>0 {y-1} else {0}),
                (x, y + 1)
            ];
            if x == 0 {
                perimeter += 1;
            }
            if y == 0 {
                perimeter += 1;
            }
            perimeter += neighbors
                .iter()
                .filter(|&&(nx, ny)| !coord_set.contains(&(nx, ny)))
                .count() as u32;
        }

        Region {
            coordinates: input.to_vec(),
            area,
            perimeter,
        }
    }

    fn print(&self, map: &Vec<Vec<char>>) -> String {
        let value = map[self.coordinates[0].0][self.coordinates[0].1];

        let coordinates_str = format!("{:?}",self.coordinates);
        return format!(
            "coordinates: {},\n perimeter: {}, area: {}, value: {}",
            coordinates_str, self.perimeter,self.area,value
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    #[traced_test]
    fn part1_test() {
        let day = Day12;
        assert_eq!(day.part1(INPUT), "1930");
    }

    #[test]
    fn part2_test() {
        let day = Day12;
        assert_eq!(day.part2(INPUT), "6");
    }
}
