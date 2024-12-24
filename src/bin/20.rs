use fxhash::FxHashMap;
use itertools::Itertools;
use std::{collections::VecDeque, ops::Add};

advent_of_code::solution!(20);
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
}

#[inline]
fn taxicab_distance(point0: Point, point1: Point) -> u32 {
    point0.x.abs_diff(point1.x) + point0.y.abs_diff(point1.y)
}

fn get(grid: &[Vec<Cell>], x: i32, y: i32) -> Cell {
    grid[y as usize][x as usize]
}

fn get_adjacent(grid: &[Vec<Cell>], x: i32, y: i32) -> impl Iterator<Item = Point> + '_ {
    let width = grid.len() as i32;
    let height = grid.len() as i32;
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter(move |(delta_x, delta_y)| {
            x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
        })
        .map(move |(delta_x, delta_y)| {
            let new_x = x + delta_x;
            let new_y = y + delta_y;
            Point { x: new_x, y: new_y }
        })
}

fn parse(input: &str) -> (Vec<Vec<Cell>>, Point, Point) {
    let mut grid = vec![];
    let mut start = Point::default();
    let mut end = Point::default();

    input.trim_end().lines().enumerate().for_each(|(y, line)| {
        let mut row = vec![];
        for (x, &byte) in line.as_bytes().iter().enumerate() {
            match byte {
                b'#' => row.push(Cell::Wall),
                b'.' => row.push(Cell::Empty),
                b'S' => {
                    start = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    row.push(Cell::Empty);
                }
                b'E' => {
                    end = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    row.push(Cell::Empty);
                }
                _ => unreachable!(),
            }
        }
        grid.push(row);
    });

    (grid, start, end)
}

fn get_distances(start: &Point, end: &Point, grid: &[Vec<Cell>]) -> FxHashMap<Point, u32> {
    let mut distances = FxHashMap::default();

    let mut queue = VecDeque::from([(*start, 0)]);

    while let Some((point, distance)) = queue.pop_front() {
        if distances.contains_key(&point) {
            continue;
        }

        distances.insert(point, distance);

        if point == *end {
            continue;
        }

        for adjacent in get_adjacent(grid, point.x, point.y) {
            if get(grid, adjacent.x, adjacent.y) == Cell::Empty {
                queue.push_back((adjacent, distance + 1));
            }
        }
    }

    distances
}

fn cheating_savings(start: &Point, end: &Point, grid: &[Vec<Cell>], cheat_distance: u32) -> u32 {
    let distances = get_distances(start, end, grid);

    let mut savings = 0;
    for ((p1, c1), (p2, c2)) in distances.iter().tuple_combinations() {
        let point_distance = taxicab_distance(*p1, *p2);
        if c2.abs_diff(*c1) >= point_distance + 100 && point_distance <= cheat_distance {
            savings += 1;
        }
    }
    savings
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input);

    Some(cheating_savings(&start, &end, &grid, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input);

    Some(cheating_savings(&start, &end, &grid, 20))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
