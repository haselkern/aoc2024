use aoc::*;
use itertools::{zip_eq, Itertools};
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../input/25");

fn main() {
    assert_example!(part1, "25-test", 3);
    println!("Part 1: {}", part1(INPUT));
}

fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(l, k)| no_overlap(l, k))
        .count()
}

fn no_overlap(lock: &Heights, key: &Heights) -> bool {
    zip_eq(lock, key).all(|(&l, &k)| l + k <= 5)
}

fn parse(input: &str) -> (Vec<Heights>, Vec<Heights>) {
    let locks = input
        .split("\n\n")
        .filter(|lines| is_lock(lines))
        .map(parse_heights)
        .collect();
    let keys = input
        .split("\n\n")
        .filter(|lines| !is_lock(lines))
        .map(parse_heights)
        .collect();
    (locks, keys)
}

fn is_lock(lines: &str) -> bool {
    lines.starts_with("#####")
}

type Heights = Vec<i64>;

fn parse_heights(lines: &str) -> Heights {
    assert_eq!(lines.lines().count(), 7);

    let mut height = BTreeMap::new();

    for line in lines.lines() {
        let indices = line
            .chars()
            .enumerate()
            .filter(|(_i, c)| *c == '#')
            .map(|(i, _c)| i);
        for index in indices {
            *height.entry(index).or_insert(-1) += 1;
        }
    }

    height.into_values().collect()
}
