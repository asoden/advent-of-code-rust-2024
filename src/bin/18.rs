use std::ops::Add;

use pathfinding::prelude::dijkstra;

#[cfg(test)]
const GRID_SIZE: usize = 7;
#[cfg(not(test))]
const GRID_SIZE: usize = 71;

#[cfg(test)]
const NUM_BRICKS: usize = 12;
#[cfg(not(test))]
const NUM_BRICKS: usize = 1024;

advent_of_code::solution!(18);

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
enum Memory {
    Normal,
    Corrupted,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

fn corrupt_memory(grid: &mut [Vec<Memory>], points: &[Point], num_bricks: usize) {
    for corrupting_brick in points.iter().take(num_bricks) {
        grid[corrupting_brick.y as usize][corrupting_brick.x as usize] = Memory::Corrupted;
    }
}
fn get(grid: &[Vec<Memory>], x: i32, y: i32) -> Memory {
    grid[y as usize][x as usize]
}

fn get_adjacent(grid: &[Vec<Memory>], x: i32, y: i32) -> impl Iterator<Item = Point> + '_ {
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

fn get_successors(point: &Point, grid: &[Vec<Memory>]) -> Vec<(Point, u32)> {
    let mut potential = vec![];

    for adjacent in get_adjacent(grid, point.x, point.y) {
        if get(grid, adjacent.x, adjacent.y) == Memory::Normal {
            potential.push((adjacent, 1));
        }
    }

    potential
}

fn find_path(grid: &[Vec<Memory>]) -> Option<(Vec<Point>, u32)> {
    let end = Point {
        x: (GRID_SIZE - 1) as i32,
        y: (GRID_SIZE - 1) as i32,
    };
    dijkstra(
        &Point { x: 0, y: 0 },
        |point| get_successors(point, grid),
        |p| *p == end,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let points = parse(input);

    let mut grid = vec![vec![Memory::Normal; GRID_SIZE]; GRID_SIZE];

    corrupt_memory(&mut grid, &points, NUM_BRICKS);
    Some(find_path(&grid).unwrap().1)
}

pub fn part_two(input: &str) -> Option<String> {
    let points = parse(input);

    let mut grid = vec![vec![Memory::Normal; GRID_SIZE]; GRID_SIZE];

    let mut num_bricks = NUM_BRICKS;

    while find_path(&grid).is_some() {
        num_bricks += 1;
        corrupt_memory(&mut grid, &points, num_bricks);
    }

    Some(format!(
        "{},{}",
        points[num_bricks - 1].x,
        points[num_bricks - 1].y
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".into()));
    }
}
