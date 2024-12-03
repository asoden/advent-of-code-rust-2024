use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res, value},
    multi::many_till,
    IResult,
};

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
enum Instruction {
    Do,
    Dont,
    Multiply(u32, u32),
}

impl Instruction {
    fn product(&self) -> u32 {
        match self {
            Instruction::Multiply(lhs, rhs) => lhs * rhs,
            _ => 0,
        }
    }
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn find_mul(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mul(")(i)?;
    let (i, lhs) = number(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, rhs) = number(i)?;
    let (i, _) = tag(")")(i)?;

    Ok((i, Instruction::Multiply(lhs, rhs)))
}

fn find_do(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Do, tag("do()"))(input)
}

fn find_dont(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Dont, tag("don't()"))(input)
}

fn find_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((find_mul, find_do, find_dont))(input)
}

fn find_next_instruction(input: &str) -> IResult<&str, Instruction> {
    let (i, (_, mul)) = many_till(map(anychar, drop), find_instruction)(input)?;
    Ok((i, mul))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parsing_str = input;
    let mut total = 0;

    while !parsing_str.is_empty() {
        let Ok((next_pass, instruction)) = find_next_instruction(parsing_str) else {
            break;
        };

        total += instruction.product();
        parsing_str = next_pass;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parsing_str = input;
    let mut total = 0;
    let mut enabled = true;

    while !parsing_str.is_empty() {
        let Ok((next_pass, instruction)) = find_next_instruction(parsing_str) else {
            break;
        };

        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Multiply(_, _) => {
                if enabled {
                    total += instruction.product();
                }
            }
        }
        parsing_str = next_pass;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
