use aoc::*;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../input/08");

fn main() {
    assert_example!(part1, "08-test", 14);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "08-test", 34);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (map, grid_size) = parse(input);
    let mut antinodes = HashSet::new();

    for antennas in map.values() {
        for (&a, &b) in antennas.iter().tuple_combinations() {
            let distance = b - a;
            let antinode = b + distance;
            if grid_contains(antinode, grid_size) {
                antinodes.insert(antinode);
            }
            let antinode = a - distance;
            if grid_contains(antinode, grid_size) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let (map, grid_size) = parse(input);
    let mut antinodes = HashSet::new();
    const RESONANCE: i32 = 50;

    for antennas in map.values() {
        for (&a, &b) in antennas.iter().tuple_combinations() {
            let distance = b - a;
            for r in -RESONANCE..RESONANCE {
                let antinode = b + distance * r;
                if grid_contains(antinode, grid_size) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

fn grid_contains(point: IVec2, size: IVec2) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < size.x && point.y < size.y
}

fn parse(input: &str) -> (HashMap<char, Vec<IVec2>>, IVec2) {
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .zip(iter::repeat(y))
                .map(|((x, c), y)| (c, IVec2::new(x as i32, y as i32)))
        })
        .filter(|&(c, _)| c != '.');

    let mut map: HashMap<char, Vec<IVec2>> = HashMap::new();
    for (c, pos) in antennas {
        map.entry(c).or_default().push(pos);
    }

    let size = IVec2::new(
        input.lines().count() as i32,
        input.lines().next().unwrap().len() as i32,
    );

    (map, size)
}
