use aoc::*;
use cached::proc_macro::cached;
use glam::U64Vec2;
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../../input/13");

fn main() {
    assert_example!(part1, "13-test", 480);
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(ClawMachine::tokens_to_win)
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(ClawMachine::fix_conversion_error)
        .map(ClawMachine::tokens_to_win2)
        .sum()
}

fn parse(input: &str) -> Vec<ClawMachine> {
    input.split("\n\n").map(ClawMachine::parse).collect_vec()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct ClawMachine {
    a: U64Vec2,
    b: U64Vec2,
    prize: U64Vec2,
}

impl ClawMachine {
    fn fix_conversion_error(self) -> Self {
        Self {
            prize: U64Vec2::new(self.prize.x + 10000000000000, self.prize.y + 10000000000000),
            ..self
        }
    }

    fn tokens_to_win(self) -> usize {
        tokens_to_win(self, 0, 0, U64Vec2::ZERO)
    }

    fn tokens_to_win2(self) -> usize {
        tokens_to_win2(self, 0, 0, U64Vec2::ZERO)
    }

    fn parse(input: &str) -> Self {
        let re =
            Regex::new(r"X\+(\d+), Y\+(\d+)\n.*X\+(\d+), Y\+(\d+)\n.*X=(\d+), Y=(\d+)").unwrap();
        let captures = re.captures(input).unwrap();
        Self {
            a: U64Vec2::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            b: U64Vec2::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            prize: U64Vec2::new(captures[5].parse().unwrap(), captures[6].parse().unwrap()),
        }
    }
}

#[cached]
fn tokens_to_win(machine: ClawMachine, pressed_a: usize, pressed_b: usize, pos: U64Vec2) -> usize {
    if pressed_a > 100 || pressed_b > 100 {
        0
    } else if pos == machine.prize {
        pressed_a * 3 + pressed_b
    } else if pos.x > machine.prize.x || pos.y > machine.prize.y {
        0
    } else {
        let a = tokens_to_win(machine, pressed_a + 1, pressed_b, pos + machine.a);
        let b = tokens_to_win(machine, pressed_a, pressed_b + 1, pos + machine.b);
        match (a, b) {
            (0, 0) => 0,
            (a, 0) => a,
            (0, b) => b,
            (a, b) => a.min(b),
        }
    }
}

#[cached]
fn tokens_to_win2(machine: ClawMachine, pressed_a: usize, pressed_b: usize, pos: U64Vec2) -> usize {
    if pos == machine.prize {
        pressed_a * 3 + pressed_b
    } else if pos.x > machine.prize.x || pos.y > machine.prize.y {
        0
    } else {
        let a = tokens_to_win(machine, pressed_a + 1, pressed_b, pos + machine.a);
        let b = tokens_to_win(machine, pressed_a, pressed_b + 1, pos + machine.b);
        match (a, b) {
            (0, 0) => 0,
            (a, 0) => a,
            (0, b) => b,
            (a, b) => a.min(b),
        }
    }
}
