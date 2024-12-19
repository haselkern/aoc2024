use aoc::*;
use cached::proc_macro::cached;
use cached::UnboundCache;

const INPUT: &str = include_str!("../../input/19");

fn main() {
    assert_example!(part1, "19-test", 6);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "19-test", 16);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (available, requested) = parse(input);
    requested
        .filter(|requested| combinations(requested, &available) > 0)
        .count()
}

fn part2(input: &str) -> usize {
    let (available, requested) = parse(input);
    requested
        .map(|requested| combinations(requested, &available))
        .sum()
}

#[cached(
    type = "UnboundCache<(String, usize), usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (requested.to_string(), available.len()) }"#
)]
fn combinations(requested: &str, available: &[&str]) -> usize {
    if requested.is_empty() {
        return 1;
    }

    available
        .iter()
        .flat_map(|available| requested.strip_prefix(available))
        .map(|requested| combinations(requested, available))
        .sum()
}

fn parse(input: &str) -> (Vec<&str>, impl Iterator<Item = &str> + use<'_>) {
    let (available, requested) = input.split_once("\n\n").unwrap();
    let available = available.split(", ").collect();
    let requested = requested.lines();
    (available, requested)
}
