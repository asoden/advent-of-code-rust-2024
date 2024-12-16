use std::{
    fmt::Display,
    mem,
    ops::{Add, AddAssign},
};

advent_of_code::solution!(15);

const DUMMY: Point = Point { x: 0, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const LEFT: Point = Point { x: -1, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq)]
enum Entity {
    Wall,
    Empty,
    Bot,
    Box,
    BoxLeft,
    BoxRight,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Wall => write!(f, "#"),
            Entity::Empty => write!(f, "."),
            Entity::Bot => write!(f, "@"),
            Entity::Box => write!(f, "O"),
            Entity::BoxLeft => write!(f, "["),
            Entity::BoxRight => write!(f, "]"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
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

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<Entity>>,
    instructions: Vec<Direction>,
    bot: Point,
    is_doubled: bool,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let (grid, instructions) = input.trim_end().split_once("\n\n").unwrap();
        let mut bot = Point::default();
        let grid = grid
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, cell)| match cell {
                        '#' => Entity::Wall,
                        '.' => Entity::Empty,
                        'O' => Entity::Box,
                        '@' => {
                            bot = Point {
                                x: x as i32,
                                y: y as i32,
                            };
                            Entity::Bot
                        }
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();

        let instructions = instructions
            .chars()
            .filter(|char| *char != '\n')
            .map(|char| match char {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => panic!("Invalid character"),
            })
            .collect();

        Self {
            grid,
            instructions,
            bot,
            is_doubled: false,
        }
    }

    fn big_new(input: &str) -> Self {
        let (grid_str, instructions) = input.trim_end().split_once("\n\n").unwrap();
        let mut bot = Point::default();
        let mut grid = vec![];
        grid_str.lines().enumerate().for_each(|(y, line)| {
            let mut row = vec![];
            line.chars().enumerate().for_each(|(x, cell)| match cell {
                '#' => {
                    row.push(Entity::Wall);
                    row.push(Entity::Wall);
                }
                '.' => {
                    row.push(Entity::Empty);
                    row.push(Entity::Empty);
                }
                'O' => {
                    row.push(Entity::BoxLeft);
                    row.push(Entity::BoxRight);
                }
                '@' => {
                    bot = Point {
                        x: (x * 2) as i32,
                        y: y as i32,
                    };
                    row.push(Entity::Bot);
                    row.push(Entity::Empty)
                }
                _ => panic!("Invalid character"),
            });
            grid.push(row);
        });

        let instructions = instructions
            .chars()
            .filter(|char| *char != '\n')
            .map(|char| match char {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => panic!("Invalid character"),
            })
            .collect();

        Self {
            grid,
            instructions,
            bot,
            is_doubled: true,
        }
    }

    fn get_tile(&self, tile: &Point) -> Entity {
        self.grid[tile.y as usize][tile.x as usize]
    }

    fn move_small_boxes(&mut self, direction: Direction) {
        let vector = direction.vector();
        let mut scan_point = self.bot + vector;
        let mut cell = self.get_tile(&scan_point);
        while cell == Entity::Box {
            scan_point += vector;
            cell = self.get_tile(&scan_point);
        }

        match cell {
            // don't do anything if we hit a wall behind all the boxes
            Entity::Wall => (),
            // simply just put a box at the end and move the bot to the first box's space replacing it
            Entity::Empty => {
                self.grid[scan_point.y as usize][scan_point.x as usize] = Entity::Box;
                self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Empty;
                self.bot += vector;
                self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Bot;
            }
            Entity::Bot => panic!("Only one bot on map which was our start point"),
            Entity::Box => panic!("Boxes have all been iterated over"),
            Entity::BoxLeft => panic!("Big boxes not supported in small move"),
            Entity::BoxRight => panic!("Big boxes not supported in small move"),
        }
    }

    fn move_large_boxes(&mut self, direction: Direction) {
        let vector = direction.vector();

        let cell = self.bot + vector;
        if self.grid[cell.y as usize][cell.x as usize] == Entity::Empty {
            self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Empty;
            self.bot += vector;
            self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Bot;
            return;
        }

        match direction {
            Direction::Left | Direction::Right => {
                let mut scan_point = self.bot + vector;
                let mut size = 1;
                let mut cell = self.get_tile(&scan_point);
                while cell != Entity::Empty && cell != Entity::Wall {
                    scan_point += vector;
                    cell = self.get_tile(&scan_point);
                    size += 1;
                }

                if cell == Entity::Empty {
                    let mut previous = Entity::Empty;
                    let mut position = self.bot + vector;

                    for _ in 0..size {
                        mem::swap(
                            &mut previous,
                            &mut self.grid[position.y as usize][position.x as usize],
                        );
                        position += vector;
                    }
                    self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Empty;
                    self.bot += vector;
                    self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Bot;
                }
            }
            Direction::Up | Direction::Down => {
                // add dummy so we don't go out of bounds checking for repeats
                let mut boxes = vec![DUMMY, self.bot];
                let mut index = 1;

                while index < boxes.len() {
                    let cell = boxes[index] + vector;
                    index += 1;

                    let (left_side, right_side) = match self.grid[cell.y as usize][cell.x as usize]
                    {
                        // don't move, entire stack is stuck
                        Entity::Wall => return,
                        // keep looping nothing stopping a push here
                        Entity::Empty => continue,
                        // treat bot as empty
                        Entity::Bot => continue,
                        Entity::Box => panic!("No small boxes in a large box type"),
                        Entity::BoxLeft => (cell, cell + RIGHT),
                        Entity::BoxRight => (cell + LEFT, cell),
                    };

                    // check we didn't add this pair before
                    if left_side != boxes[boxes.len() - 2] {
                        boxes.push(left_side);
                        boxes.push(right_side);
                    }
                }

                for &point in boxes[2..].iter().rev() {
                    let new_spot = point + vector;

                    self.grid[new_spot.y as usize][new_spot.x as usize] =
                        self.grid[point.y as usize][point.x as usize];
                    self.grid[point.y as usize][point.x as usize] = Entity::Empty;
                }

                self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Empty;
                self.bot += vector;
                self.grid[self.bot.y as usize][self.bot.x as usize] = Entity::Bot;
            }
        }
    }

    fn calculate_gps_sum(&self) -> u64 {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell == Entity::Box || cell == Entity::BoxLeft)
                    .map(|(x, _)| (100 * y + x) as u64)
                    .sum::<u64>()
            })
            .sum()
    }

    fn process_instructions(&mut self) {
        let move_func = if !self.is_doubled {
            Self::move_small_boxes
        } else {
            Self::move_large_boxes
        };
        // we don't need ownership of our whole self, just self.instructions
        // NOTE: self.instructions will be the default value after this
        let instructions = mem::take(&mut self.instructions);
        for instruction in instructions {
            move_func(self, instruction);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut warehouse = Warehouse::new(input);

    warehouse.process_instructions();
    Some(warehouse.calculate_gps_sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut warehouse: Warehouse = Warehouse::big_new(input);

    warehouse.process_instructions();
    Some(warehouse.calculate_gps_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
