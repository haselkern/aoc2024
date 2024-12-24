use aoc::*;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::fmt;

const INPUT: &str = include_str!("../../input/24");

fn main() {
    assert_example!(part1, "24-test", 2024);
    println!("Part 1: {}", part1(INPUT));
    part2(INPUT);
}

fn part1(input: &str) -> usize {
    let mut device = Device::parse(input);
    device.propagate();
    device.integer('z')
}

fn part2(input: &str) {
    let device = Device::parse(input);

    let mistakes = device
        .gates
        .iter()
        .filter(|&&gate| !is_gate_ok(&device, gate))
        .map(|gate| gate.out)
        .unique()
        .collect_vec();

    println!("digraph {{");
    for gate in &device.gates {
        let shape = match gate.op {
            Op::And => "box",
            Op::Or => "circle",
            Op::Xor => "diamond",
        };
        let color = if mistakes.contains(&gate.out) {
            ",fillcolor=red,style=filled"
        } else {
            ""
        };
        println!(
            r#"{} [label={:?}{color},shape={shape}];"#,
            gate.out, gate.op
        );
        println!("{} -> {} [label={}];", gate.a, gate.out, gate.a);
        println!("{} -> {} [label={}];", gate.b, gate.out, gate.b);
    }
    for gate in device.gates.iter().filter(|gate| gate.out.is_output()) {
        println!("{}_out [label={},shape=doublecircle];", gate.out, gate.out);
        println!("{} -> {}_out [label={}];", gate.out, gate.out, gate.out);
    }
    println!("}}");

    eprintln!("\nYou should pipe this diagram into graphviz");
    eprintln!("and look for mistakes in the colored nodes.");
    eprintln!("    just day=24 | dot -T svg -o 24.svg");
}

/// Return true if the gate seems to be connected correctly.
/// Rules were derived by staring at circuit diagrams:
/// https://en.wikipedia.org/wiki/Adder_(electronics)
fn is_gate_ok(device: &Device, gate: Gate) -> bool {
    match gate.op {
        Op::And | Op::Xor => {
            if gate.out.is_output() && gate.out.n() <= 1 {
                return true;
            }
            if gate.a.is_input() && gate.b.is_input() {
                return true;
            }
            let a = device.gate_with_output(gate.a).unwrap();
            let b = device.gate_with_output(gate.b).unwrap();
            if a.op == Op::Or && b.op == Op::Xor || a.op == Op::Xor && b.op == Op::Or {
                return true;
            }
            eprintln!(
                "[{:?}] needs to be connected to input or to XOR and OR {gate:?}",
                gate.op
            );
            false
        }
        Op::Or => {
            let a = device.gate_with_output(gate.a);
            let b = device.gate_with_output(gate.b);
            let (a, b) = match (a, b) {
                (Some(a), Some(b)) => (a, b),
                _ => {
                    eprintln!("[Or] gate is missing an input: {gate:?}");
                    return false;
                }
            };
            if a.op != Op::And || b.op != Op::And {
                eprintln!("[Or] gate must only be connected to AND gates: {gate:?}");
                return false;
            }
            true
        }
    }
}

#[derive(Debug, Clone)]
struct Device {
    values: HashMap<Wire, bool>,
    gates: Vec<Gate>,
    swap: Vec<(Wire, Wire)>,
}

impl Device {
    fn propagate(&mut self) {
        // Limit the number of iterations to prevent loops
        for _ in 0..50 {
            let mut changed = false;

            for gate in &self.gates {
                let Some(a) = self.get_value(gate.a) else {
                    continue;
                };
                let Some(b) = self.get_value(gate.b) else {
                    continue;
                };
                let value = gate.op.eval(a, b);

                let prev = self.values.insert(gate.out, value);
                if prev != Some(value) {
                    changed = true;
                }
            }

            if !changed {
                return;
            }
        }
    }

    fn get_value(&self, wire: Wire) -> Option<bool> {
        // Swap wire, if applicable
        let wire = self
            .swap
            .iter()
            .copied()
            .find_map(|(a, b)| {
                if a == wire {
                    Some(b)
                } else if b == wire {
                    Some(a)
                } else {
                    None
                }
            })
            .unwrap_or(wire);
        self.values.get(&wire).copied()
    }

    fn gate_with_output(&self, wire: Wire) -> Option<Gate> {
        self.gates
            .iter()
            .filter(move |g| g.out == wire)
            .exactly_one()
            .copied()
            .ok()
    }

    fn integer(&self, c: char) -> usize {
        let wires = self
            .values
            .keys()
            .copied()
            .filter(|w| w.0 == c)
            .sorted_by_key(|&w| Reverse(w));
        let values = wires
            .map(|w| self.get_value(w).unwrap())
            .map(|v| v as usize);
        values.fold(0, |acc, v| (acc << 1) | v)
    }

    fn parse(input: &str) -> Self {
        let (values, gates) = input.split_once("\n\n").unwrap();
        let values = values.lines().map(parse_wire_value).collect();
        let gates = gates.lines().map(Gate::parse).collect();
        Self {
            values,
            gates,
            swap: Vec::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Gate {
    a: Wire,
    b: Wire,
    op: Op,
    out: Wire,
}

impl Gate {
    fn parse(line: &str) -> Self {
        let (input, out) = line.split_once(" -> ").unwrap();
        let [a, op, b] = input.split_whitespace().collect_vec().try_into().unwrap();
        let a = Wire::parse(a);
        let b = Wire::parse(b);
        let op = Op::parse(op);
        let out = Wire::parse(out);
        Self { a, b, op, out }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Wire(char, char, char);

impl Wire {
    fn parse(s: &str) -> Self {
        if s.len() != 3 {
            panic!("wire length must be three, got {} ({s})", s.len());
        }

        let (a, b, c) = s.chars().tuple_windows().exactly_one().unwrap();
        Self(a, b, c)
    }

    fn is_input(self) -> bool {
        self.0 == 'x' || self.0 == 'y'
    }

    fn is_output(self) -> bool {
        self.0 == 'z'
    }

    fn n(self) -> u8 {
        format!("{}{}", self.1, self.2).parse().unwrap()
    }
}

impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0, self.1, self.2).cmp(&(other.0, other.1, other.2))
    }
}

impl PartialOrd for Wire {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{}{}{}", self.0, self.1, self.2);
        f.debug_tuple("Wire").field(&s).finish()
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

fn parse_wire_value(s: &str) -> (Wire, bool) {
    let (wire, value) = s.split_once(": ").unwrap();
    let wire = Wire::parse(wire);
    let value: u8 = value.parse().unwrap();
    (wire, value > 0)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            other => panic!("unknown gate '{other}'"),
        }
    }

    fn eval(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::Xor => a ^ b,
        }
    }
}
