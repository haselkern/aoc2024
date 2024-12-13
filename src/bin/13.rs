use aoc::*;
use glam::I64Vec2;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

const INPUT: &str = include_str!("../../input/13");

fn main() {
    assert_example!(part1, "13-test", 480);
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    parse(input).into_iter().map(|c| c.tokens_to_win(100)).sum()
}

fn part2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(ClawMachine::fix_conversion_error)
        .map(|c| c.tokens_to_win(i64::MAX))
        .sum()
}

fn parse(input: &str) -> Vec<ClawMachine> {
    input.split("\n\n").map(ClawMachine::parse).collect_vec()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct ClawMachine {
    a: I64Vec2,
    b: I64Vec2,
    prize: I64Vec2,
}

impl ClawMachine {
    fn fix_conversion_error(self) -> Self {
        Self {
            prize: self.prize + I64Vec2::splat(10000000000000),
            ..self
        }
    }

    fn tokens_to_win(self, limit: i64) -> i64 {
        let b = (self.a.x * self.prize.y - self.a.y * self.prize.x)
            / (self.a.x * self.b.y - self.a.y * self.b.x);
        let a = (self.prize.x - self.b.x * b) / self.a.x;

        if a > limit || b > limit {
            return 0;
        }

        if (a * self.a + b * self.b) == self.prize {
            a * 3 + b
        } else {
            0
        }
    }

    fn parse(input: &str) -> Self {
        let captures = REGEX.captures(input).unwrap();
        Self {
            a: I64Vec2::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            b: I64Vec2::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            prize: I64Vec2::new(captures[5].parse().unwrap(), captures[6].parse().unwrap()),
        }
    }
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"X\+(\d+), Y\+(\d+)\n.*X\+(\d+), Y\+(\d+)\n.*X=(\d+), Y=(\d+)").unwrap()
});
