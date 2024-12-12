use std::{
    collections::{HashSet, VecDeque},
    hash::BuildHasherDefault,
};

use fxhash::{FxHashSet, FxHasher};

advent_of_code::solution!(12);

#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
        let width: i32 = grid[0].len() as i32;
        let height: i32 = grid.len() as i32;

        Self {
            height,
            width,
            grid,
        }
    }

    fn count_edges(&self, x: i32, y: i32) -> u32 {
        let mut count = 0;
        if x == 0 || x == self.width - 1 {
            count += 1;
        }
        if y == 0 || y == self.height - 1 {
            count += 1;
        }

        let current_value = self.get(x, y);

        for (value, _adjacent_x, _adjacent_y) in self.get_adjacent(x, y) {
            if current_value != value {
                count += 1;
            }
        }

        count
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        self.grid[y as usize][x as usize]
    }

    fn get_safe(&self, x: i32, y: i32) -> Option<u8> {
        self.grid.get(y as usize)?.get(x as usize).copied()
    }
    fn get_adjacent(&self, x: i32, y: i32) -> impl Iterator<Item = (u8, i32, i32)> + '_ {
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

fn trace_plot(
    crop: u8,
    start: (i32, i32),
    grid: &Grid,
    visited: &mut HashSet<(i32, i32), BuildHasherDefault<FxHasher>>,
) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(point) = queue.pop_front() {
        if !visited.insert(point) {
            continue;
        }

        let (x, y) = point;

        area += 1;

        perimeter += grid.count_edges(x, y);

        for (value, bordering_x, bordering_y) in grid.get_adjacent(x, y) {
            if value == crop {
                queue.push_back((bordering_x, bordering_y));
            }
        }
    }

    (area, perimeter)
}

fn trace_plot2(
    crop: u8,
    start: (i32, i32),
    grid: &Grid,
    visited: &mut HashSet<(i32, i32), BuildHasherDefault<FxHasher>>,
) -> (u32, u32) {
    let mut area = 0;
    let mut sides = 0;
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(point) = queue.pop_front() {
        if !visited.insert(point) {
            continue;
        }

        let (x, y) = point;

        area += 1;

        let some_crop = Some(crop);

        // Outer corners
        if grid.get_safe(x - 1, y) != some_crop && grid.get_safe(x, y - 1) != some_crop {
            sides += 1;
        }
        if grid.get_safe(x - 1, y) != some_crop && grid.get_safe(x, y + 1) != some_crop {
            sides += 1;
        }
        if grid.get_safe(x + 1, y) != some_crop && grid.get_safe(x, y - 1) != some_crop {
            sides += 1;
        }
        if grid.get_safe(x + 1, y) != some_crop && grid.get_safe(x, y + 1) != some_crop {
            sides += 1;
        }
        // Inner corners
        if grid.get_safe(x - 1, y) == some_crop
            && grid.get_safe(x, y - 1) == some_crop
            && grid.get_safe(x - 1, y - 1) != some_crop
        {
            sides += 1;
        }
        if grid.get_safe(x + 1, y) == some_crop
            && grid.get_safe(x, y - 1) == some_crop
            && grid.get_safe(x + 1, y - 1) != some_crop
        {
            sides += 1;
        }
        if grid.get_safe(x - 1, y) == some_crop
            && grid.get_safe(x, y + 1) == some_crop
            && grid.get_safe(x - 1, y + 1) != some_crop
        {
            sides += 1;
        }
        if grid.get_safe(x + 1, y) == some_crop
            && grid.get_safe(x, y + 1) == some_crop
            && grid.get_safe(x + 1, y + 1) != some_crop
        {
            sides += 1;
        }

        for (value, bordering_x, bordering_y) in grid.get_adjacent(x, y) {
            if value == crop {
                queue.push_back((bordering_x, bordering_y));
            }
        }
    }

    (area, sides)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut visited = FxHashSet::default();
    let mut areas: Vec<u32> = vec![];
    let mut perimeters: Vec<u32> = vec![];

    for (y, row) in grid.grid.iter().enumerate() {
        for (x, plot) in row.iter().enumerate() {
            if visited.contains(&(x as i32, y as i32)) {
                continue;
            }

            let (area, perimeter) = trace_plot(*plot, (x as i32, y as i32), &grid, &mut visited);
            areas.push(area);
            perimeters.push(perimeter);
        }
    }
    Some(
        areas
            .iter()
            .zip(perimeters)
            .map(|(area, perimeter)| area * perimeter)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut visited = FxHashSet::default();
    let mut areas: Vec<u32> = vec![];
    let mut perimeters: Vec<u32> = vec![];

    for (y, row) in grid.grid.iter().enumerate() {
        for (x, plot) in row.iter().enumerate() {
            if visited.contains(&(x as i32, y as i32)) {
                continue;
            }

            let (area, perimeter) = trace_plot2(*plot, (x as i32, y as i32), &grid, &mut visited);
            areas.push(area);
            perimeters.push(perimeter);
        }
    }
    Some(
        areas
            .iter()
            .zip(perimeters)
            .map(|(area, perimeter)| area * perimeter)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
