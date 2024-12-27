use std::{
    collections::{HashMap, VecDeque},
    iter::once,
    rc::Rc,
};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

type NodeList = FxHashSet<Rc<str>>;

fn parse(input: &str) -> (FxHashMap<Rc<str>, NodeList>, NodeList) {
    let mut adjacent: HashMap<Rc<str>, NodeList, _> = FxHashMap::default();
    let mut tnames = FxHashSet::default();

    input.trim_end().lines().for_each(|line| {
        let (left, right) = line.split_once('-').unwrap();
        let left: Rc<str> = left.into();
        let right: Rc<str> = right.into();
        adjacent
            .entry(left.clone())
            .or_default()
            .insert(right.clone());
        adjacent
            .entry(right.clone())
            .or_default()
            .insert(left.clone());

        if left.starts_with('t') {
            tnames.insert(left);
        }
        if right.starts_with('t') {
            tnames.insert(right);
        }
    });

    (adjacent, tnames)
}

fn bron_kerbosch(
    current: NodeList,
    mut potential: NodeList,
    mut processed: NodeList,
    adjacent: &FxHashMap<Rc<str>, NodeList>,
    result: &mut NodeList,
) {
    if potential.is_empty() && processed.is_empty() {
        if current.len() > result.len() {
            *result = current;
        }
        return;
    }
    for node in potential.clone() {
        let neighbors = &adjacent[&node];
        bron_kerbosch(
            current.iter().cloned().chain(once(node.clone())).collect(),
            potential.intersection(neighbors).cloned().collect(),
            processed.intersection(neighbors).cloned().collect(),
            adjacent,
            result,
        );
        potential.remove(&node);
        processed.insert(node);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (adjacent, tnames) = parse(input);

    let mut total = FxHashSet::default();

    let mut queue: VecDeque<_> = tnames
        .into_iter()
        .map(|n| (n.clone(), vec![n.clone()]))
        .collect();

    while let Some((name, mut path)) = queue.pop_front() {
        if path.len() == 3 {
            path.sort_unstable();
            total.insert(path);
            continue;
        }

        for next in adjacent[&name].iter() {
            let mut temp = path.clone();
            if !temp.contains(next) && adjacent[next].contains(&path[0]) {
                temp.push(next.clone());
                queue.push_back((next.clone(), temp));
            }
        }
    }

    Some(total.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let (adjacent, _) = parse(input);

    let mut result = FxHashSet::default();

    bron_kerbosch(
        Default::default(),
        adjacent.keys().cloned().collect(),
        Default::default(),
        &adjacent,
        &mut result,
    );

    Some(result.into_iter().sorted_unstable().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".into()));
    }
}
