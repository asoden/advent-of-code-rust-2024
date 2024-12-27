use fxhash::FxHashMap;
use std::str;

advent_of_code::solution!(21);

const NUMPAD: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

const DIRPAD: [[u8; 3]; 2] = [[b' ', b'^', b'A'], [b'<', b'v', b'>']];

fn construct_mapping(map: &[[u8; 3]]) -> FxHashMap<u8, (i32, i32)> {
    let mut mapping = FxHashMap::default();

    for (y, row) in map.iter().enumerate() {
        for (x, key) in row.iter().enumerate() {
            mapping.insert(*key, (x as i32, y as i32));
        }
    }
    mapping
}

fn shortest_path(key1: u8, key2: u8, pad_mapping: &FxHashMap<u8, (i32, i32)>) -> Vec<u8> {
    let (col1, row1) = pad_mapping[&key1];
    let (col2, row2) = pad_mapping[&key2];

    let vertical_moves = if row2 > row1 {
        let diff = (row2 - row1) as usize;
        vec![b'v'; diff]
    } else {
        let diff = (row1 - row2) as usize;
        vec![b'^'; diff]
    };

    let horizontal_moves = if col2 > col1 {
        let diff = (col2 - col1) as usize;
        vec![b'>'; diff]
    } else {
        let diff = (col1 - col2) as usize;
        vec![b'<'; diff]
    };

    let gap = pad_mapping[&b' '];
    if col2 > col1 && (col1, row2) != gap {
        // safe to move vertically first if heading right and corner isn't the gap
        return [vertical_moves, horizontal_moves, vec![b'A']].concat();
    }

    if (col2, row1) != gap {
        // safe to movehorizontally first if corner isn't gap
        return [horizontal_moves, vertical_moves, vec![b'A']].concat();
    }

    // save to move vertically first as we are not in same colum as gap
    [vertical_moves, horizontal_moves, vec![b'A']].concat()
}

fn sequences(sequence: &[u8], pad_mapping: &FxHashMap<u8, (i32, i32)>) -> Vec<Vec<u8>> {
    let mut keys = vec![];
    let mut previous_key = b'A';
    for &key in sequence {
        keys.push(shortest_path(previous_key, key, pad_mapping));
        previous_key = key;
    }
    keys
}

fn sequence_counts(
    sequence: &[u8],
    pad_mapping: &FxHashMap<u8, (i32, i32)>,
) -> FxHashMap<Vec<u8>, usize> {
    sequences(sequence, pad_mapping)
        .into_iter()
        .fold(FxHashMap::default(), |mut acc, seq| {
            *acc.entry(seq).or_insert(0) += 1;
            acc
        })
}

fn complexity(
    codes: &[Vec<u8>],
    num_bots: u32,
    numpad_lookup: &FxHashMap<u8, (i32, i32)>,
    dirpad_lookup: &FxHashMap<u8, (i32, i32)>,
) -> usize {
    let mut frequency_tables: Vec<FxHashMap<Vec<u8>, usize>> = codes
        .iter()
        .map(|code| sequences(code, numpad_lookup).concat())
        .map(|seq| {
            let mut temp = FxHashMap::default();
            temp.insert(seq, 1);
            temp
        })
        .collect();

    for _ in 0..num_bots {
        frequency_tables = frequency_tables
            .iter()
            .map(|frequency_table| {
                let mut sub_freq_table: FxHashMap<Vec<u8>, usize> = FxHashMap::default();
                frequency_table.iter().for_each(|(seq, freq)| {
                    sequence_counts(seq, dirpad_lookup).into_iter().for_each(
                        |(sub_seq, sub_freq)| {
                            *sub_freq_table.entry(sub_seq).or_default() += sub_freq * freq;
                        },
                    );
                });
                sub_freq_table
            })
            .collect();
    }

    frequency_tables
        .iter()
        .zip(codes)
        .map(|(seqs, codes)| {
            let temp = str::from_utf8(&codes[..codes.len() - 1])
                .unwrap()
                .parse::<usize>()
                .unwrap();
            temp * seqs
                .iter()
                .map(|(seq, freq)| seq.len() * freq)
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_end()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let numpad_lookup = construct_mapping(&NUMPAD);
    let dirpad_lookup = construct_mapping(&DIRPAD);

    let codes = parse(input);

    let sequences: Vec<Vec<u8>> = codes
        .iter()
        .map(|code| sequences(code, &numpad_lookup).concat())
        .map(|robot1_seq| sequences(&robot1_seq, &dirpad_lookup).concat())
        .map(|robot2_seq| sequences(&robot2_seq, &dirpad_lookup).concat())
        .collect();

    Some(
        codes
            .iter()
            .zip(sequences)
            .map(|(codes, seq)| {
                let temp = str::from_utf8(&codes[..codes.len() - 1])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                temp * seq.len()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let numpad_lookup = construct_mapping(&NUMPAD);
    let dirpad_lookup = construct_mapping(&DIRPAD);

    let codes = parse(input);

    Some(complexity(&codes, 25, &numpad_lookup, &dirpad_lookup))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
