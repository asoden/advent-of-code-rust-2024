use fxhash::FxHashMap;
use rayon::prelude::*;

advent_of_code::solution!(22);

#[inline]
fn mix(value: i64, seed: i64) -> i64 {
    value ^ seed
}

#[inline]
fn prune(seed: i64) -> i64 {
    seed & 0xFFFFFF
}

fn generate_secret(seed: i64) -> i64 {
    let intermediate = prune(mix(seed << 6, seed));

    let intermediate = prune(mix(intermediate >> 5, intermediate));

    prune(mix(intermediate << 11, intermediate))
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim_end()
        .lines()
        .flat_map(|line| line.parse())
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let secret_bids = parse(input);

    Some(
        secret_bids
            .par_iter()
            .map(|&secret_bid| {
                let mut bid = secret_bid;
                for _ in 0..2000 {
                    bid = generate_secret(bid);
                }

                bid
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i16> {
    let secret_bids = parse(input);

    secret_bids
        .par_iter()
        .map(|&secret_bid| {
            let mut bids = Vec::with_capacity(2001);
            let mut bid = secret_bid;
            bids.push((bid % 10) as i16);
            for _ in 0..2000 {
                bid = generate_secret(bid);
                bids.push((bid % 10) as i16);
            }

            let mut mapping = FxHashMap::default();

            for window in bids.windows(5) {
                let first = window[0];
                let second = window[1];
                let third = window[2];
                let fourth = window[3];
                let fifth = window[4];

                mapping
                    .entry((
                        first - second,
                        second - third,
                        third - fourth,
                        fourth - fifth,
                    ))
                    .or_insert(fifth);
            }
            mapping
        })
        .reduce(FxHashMap::default, |mut a, b| {
            b.into_iter().for_each(|(k, v)| {
                *a.entry(k).or_insert(0) += v;
            });
            a
        })
        .into_values()
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
