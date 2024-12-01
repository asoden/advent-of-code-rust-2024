use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list = vec![];
    let mut right_list = vec![];

    input.trim_end().lines().for_each(|line| {
        let mut line = line.split_whitespace();
        left_list.push(
            line.next()
                .expect("Left value of list")
                .parse::<u32>()
                .expect("Numeric value"),
        );
        right_list.push(
            line.next()
                .expect("Right value of list")
                .parse::<u32>()
                .expect("Numeric value"),
        );
    });

    left_list.sort_unstable();
    right_list.sort_unstable();

    Some(
        left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list = vec![];
    let mut right_keys: HashMap<u32, u32> = HashMap::new();

    input.trim_end().lines().for_each(|line| {
        let mut line = line.split_whitespace();

        left_list.push(
            line.next()
                .expect("Left value of list")
                .parse::<u32>()
                .expect("Numeric value"),
        );

        let right_num = line
            .next()
            .expect("Left value of list")
            .parse::<u32>()
            .expect("Numeric value");

        right_keys
            .entry(right_num)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    });

    Some(
        left_list
            .iter()
            .map(|&val| right_keys.get(&val).unwrap_or(&0) * val)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
