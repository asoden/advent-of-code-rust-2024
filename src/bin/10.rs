use std::collections::VecDeque;

use fxhash::FxHashSet;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<i8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<i8>> = input
            .lines()
            .map(|line| line.bytes().map(|char| (char - b'0') as i8).collect())
            .collect();
        let width: i32 = grid[0].len() as i32;
        let height: i32 = grid.len() as i32;

        Self {
            height,
            width,
            grid,
        }
    }

    fn find_starts(&self) -> Vec<(i32, i32)> {
        let mut starts = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, elevation) in row.iter().enumerate() {
                if *elevation == 0 {
                    starts.push((x as i32, y as i32));
                }
            }
        }
        starts
    }

    fn get(&self, x: i32, y: i32) -> i8 {
        self.grid[y as usize][x as usize]
    }

    fn get_adjacent(&self, x: i32, y: i32) -> impl Iterator<Item = (i8, i32, i32)> + '_ {
        let width = self.width;
        let height = self.height;
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter(move |(delta_x, delta_y)| {
                x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
            })
            .map(move |(delta_x, delta_y)| {
                let new_x = x + delta_x;
                let new_y = y + delta_y;
                (self.get(new_x, new_y), new_x, new_y)
            })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    let starts = grid.find_starts();

    Some(
        starts
            .iter()
            .map(|start| {
                let mut num_paths = 0;
                let mut queue = VecDeque::new();
                let mut seen = FxHashSet::default();
                queue.push_back(*start);

                while let Some(current_location) = queue.pop_front() {
                    let (x, y) = current_location;

                    if seen.contains(&(x, y)) {
                        continue;
                    }
                    let current_value = grid.get(x, y);
                    if current_value == 9 {
                        num_paths += 1;
                        seen.insert((x, y));
                    } else {
                        for (adjacent_value, next_x, next_y) in grid.get_adjacent(x, y) {
                            if adjacent_value - current_value == 1 {
                                queue.push_back((next_x, next_y));
                            }
                        }
                    }
                }

                num_paths
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    let starts = grid.find_starts();

    Some(
        starts
            .iter()
            .map(|start| {
                let mut num_paths = 0;
                let mut queue = VecDeque::new();
                queue.push_back(*start);

                while let Some(current_location) = queue.pop_front() {
                    let (x, y) = current_location;

                    let current_value = grid.get(x, y);
                    if current_value == 9 {
                        num_paths += 1;
                    } else {
                        for (adjacent_value, next_x, next_y) in grid.get_adjacent(x, y) {
                            if adjacent_value - current_value == 1 {
                                queue.push_back((next_x, next_y));
                            }
                        }
                    }
                }

                num_paths
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
