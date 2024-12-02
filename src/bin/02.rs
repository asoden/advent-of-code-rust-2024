advent_of_code::solution!(2);

use itertools::Itertools;

fn parse_report(report: &str) -> Vec<i32> {
    report
        .split_ascii_whitespace()
        .filter_map(|val| val.parse().ok())
        .collect()
}

fn parse_all_reports(reports: &str) -> Vec<Vec<i32>> {
    reports.trim_end().lines().map(parse_report).collect()
}

fn safe_check(report: &[i32]) -> bool {
    let mut safe = false;
    safe |= report
        .iter()
        .tuple_windows()
        .all(|(left, right)| left < right);
    safe |= report
        .iter()
        .tuple_windows()
        .all(|(left, right)| left > right);
    safe && report
        .iter()
        .tuple_windows()
        .all(|(left, right)| (1..=3).contains(&(left - right).abs()))
}

fn any_elf_safety(report: &[i32]) -> bool {
    (0..report.len()).any(|i| {
        let mut temp = report.to_vec();
        temp.remove(i);
        safe_check(&temp)
    })
}

pub fn part_one(input: &str) -> Option<i32> {
    let reports = parse_all_reports(input);

    let x: i32 = reports
        .iter()
        .filter(|report| safe_check(report))
        .count()
        .try_into()
        .unwrap();
    Some(x)
}

pub fn part_two(input: &str) -> Option<i32> {
    let reports = parse_all_reports(input);

    let x: i32 = reports
        .iter()
        .filter(|report| any_elf_safety(report))
        .count()
        .try_into()
        .unwrap();
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
