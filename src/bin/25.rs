use itertools::Itertools;

advent_of_code::solution!(25);

fn parse(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_punctuation)
                .map(|c| c as u64)
                // character # has a 1 as least sig bit, . has a 0
                .fold(0, |acc, c| (c & 1) | (acc << 1))
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let schematics = parse(input);

    Some(
        schematics
            .iter()
            .copied()
            .tuple_combinations()
            .filter(|(a, b)| a & b == 0)
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
