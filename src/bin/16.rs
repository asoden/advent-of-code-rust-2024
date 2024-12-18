use std::ops::{Add, AddAssign, Sub};

use pathfinding::prelude::{astar, astar_bag, dijkstra};

advent_of_code::solution!(16);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
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

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }

    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn rotate_counter_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
    position: Point,
    direction: Direction,
}

fn taxicab_distance(point0: Point, point1: Point) -> u32 {
    point0.x.abs_diff(point1.x) + point0.y.abs_diff(point1.y)
}

fn get(grid: &[Vec<u8>], point: &Point) -> u8 {
    grid[point.y as usize][point.x as usize]
}

fn successors(reindeer: &Reindeer, grid: &[Vec<u8>]) -> Vec<(Reindeer, u32)> {
    let mut positions = vec![];

    let vector = reindeer.direction.vector();
    let ahead = reindeer.position + vector;

    if get(grid, &ahead) != b'#' {
        positions.push((
            Reindeer {
                position: ahead,
                direction: reindeer.direction,
            },
            1,
        ));
    }

    positions.push((
        Reindeer {
            position: reindeer.position,
            direction: reindeer.direction.rotate_counter_clockwise(),
        },
        1000,
    ));

    positions.push((
        Reindeer {
            position: reindeer.position,
            direction: reindeer.direction.rotate_clockwise(),
        },
        1000,
    ));

    positions
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Reindeer, Point) {
    let grid: Vec<Vec<u8>> = input
        .trim_end()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let start = Point {
        x: (grid.len() - 2) as i32,
        y: 1,
    };
    let end = Point {
        x: 1,
        y: (grid.len() - 2) as i32,
    };

    let reindeer = Reindeer {
        direction: Direction::Right,
        position: start,
    };

    (grid, reindeer, end)
}

fn find_path(grid: &[Vec<u8>], reindeer: &Reindeer, end: &Point) -> u32 {
    let (_, cost) = astar(
        reindeer,
        |reindeer| successors(reindeer, grid),
        |reindeer| taxicab_distance(reindeer.position, *end),
        |reindeer| reindeer.position == *end,
    )
    .unwrap();

    cost
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, reindeer, end) = parse(input);
    Some(find_path(&grid, &reindeer, &end))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
