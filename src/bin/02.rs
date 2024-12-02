use aoc::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/02");

fn main() {
    assert_example!(part1, "02-test", 2);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "02-test", 4);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|report| is_safe(report))
        .count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|report| is_dampened_safe(report))
        .count()
}

fn is_safe(report: &[i64]) -> bool {
    let monotonic = decreasing(report) || increasing(report);
    monotonic && not_steep(report)
}

fn is_dampened_safe(report: &[i64]) -> bool {
    (0..report.len()).any(|i| {
        let mut report = report.to_owned();
        report.remove(i);
        is_safe(&report)
    })
}

fn increasing(report: &[i64]) -> bool {
    gradient(report).all(|n| n > 0)
}

fn decreasing(report: &[i64]) -> bool {
    gradient(report).all(|n| n < 0)
}

fn not_steep(report: &[i64]) -> bool {
    gradient(report).all(|n| n.abs() <= 3)
}

fn gradient(report: &[i64]) -> impl Iterator<Item = i64> + use<'_> {
    report.iter().copied().tuple_windows().map(|(a, b)| b - a)
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}
