use aoc::*;
use glam::IVec2;
use std::collections::{HashMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../input/12");

fn main() {
    assert_example!(part1, "12-test", 140);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "12-test", 0);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse_regions(input).iter().map(Region::fence_cost).sum()
}
fn part2(_input: &str) -> usize {
    0
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
}

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    // TODO Reading a grid with coordinates should be a helper function.
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
