use aoc::*;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../../input/07");

fn main() {
    assert_example!(part1, "07-test", 3749);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "07-test", 11387);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    parse(input)
        .filter(|e| e.is_possible(&[Operator::Add, Operator::Mul]))
        .map(|e| e.target)
        .sum()
}

fn part2(input: &str) -> i64 {
    parse(input)
        .filter(|e| e.is_possible(&[Operator::Add, Operator::Mul, Operator::Concat]))
        .map(|e| e.target)
        .sum()
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn eval(self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Concat => concat(a, b),
        }
    }
}

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn is_possible(&self, operators: &[Operator]) -> bool {
        let mut numbers = VecDeque::from(self.numbers.clone());
        let acc = numbers.pop_front().unwrap();
        is_possible(operators, self.target, acc, numbers)
    }
}

fn is_possible(
    operators: &[Operator],
    target: i64,
    acc: i64,
    mut remaining: VecDeque<i64>,
) -> bool {
    if target == acc && remaining.is_empty() {
        return true;
    } else if acc > target || remaining.is_empty() {
        return false;
    }

    let next = remaining.pop_front().unwrap();

    operators.iter().any(|&op| {
        let acc = op.eval(acc, next);
        is_possible(operators, target, acc, remaining.clone())
    })
}

fn parse(input: &str) -> impl Iterator<Item = Equation> + use<'_> {
    input.lines().map(parse_equation)
}

fn parse_equation(input: &str) -> Equation {
    let (target, numbers) = input.split_once(": ").unwrap();
    let target = target.parse().unwrap();
    let numbers = parse_ws_separated(numbers).collect();
    Equation { target, numbers }
}
