use std::ops::{Add, AddAssign, Sub, SubAssign};

use fxhash::{FxHashMap, FxHashSet};

advent_of_code::solution!(8);

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_inbounds(&self, bounds: i32) -> bool {
        self.x >= 0 && self.x < bounds && self.y >= 0 && self.y < bounds
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse(input: &str) -> (FxHashMap<u8, Vec<Point>>, i32) {
    let mut antennas = FxHashMap::default();
    let size = input
        .find("\n")
        .expect("input delimited by new lines for grid")
        .try_into()
        .expect("Problem space not larger than what i32 can handle");

    input.trim_end().lines().enumerate().for_each(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, character)| **character != b'.')
            .for_each(|(x, character)| {
                antennas
                    .entry(*character)
                    .and_modify(|list: &mut Vec<_>| list.push(Point::new(x as i32, y as i32)))
                    .or_insert(vec![Point::new(x as i32, y as i32)]);
            });
    });

    (antennas, size)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, grid_size) = parse(input);
    let mut antinodes = FxHashSet::default();

    antennas.iter().for_each(|(_, entry)| {
        for i in 0..entry.len() {
            for j in i..entry.len() {
                if entry[i] != entry[j] {
                    let diff = entry[i] - entry[j];
                    let potential_node = entry[i] + diff;
                    if potential_node.is_inbounds(grid_size) {
                        antinodes.insert(potential_node);
                    }
                    let potential_node = entry[j] - diff;
                    if potential_node.is_inbounds(grid_size) {
                        antinodes.insert(potential_node);
                    }
                }
            }
        }
    });

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas, grid_size) = parse(input);
    let mut antinodes = FxHashSet::default();

    antennas.iter().for_each(|(_, entry)| {
        for i in 0..entry.len() {
            for j in i + 1..entry.len() {
                let diff = entry[i] - entry[j];
                let mut potential_node = entry[i] + diff;
                while potential_node.is_inbounds(grid_size) {
                    antinodes.insert(potential_node);
                    potential_node += diff;
                }
                let mut potential_node = entry[j] - diff;
                while potential_node.is_inbounds(grid_size) {
                    antinodes.insert(potential_node);
                    potential_node -= diff;
                }
            }
            antinodes.insert(entry[i]);
        }
    });

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
