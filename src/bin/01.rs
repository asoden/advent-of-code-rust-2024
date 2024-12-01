use fxhash::{FxBuildHasher, FxHashMap};

advent_of_code::solution!(1);

#[inline]
fn get_abs(n: i32) -> i32 {
    let mask = n >> 31;
    (n + mask) ^ mask
}
fn parse_file_content1(file_content: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::with_capacity(1000);
    let mut right: Vec<i32> = Vec::with_capacity(1000);
    let lists = [&mut left, &mut right];
    let mut value: i32 = 0;
    let mut in_value = false;
    let mut was_in_value: bool;
    let mut insert_index: usize = 0;

    for c in file_content.bytes() {
        was_in_value = in_value;
        in_value = c.is_ascii_digit();
        if in_value {
            value = value * 10 + (c - b'0') as i32;
        } else if was_in_value {
            lists[insert_index].push(value);
            insert_index = 1 - insert_index;
            value = 0;
        }
    }

    if in_value {
        right.push(value);
    }

    (left, right)
}

fn parse_file_content2(file_content: &str) -> (Vec<i32>, FxHashMap<i32, i32>) {
    let mut left: Vec<i32> = Vec::with_capacity(1000);
    let mut right_keys: FxHashMap<i32, i32> =
        FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher::default());
    let mut left_inserter = |v| left.push(v);
    let mut right_inserter = |v| {
        right_keys.entry(v).and_modify(|val| *val += 1).or_insert(1);
    };
    let mut inserters: Vec<&mut dyn FnMut(i32)> = vec![&mut left_inserter, &mut right_inserter];
    let mut value: i32 = 0;
    let mut in_value = false;
    let mut was_in_value: bool;
    let mut insert_index: usize = 0;

    for c in file_content.bytes() {
        was_in_value = in_value;
        in_value = c.is_ascii_digit();
        if in_value {
            value = value * 10 + (c - b'0') as i32;
        } else if was_in_value {
            inserters[insert_index](value);
            insert_index = 1 - insert_index;
            value = 0;
        }
    }

    if in_value {
        inserters[insert_index](value);
    }

    (left, right_keys)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left_list, mut right_list) = parse_file_content1(input);

    left_list.sort_unstable();
    right_list.sort_unstable();

    Some(
        left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left, right)| get_abs(unsafe { left.unchecked_sub(*right) }))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let (left_list, right_keys) = parse_file_content2(input);

    Some(
        left_list
            .iter()
            .filter_map(|&val| right_keys.get(&val).map(|r| r * val))
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
