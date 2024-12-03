use aoc::*;
use regex::{Captures, Regex};

const INPUT: &str = include_str!("../../input/03");

fn main() {
    assert_example!(part1, "03-test", 161);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "03-test", 48);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .filter_map(|i| match i {
            Instruction::Mul(a, b) => Some(a * b),
            Instruction::Do | Instruction::Dont => None,
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    let mut enabled = true;
    for instruction in parse(input) {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(a, b) if enabled => {
                sum += a * b;
            }
            Instruction::Mul(_, _) => (),
        }
    }
    sum
}

fn parse(input: &str) -> Vec<Instruction> {
    let regex = Regex::new(r#"(mul\((\d+),(\d+)\)|don't\(\)|do\(\))"#).unwrap();
    regex.captures_iter(input).map(parse_instruction).collect()
}

fn parse_instruction(c: Captures) -> Instruction {
    if c[0].starts_with("mul") {
        let (a, b) = (c[2].parse().unwrap(), c[3].parse().unwrap());
        Instruction::Mul(a, b)
    } else if c[0].starts_with("don") {
        Instruction::Dont
    } else {
        Instruction::Do
    }
}

enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}
