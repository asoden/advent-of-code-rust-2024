advent_of_code::solution!(5);

use fxhash::{FxHashMap, FxHashSet};

fn parse(input: &str) -> (FxHashMap<u32, FxHashSet<u32>>, Vec<Vec<u32>>) {
    let (rules, pages) = input
        .trim_end()
        .split_once("\n\n")
        .expect("Rules and pages split by empty new line");
    let mut rules_map = FxHashMap::default();

    rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("|").expect("Rules delimited by |");
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .for_each(|(left, right)| {
            rules_map
                .entry(left)
                .or_insert(FxHashSet::default())
                .insert(right);
        });

    let pages = pages
        .lines()
        .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
        .collect();
    (rules_map, pages)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse(input);

    Some(
        pages
            .iter()
            .filter(|page| page.is_sorted_by(|left, right| rules[left].contains(right)))
            .map(|page| page[page.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut pages) = parse(input);

    Some(
        pages
            .iter_mut()
            .filter(|page| !page.is_sorted_by(|left, right| rules[left].contains(right)))
            .map(|page| {
                page.sort_by(|left, right| rules[left].contains(right).cmp(&true));
                page[page.len() / 2]
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
