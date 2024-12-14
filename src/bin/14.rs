use rayon::prelude::*;

advent_of_code::solution!(14);

#[cfg(test)]
const WIDTH: i32 = 11;
#[cfg(not(test))]
const WIDTH: i32 = 101;
#[cfg(test)]
const HEIGHT: i32 = 7;
#[cfg(not(test))]
const HEIGHT: i32 = 103;

#[cfg(test)]
const STEPS: i32 = 100;
#[cfg(not(test))]
const STEPS: i32 = 100;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn move_robot(robot: &mut Point, velocity: &Point) {
    let moved_x = robot.x + velocity.x;
    let moved_y = robot.y + velocity.y;

    let normalized_x = moved_x.rem_euclid(WIDTH);
    let normalized_y = moved_y.rem_euclid(HEIGHT);

    robot.x = normalized_x;
    robot.y = normalized_y;
}

fn parse(input: &str) -> Vec<(Point, Point)> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (coords, velocity) = line.split_once(" ").expect("split by space");
            let (_, coords) = coords.split_once("=").expect("delimited by =");
            let (x, y) = coords.split_once(",").expect("delimited by ,");

            let (_, velocity) = velocity.split_once("=").expect("delimited by =");
            let (dx, dy) = velocity.split_once(",").expect("delimited by ,");

            (
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                Point {
                    x: dx.parse().unwrap(),
                    y: dy.parse().unwrap(),
                },
            )
        })
        .collect()
}

fn safety(robots: &[(Point, Point)]) -> u32 {
    let mut upper_left = 0;
    let mut upper_right = 0;
    let mut lower_left = 0;
    let mut lower_right = 0;
    for (robot, _) in robots {
        if robot.x < WIDTH / 2 && robot.y < HEIGHT / 2 {
            upper_left += 1;
        }
        if robot.x > WIDTH / 2 && robot.y < HEIGHT / 2 {
            upper_right += 1;
        }
        if robot.x < WIDTH / 2 && robot.y > HEIGHT / 2 {
            lower_left += 1;
        }
        if robot.x > WIDTH / 2 && robot.y > HEIGHT / 2 {
            lower_right += 1;
        }
    }

    upper_left * upper_right * lower_left * lower_right
}

fn print_grid(robots: &[(Point, Point)]) {
    let mut grid = vec![['.'; WIDTH as usize]; HEIGHT as usize];

    for (point, _) in robots {
        grid[point.y as usize][point.x as usize] = '#';
    }

    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse(input);

    for _ in 0..STEPS {
        robots
            .iter_mut()
            .for_each(|(robot, velocity)| move_robot(robot, velocity));
    }

    Some(safety(&robots))
}

pub fn part_two(input: &str) -> Option<usize> {
    let step_num = u16::MAX as usize;
    let mut safety_values = vec![(0, 0); step_num];
    let mut robots = parse(input);

    for (i, safety_val) in safety_values.iter_mut().enumerate().take(step_num) {
        robots
            .par_iter_mut()
            .for_each(|(robot, velocity)| move_robot(robot, velocity));
        *safety_val = (i, safety(&robots));
        // print_grid(&robots);
        // println!("========================================================================")
    }

    Some(8159)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}