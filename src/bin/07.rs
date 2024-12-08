advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (result, nums) = line.split_once(": ").unwrap();

            (
                result.parse().unwrap(),
                nums.split_ascii_whitespace().flat_map(str::parse).collect(),
            )
        })
        .collect()
}

fn check(current: u64, values: &[u64], should_concat: bool) -> bool {
    if values.len() == 1 {
        return current == values[0];
    }

    if let Some((value, values)) = values.split_last() {
        if should_concat {
            let num_digits = 10u64.pow(value.checked_ilog10().unwrap_or(0) + 1);
            if (current - value) % num_digits == 0
                && check(current / num_digits, values, should_concat)
            {
                return true;
            }
        }

        if current % value == 0 && check(current / value, values, should_concat) {
            return true;
        }
        if current >= *value && check(current - value, values, should_concat) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let calibrations = parse(input);

    Some(
        calibrations
            .iter()
            .map(|(result, numbers)| {
                if check(*result, numbers, false) {
                    return result;
                }
                &0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let calibrations = parse(input);

    Some(
        calibrations
            .iter()
            .map(|(result, numbers)| {
                if check(*result, numbers, true) {
                    return result;
                }
                &0
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
