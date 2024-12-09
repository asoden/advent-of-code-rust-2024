advent_of_code::solution!(9);

#[derive(Debug, PartialEq, Clone, Copy)]
enum FileBlock {
    File(u32),
    Empty,
}

fn parse(input: &str) -> Vec<FileBlock> {
    let mut memory_bank = Vec::new();
    let mut on_file = true;
    let mut index = 0;
    for block_size in input.trim_end().chars() {
        let block_size = block_size as u32 - '0' as u32;
        for _ in 0..block_size {
            match on_file {
                true => memory_bank.push(FileBlock::File(index as u32)),
                false => memory_bank.push(FileBlock::Empty),
            }
        }
        if !on_file {
            index += 1;
        }
        on_file = !on_file;
    }

    memory_bank
}

fn checksum(memory: &[FileBlock]) -> u64 {
    memory
        .iter()
        .enumerate()
        .map(|(index, file)| match file {
            FileBlock::File(id) => index as u64 * *id as u64,
            FileBlock::Empty => 0,
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut memory_bank = parse(input);

    let mut front = 0;
    let mut back = memory_bank.len() - 1;

    loop {
        //get first empty space from left
        while memory_bank[front] != FileBlock::Empty {
            front += 1;
        }

        //get last none empty block from right
        while memory_bank[back] == FileBlock::Empty {
            back -= 1;
        }

        if front >= back {
            break;
        }

        memory_bank[front] = memory_bank[back];
        memory_bank[back] = FileBlock::Empty;
    }

    Some(checksum(&memory_bank))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memory_bank = parse(input);

    let mut front;
    let mut back = memory_bank.len() - 1;

    loop {
        front = 0;
        //get last none empty block from right
        while memory_bank[back] == FileBlock::Empty {
            back -= 1;
        }

        if back == 0 {
            break;
        }

        let mut file_size = 1;
        let file = memory_bank[back];
        loop {
            if back - file_size == 0 {
                break;
            }
            if memory_bank[back - file_size] != file {
                break;
            }
            file_size += 1;
        }

        let mut empty_size = 0;
        loop {
            //get first empty space from left
            while memory_bank[front] != FileBlock::Empty {
                front += 1;
            }
            if front + empty_size > back {
                break;
            }

            // find length of empty space
            loop {
                if memory_bank[front + empty_size] != FileBlock::Empty {
                    break;
                }
                empty_size += 1;
            }
            if empty_size >= file_size {
                break;
            }
            front += empty_size;
            empty_size = 0;
        }

        if file_size > empty_size {
            back -= file_size;
            continue;
        }

        for i in 0..file_size {
            memory_bank[front + i] = memory_bank[back - i];
            memory_bank[back - i] = FileBlock::Empty;
        }
    }

    Some(checksum(&memory_bank))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
