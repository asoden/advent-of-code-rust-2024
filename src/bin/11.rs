use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

advent_of_code::solution!(11);

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn engraved_stones(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), number)(input)
}

fn apply_rules(stone: u64) -> Vec<u64> {
    // 0 becomes 1
    if stone == 0 {
        return vec![1];
    }
    let num_digits = stone.checked_ilog10().unwrap_or(0) + 1;
    // even digits split number
    if num_digits % 2 == 0 {
        let magnitude = 10u64.pow(num_digits / 2);
        let left_half = stone / magnitude;
        let right_half = stone - (left_half * magnitude);
        return vec![left_half, right_half];
    }
    vec![stone * 2024]
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, mut rocks) = engraved_stones(input).unwrap();

    for _ in 0..25 {
        rocks = rocks.into_iter().flat_map(apply_rules).collect();
    }

    Some(rocks.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, rocks) = engraved_stones(input).unwrap();

    let mut rock_collection = FxHashMap::default();
    rocks.iter().for_each(|rock| {
        rock_collection.insert(*rock, 1u64);
    });

    for _ in 0..75 {
        let mut temp_collection = FxHashMap::default();
        for (&stone, &count) in rock_collection.iter() {
            let rules_collection = apply_rules(stone);
            for stone in rules_collection {
                temp_collection
                    .entry(stone)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            }
        }
        rock_collection = temp_collection;
    }

    Some(rock_collection.values().map(|x| *x as usize).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
