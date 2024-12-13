use itertools::Itertools;

advent_of_code::solution!(13);

const RIGGED_BS: i64 = 10000000000000;

fn solve(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let a = (y2 * z1 - y1 * z2) / (y2 * x1 - x2 * y1);
    let b = (z2 - x2 * a) / y2;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return 0;
    }
    a * 3 + b
}

fn parse(input: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let machines = input.split("\n\n");
    machines
        .map(|machine| {
            machine
                .split(|c: char| !c.is_ascii_digit())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse(input);

    Some(
        machines
            .into_iter()
            .map(|(x1, x2, y1, y2, z1, z2)| solve(x1, x2, y1, y2, z1, z2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse(input);

    Some(
        machines
            .into_iter()
            .map(|(x1, x2, y1, y2, z1, z2)| solve(x1, x2, y1, y2, z1 + RIGGED_BS, z2 + RIGGED_BS))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
