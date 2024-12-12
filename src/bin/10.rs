use aoc::*;
use glam::IVec2;
use std::collections::{HashMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../input/10");

fn main() {
    assert_example!(part1, "10-test", 36);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "10-test", 81);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    map.trailheads().map(|t| map.reachable(t).len()).sum()
}

fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    map.trailheads().map(|t| map.distinct_trails(t)).sum()
}

#[derive(Debug)]
struct Map {
    height: HashMap<IVec2, i64>,
}

impl Map {
    fn reachable(&self, from: IVec2) -> HashSet<IVec2> {
        let from_height = match self.get_height(from) {
            Some(9) => return [from].into(),
            None => return [].into(),
            Some(h) => h,
        };

        self.climb_up(from, from_height)
            .into_iter()
            .map(|pos| self.reachable(pos))
            .fold(HashSet::new(), |mut acc, other| {
                acc.extend(other);
                acc
            })
    }

    fn distinct_trails(&self, from: IVec2) -> usize {
        let from_height = match self.get_height(from) {
            Some(9) => return 1,
            None => return 0,
            Some(h) => h,
        };

        self.climb_up(from, from_height)
            .into_iter()
            .map(|pos| self.distinct_trails(pos))
            .sum()
    }

    fn climb_up(&self, from: IVec2, from_height: i64) -> Vec<IVec2> {
        DIRECTIONS4
            .into_iter()
            .filter_map(|dir| {
                let pos = from + dir;
                self.get_height(pos).map(|height| (pos, height))
            })
            .filter_map(|(pos, h)| {
                if h == from_height + 1 {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_height(&self, at: IVec2) -> Option<i64> {
        self.height.get(&at).copied()
    }

    fn trailheads(&self) -> impl Iterator<Item = IVec2> + use<'_> {
        self.height.iter().filter_map(|(&pos, &h)| match h {
            0 => Some(pos),
            _ => None,
        })
    }

    fn parse(input: &str) -> Self {
        let height = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().zip(iter::repeat(y)))
            .map(|((x, c), y)| {
                let pos = IVec2::new(x as i32, y as i32);
                let height = c.to_digit(10).unwrap() as i64;
                (pos, height)
            })
            .collect();

        Self { height }
    }
}
