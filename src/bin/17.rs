#![allow(clippy::upper_case_acronyms)]
use fxhash::FxHashSet;
use itertools::Itertools;

advent_of_code::solution!(17);

enum OperandType {
    Literal,
    Combo,
}

impl From<Opcode> for OperandType {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::ADV => Self::Combo,
            Opcode::BXL => Self::Literal,
            Opcode::BST => Self::Combo,
            Opcode::JNZ => Self::Combo,
            Opcode::BXC => Self::Literal,
            Opcode::OUT => Self::Combo,
            Opcode::BDV => Self::Combo,
            Opcode::CDV => Self::Combo,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    counter: usize,
    program: Vec<usize>,
    stopped: bool,
    output: Vec<usize>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let (registers, program) = input.trim_end().split_once("\n\n").unwrap();
        let mut register_lines = registers.split("\n");
        let (_, a) = register_lines.next().unwrap().split_once(": ").unwrap();
        let (_, b) = register_lines.next().unwrap().split_once(": ").unwrap();
        let (_, c) = register_lines.next().unwrap().split_once(": ").unwrap();

        let (_, program) = program.split_once(": ").unwrap();
        let program = program.split(",").flat_map(str::parse).collect();

        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
            counter: 0,
            program,
            stopped: false,
            output: Vec::new(),
        }
    }

    fn step(&mut self) {
        if let Some(opcode) = self.program.get(self.counter) {
            let opcode = Opcode::from(*opcode);
            let operand = match OperandType::from(opcode) {
                OperandType::Literal => self.program[self.counter + 1],
                OperandType::Combo => match self.program[self.counter + 1] {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => unreachable!(),
                },
            };

            self.execute_opcode(opcode, operand);
            self.counter = self.counter.wrapping_add(2);
        } else {
            self.stopped = true;
        }
    }

    fn execute_opcode(&mut self, opcode: Opcode, operand: usize) {
        match opcode {
            Opcode::ADV => self.a /= 2usize.pow(operand as u32),
            Opcode::BXL => self.b ^= operand,
            Opcode::BST => self.b = operand % 8,
            Opcode::JNZ => {
                if self.a != 0 {
                    self.counter = operand.wrapping_sub(2);
                }
            }
            Opcode::BXC => self.b ^= self.c,
            Opcode::OUT => self.output.push(operand % 8),
            Opcode::BDV => self.b = self.a / 2usize.pow(operand as u32),
            Opcode::CDV => self.c = self.a / 2usize.pow(operand as u32),
        }
    }

    fn find_quine(&mut self) -> usize {
        let mut quines = FxHashSet::default();
        quines.insert(0);

        for num in self.program.iter().rev() {
            let mut new_quines = FxHashSet::default();
            for current in quines {
                for i in 0..8 {
                    let new = (current << 3) + i;
                    if Self::output(new) == *num {
                        new_quines.insert(new);
                    }
                }
            }
            quines = new_quines;
        }

        *quines.iter().min().unwrap_or(&0)
    }

    fn output(a: usize) -> usize {
        let partial = (a % 8) ^ 1;
        ((partial ^ (a >> partial)) ^ 4) % 8
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::new(input);
    while !computer.stopped {
        computer.step();
    }
    Some(computer.output.iter().join(","))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut computer = Computer::new(input);

    Some(computer.find_quine())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
