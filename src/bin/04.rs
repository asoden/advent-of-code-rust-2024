use rayon::prelude::*;
advent_of_code::solution!(4);

fn build_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_end()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn count_horizontal(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for row in grid.iter() {
        for index in 0..row.len() - 3 {
            if row[index] == b'X'
                && row[index + 1] == b'M'
                && row[index + 2] == b'A'
                && row[index + 3] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_latnoziroh(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for row in grid.iter() {
        for index in 3..row.len() {
            if row[index] == b'X'
                && row[index - 1] == b'M'
                && row[index - 2] == b'A'
                && row[index - 3] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_vertical(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() - 3 {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'X'
                && grid[y + 1][x] == b'M'
                && grid[y + 2][x] == b'A'
                && grid[y + 3][x] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_lacitrev(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 3..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'X'
                && grid[y - 1][x] == b'M'
                && grid[y - 2][x] == b'A'
                && grid[y - 3][x] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_right_diagonal(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() - 3 {
        for x in 0..grid[y].len() - 3 {
            if grid[y][x] == b'X'
                && grid[y + 1][x + 1] == b'M'
                && grid[y + 2][x + 2] == b'A'
                && grid[y + 3][x + 3] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_thgir_lanogaid(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 3..grid.len() {
        for x in 3..grid[y].len() {
            if grid[y][x] == b'X'
                && grid[y - 1][x - 1] == b'M'
                && grid[y - 2][x - 2] == b'A'
                && grid[y - 3][x - 3] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_left_diagonal(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 3..grid.len() {
        for x in 0..grid[y].len() - 3 {
            if grid[y][x] == b'X'
                && grid[y - 1][x + 1] == b'M'
                && grid[y - 2][x + 2] == b'A'
                && grid[y - 3][x + 3] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_tfel_lanogaid(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() - 3 {
        for x in 3..grid[y].len() {
            if grid[y][x] == b'X'
                && grid[y + 1][x - 1] == b'M'
                && grid[y + 2][x - 2] == b'A'
                && grid[y + 3][x - 3] == b'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn count_mas(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] == b'A'
                && grid[y + 1][x - 1].abs_diff(grid[y - 1][x + 1]) == 6
                && grid[y + 1][x + 1].abs_diff(grid[y - 1][x - 1]) == 6
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = build_grid(input);

    let val = [
        count_horizontal,
        count_latnoziroh,
        count_vertical,
        count_lacitrev,
        count_right_diagonal,
        count_thgir_lanogaid,
        count_left_diagonal,
        count_tfel_lanogaid,
    ];

    let x: u32 = val.par_iter().map(|func| func(&grid)).sum();
    Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = build_grid(input);
    Some(count_mas(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
