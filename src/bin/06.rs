use std::collections::btree_set::IntoIter;

use fxhash::FxHashSet;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
enum Cell {
    Obstruction,
    Open,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn advance(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }

    fn rotate(&mut self) {
        self.direction.rotate();
    }
}

fn parse(input: &str) -> (Guard, Vec<Vec<Cell>>) {
    let mut guard = Guard::default();

    let grid = input
        .trim_end()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '#' => Cell::Obstruction,
                    '.' => Cell::Open,
                    '^' => {
                        guard.x = x;
                        guard.y = y;
                        Cell::Open
                    }
                    _ => panic!("Impossible character encountered"),
                })
                .collect()
        })
        .collect();

    (guard, grid)
}

fn get_grid_at_pos(guard: &Guard, grid: &[Vec<Cell>]) -> Cell {
    grid[guard.y][guard.x]
}

fn look_ahead(guard: &Guard, grid: &[Vec<Cell>]) -> Option<Cell> {
    if (guard.x == 0 && guard.direction == Direction::Left)
        || (guard.y == 0 && guard.direction == Direction::Up)
        || (guard.x == grid[0].len() - 1 && guard.direction == Direction::Right)
        || (guard.y == grid.len() - 1 && guard.direction == Direction::Down)
    {
        return None;
    }

    let mut clone = guard.clone();
    clone.advance();
    Some(get_grid_at_pos(&clone, grid))
}

fn detect_cycle(mut guard: Guard, grid: Vec<Vec<Cell>>) -> bool {
    let mut visited = FxHashSet::default();
    visited.insert((guard.x, guard.y, guard.direction));

    while let Some(cell) = look_ahead(&guard, &grid) {
        match cell {
            Cell::Obstruction => {
                guard.rotate();
                guard.advance();
            }
            Cell::Open => guard.advance(),
        }
        let pos = (guard.x, guard.y, guard.direction);
        if visited.contains(&pos) {
            return true;
        }
        visited.insert(pos);
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut ray, grid) = parse(input);

    let mut visited = FxHashSet::default();

    visited.insert((ray.x, ray.y));

    while let Some(cell) = look_ahead(&ray, &grid) {
        match cell {
            Cell::Obstruction => {
                ray.rotate();
                ray.advance();
            }
            Cell::Open => ray.advance(),
        }
        visited.insert((ray.x, ray.y));
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut ray, grid) = parse(input);

    let initial_ray = ray.clone();
    let mut visited = FxHashSet::default();

    visited.insert((ray.x, ray.y));

    while let Some(cell) = look_ahead(&ray, &grid) {
        match cell {
            Cell::Obstruction => {
                ray.rotate();
                ray.advance();
            }
            Cell::Open => ray.advance(),
        }
        visited.insert((ray.x, ray.y));
    }

    let positions = visited
        .iter()
        .map(|(x, y)| {
            let mut new_grid = grid.clone();
            new_grid[*y][*x] = Cell::Obstruction;

            detect_cycle(initial_ray.clone(), new_grid)
        })
        .filter(|x| *x)
        .count();

    Some(positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
