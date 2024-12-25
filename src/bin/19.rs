use std::collections::BinaryHeap;

use fxhash::FxHashMap;

advent_of_code::solution!(19);

#[derive(Default)]
struct Trie(bool, FxHashMap<u8, Trie>);

fn add_to_tree(pattern: &[u8], Trie(is_leaf, child): &mut Trie) {
    match pattern {
        [] => *is_leaf = true,
        [front, rest @ ..] => {
            if let Some(sub_tree) = child.get_mut(front) {
                add_to_tree(rest, sub_tree);
            } else {
                let mut new_trie = Trie::default();
                add_to_tree(rest, &mut new_trie);
                child.insert(*front, new_trie);
            }
        }
    }
}

fn build_pattern_tree(patterns: &[Vec<u8>]) -> Trie {
    let mut tree = Trie::default();
    patterns
        .iter()
        .for_each(|pattern| add_to_tree(pattern, &mut tree));
    tree
}

fn trie_search(pattern: &[u8], Trie(is_leaf, child): &Trie, depth: usize) -> Vec<usize> {
    let mut sub_pattern_lengths = match pattern {
        [] => vec![],
        [first, rest @ ..] => child
            .get(first)
            .map(|child| trie_search(rest, child, depth + 1))
            .unwrap_or_default(),
    };
    if *is_leaf {
        sub_pattern_lengths.push(depth);
    }
    sub_pattern_lengths
}

fn is_possible(pattern: &[u8], available: &Trie) -> bool {
    let mut stack = BinaryHeap::from([0usize]);
    let mut found = false;
    while let Some(read_length) = stack.pop() {
        if read_length == pattern.len() {
            found = true;
            break;
        }
        stack.extend(trie_search(&pattern[read_length..], available, read_length));
    }
    found
}

fn possible_counts(pattern: &[u8], available: &Trie) -> usize {
    let mut counts = [vec![1], vec![0; pattern.len()]].concat();
    for idx in 0..counts.len() {
        for possible_end in trie_search(&pattern[idx..], available, idx) {
            counts[possible_end] += counts[idx];
        }
    }
    *counts.last().unwrap()
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let (available_towels, requested_patterns) = input.trim_end().split_once("\n\n").unwrap();

    let available = available_towels
        .split(", ")
        .map(|char| char.bytes().collect())
        .collect();
    let requested = requested_patterns
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    (available, requested)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (available, requested) = parse(input);

    let tree = build_pattern_tree(&available);

    let counts = requested
        .iter()
        .map(|request| is_possible(request, &tree))
        .filter(|a| *a)
        .count();
    Some(counts)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (available, requested) = parse(input);

    let tree = build_pattern_tree(&available);
    Some(
        requested
            .iter()
            .map(|wanted| possible_counts(wanted, &tree))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
