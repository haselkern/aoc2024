use aoc::*;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../input/12");

fn main() {
    assert_example!(part1, "12-test", 140);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "12-test", 80);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse_regions(input).iter().map(Region::fence_cost).sum()
}

fn part2(input: &str) -> usize {
    parse_regions(input)
        .iter()
        .map(Region::bulk_fence_cost)
        .sum()
}

fn parse_regions(input: &str) -> Vec<Region> {
    let mut map = parse_map(input);
    let mut regions = Vec::new();
    loop {
        let Some((&start_pos, &plant)) = map.iter().next() else {
            break;
        };
        let mut visited = HashSet::new();
        let mut region = Vec::new();
        let mut check = vec![start_pos];
        let mut perimeter = 0;
        while let Some(next) = check.pop() {
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            let Some(&p) = map.get(&next) else { continue };
            if p != plant {
                continue;
            }

            region.push(next);
            map.remove(&next);

            // Adjust perimeter
            let empty_sides = DIRECTIONS4
                .into_iter()
                .map(|dir| dir + next)
                .filter(|pos| !region.contains(pos))
                .count();
            let taken_sides = 4 - empty_sides;
            perimeter = perimeter + empty_sides - taken_sides;

            let neighbors = DIRECTIONS4.into_iter().map(|dir| dir + next);
            check.extend(neighbors);
        }
        regions.push(Region {
            _plant: plant,
            fields: region,
            perimeter,
        });
    }
    regions
}

#[derive(Debug)]
struct Region {
    _plant: char,
    fields: Vec<IVec2>,
    perimeter: usize,
}

impl Region {
    fn area(&self) -> usize {
        self.fields.len()
    }

    fn fence_cost(&self) -> usize {
        self.perimeter * self.area()
    }

    fn bulk_fence_cost(&self) -> usize {
        self.corners() * self.area()
    }

    /// Detect corners by moving a 2x2 window over the region and checking the shape.
    /// A region has as many corners as edges.
    fn corners(&self) -> usize {
        let min = self.fields.iter().fold(IVec2::ZERO, |acc, &x| acc.min(x)) - IVec2::ONE;
        let max = self.fields.iter().fold(IVec2::ZERO, |acc, &x| acc.max(x));

        let clockwise = [
            IVec2::ZERO,
            IVec2::new(1, 0),
            IVec2::new(1, 1),
            IVec2::new(0, 1),
        ];

        let mut corners = 0;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let p = IVec2::new(x, y);
                let pattern = clockwise
                    .iter()
                    .map(|dir| p + dir)
                    .map(|p| self.fields.contains(&p))
                    .collect_vec();
                let count = pattern.iter().filter(|b| **b).count();
                match count {
                    0 | 4 => (),
                    1 | 3 => corners += 1,
                    2 if pattern == [true, false, true, false] => corners += 2,
                    2 if pattern == [false, true, false, true] => corners += 2,
                    2 => (),
                    other => unreachable!("should not have {other} elements when checking corners"),
                }
            }
        }

        corners
    }
}

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().zip(iter::repeat(y)))
        .map(|((x, c), y)| {
            let pos = IVec2::new(x as i32, y as i32);
            (pos, c)
        })
        .collect()
}
