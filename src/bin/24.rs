#![allow(clippy::upper_case_acronyms)]
use std::{
    collections::HashMap,
    mem,
    ops::{BitAnd, BitOr, BitXor},
    rc::Rc,
};

use fxhash::{FxHashMap, FxHashSet};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    On,
    Off,
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        match value {
            "0" => Self::Off,
            "1" => Self::On,
            _ => unreachable!(),
        }
    }
}

impl BitAnd for State {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (State::On, State::On) => State::On,
            (State::On, State::Off) => State::Off,
            (State::Off, State::On) => State::Off,
            (State::Off, State::Off) => State::Off,
        }
    }
}

impl BitOr for State {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (State::On, State::On) => State::On,
            (State::On, State::Off) => State::On,
            (State::Off, State::On) => State::On,
            (State::Off, State::Off) => State::Off,
        }
    }
}

impl BitXor for State {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (State::On, State::On) => State::Off,
            (State::On, State::Off) => State::On,
            (State::Off, State::On) => State::On,
            (State::Off, State::Off) => State::Off,
        }
    }
}

#[derive(Debug)]
struct Wire {
    value: Option<State>,
}

impl Wire {
    fn new() -> Self {
        Self { value: None }
    }

    fn update(&mut self, value: State) {
        self.value = Some(value);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GateType {
    AND,
    OR,
    XOR,
}

impl From<&str> for GateType {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gate {
    input1: Rc<str>,
    input2: Rc<str>,
    output: Rc<str>,
    operation: GateType,
}

impl Gate {
    fn new(input1: Rc<str>, input2: Rc<str>, output: Rc<str>, operation: &str) -> Self {
        Self {
            input1,
            input2,
            output,
            operation: operation.into(),
        }
    }
}

#[derive(Debug)]
struct Device {
    wires: FxHashMap<Rc<str>, Wire>,
    gates: Vec<Gate>,
    z_outputs: Vec<Rc<str>>,
}

impl Device {
    fn new(input: &str) -> Self {
        let mut wires: HashMap<Rc<str>, _, _> = FxHashMap::default();
        let mut gates = Vec::new();
        let mut z_outputs = Vec::new();

        let (init_values, circuits) = input.trim_end().split_once("\n\n").unwrap();

        for line in circuits.lines() {
            let (left_side, output) = line.split_once(" -> ").unwrap();

            let output: Rc<str> = output.into();
            wires.insert(output.clone(), Wire::new());
            if output.starts_with('z') {
                z_outputs.push(output.clone());
            }

            let mut gate_split = left_side.split(" ");

            let input1: Rc<str> = gate_split.next().unwrap().into();
            wires.insert(input1.clone(), Wire::new());

            let operation = gate_split.next().unwrap();

            let input2: Rc<str> = gate_split.next().unwrap().into();
            wires.insert(input2.clone(), Wire::new());

            gates.push(Gate::new(input1, input2, output, operation));
        }

        for line in init_values.lines() {
            let (label, value) = line.split_once(": ").unwrap();
            wires
                .entry(label.into())
                .and_modify(|wire| wire.update(value.into()));
        }

        z_outputs.sort_unstable();

        Self {
            wires,
            gates,
            z_outputs,
        }
    }

    fn evaluate_gates(&mut self) {
        // take ownership for duration of function
        let gates = mem::take(&mut self.gates);
        while self.get_outputs().contains(&None) {
            for gate in &gates {
                self.evaluate(gate);
            }
        }
        // give it back
        self.gates = gates;
    }

    fn evaluate(&mut self, gate: &Gate) {
        if self.wires[&gate.output].value.is_none() {
            let new_value = Some(
                match (
                    self.wires[&gate.input1].value,
                    self.wires[&gate.input2].value,
                    gate.operation,
                ) {
                    (Some(state1), Some(state2), GateType::AND) => state1 & state2,
                    (Some(state1), Some(state2), GateType::OR) => state1 | state2,
                    (Some(state1), Some(state2), GateType::XOR) => state1 ^ state2,
                    _ => return,
                },
            );

            self.wires
                .entry(gate.output.clone())
                .and_modify(|wire| wire.value = new_value);
        }
    }

    fn get_outputs(&self) -> Vec<Option<State>> {
        let mut output = Vec::with_capacity(self.z_outputs.len());

        self.z_outputs.iter().for_each(|z| {
            output.push(self.wires[z].value);
        });
        output
    }

    fn get_value(&self) -> u64 {
        let mut output_bits = self.get_outputs();
        output_bits.reverse();

        let mut value = 0;

        for &bit in output_bits.iter().flatten() {
            value <<= 1;
            if bit == State::On {
                value |= 1;
            }
        }

        value
    }

    fn find_swapped_wires(&self) -> Vec<Rc<str>> {
        let mut edges: HashMap<Rc<str>, Vec<Rc<str>>, _> = FxHashMap::default();
        for gate in &self.gates {
            edges
                .entry(gate.input1.clone())
                .or_default()
                .push(gate.output.clone());
            edges
                .entry(gate.input2.clone())
                .or_default()
                .push(gate.output.clone());
        }

        let mut broken = FxHashSet::default();

        let final_label: Rc<str> = format! {"z{}", self.z_outputs.len() -1}.into();
        for gate in &self.gates {
            // the output bits except the final one must be connected to an XOR gate
            if gate.output.starts_with('z')
                && gate.output != final_label
                && gate.operation != GateType::XOR
            {
                broken.insert(gate.output.clone());
            }

            // z wires must be outputs only, never an input
            if gate.input1.starts_with('z') {
                broken.insert(gate.input1.clone());
            }

            if gate.input2.starts_with('z') {
                broken.insert(gate.input2.clone());
            }

            // XOR gates are only connected to x or y inputs and a z output
            if gate.operation == GateType::XOR
                && !gate.output.starts_with('z')
                && !((gate.input1.starts_with('x') && gate.input2.starts_with('y'))
                    || (gate.input1.starts_with('y') && gate.input2.starts_with('x')))
            {
                broken.insert(gate.output.clone());
            }

            // non z wires from XOR gates msut input to two other gates
            if gate.operation == GateType::XOR
                && !gate.output.starts_with('z')
                && edges[&gate.output].len() != 2
            {
                broken.insert(gate.output.clone());
            }

            // AND gates always output to only 1 node, save for the first addr
            if gate.operation == GateType::AND
                && !gate.output.starts_with('z')
                && edges[&gate.output].len() != 1
                && !((*gate.input1 == *"x00" && *gate.input2 == *"y00")
                    || (*gate.input1 == *"y00" && *gate.input2 == *"x00"))
            {
                broken.insert(gate.output.clone());
            }
        }

        broken.into_iter().collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut device = Device::new(input);

    device.evaluate_gates();

    Some(device.get_value())
}

pub fn part_two(input: &str) -> Option<String> {
    let device = Device::new(input);

    let mut broken = device.find_swapped_wires();

    broken.sort_unstable();

    Some(broken.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
