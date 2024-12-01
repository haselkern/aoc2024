use aoc::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/01");

fn main() {
    assert_example!(part1, "01-test", 11);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "01-test", 31);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (left, right) = parse(input);
    let pairs = left.into_iter().sorted().zip(right.into_iter().sorted());
    pairs.map(|(left, right)| left.abs_diff(right)).sum()
}

fn part2(input: &str) -> usize {
    let (left, right) = parse(input);
    left.into_iter().map(|l| similarity(l, &right)).sum()
}

fn similarity(left: usize, right: &[usize]) -> usize {
    let count = right.iter().filter(|&&r| r == left).count();
    count * left
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.lines().map(parse_line).unzip()
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut numbers = line.split_whitespace();
    let left = numbers.next().unwrap().parse().unwrap();
    let right = numbers.next().unwrap().parse().unwrap();
    (left, right)
}
