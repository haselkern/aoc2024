use aoc::*;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/18");

fn main() {
    assert_example!(part1_example, "18-test", 22);
    println!("Part 1: {}", part1(INPUT, 70, 1024));
    assert_example!(part2_example, "18-test", "6,1");
    println!("Part 2: {}", part2(INPUT, 70));
}

fn part1_example(input: &str) -> usize {
    part1(input, 6, 12)
}

fn part1(input: &str, size: i32, steps: usize) -> usize {
    let blocks = parse(input);
    simulate(size, steps, &blocks)
}

fn part2_example(input: &str) -> String {
    part2(input, 6)
}

fn part2(input: &str, size: i32) -> String {
    let blocks = parse(input);
    let mut steps = 1;
    let mut div = blocks.len() / 2;

    // Binary search for the number of steps
    loop {
        let a_works = simulate(size, steps, &blocks) < usize::MAX;
        let b_works = simulate(size, steps + 1, &blocks) < usize::MAX;
        match (a_works, b_works) {
            (true, false) => break,
            (true, true) => {
                steps += div;
                div = 1.max(div / 2);
            }
            (false, false) => {
                steps -= div;
                div = 1.max(div / 2);
            }
            (false, true) => unreachable!(),
        }
    }

    let blocker = blocks[steps];
    format!("{},{}", blocker.x, blocker.y)
}

fn simulate(size: i32, steps: usize, blocks: &[IVec2]) -> usize {
    let walls: HashSet<IVec2> = blocks.iter().take(steps).copied().collect();
    let mut grid: HashMap<IVec2, Tile> = (0..=size)
        .cartesian_product(0..=size)
        .map(|(i, j)| IVec2::new(i, j))
        .filter_map(|pos| {
            if walls.contains(&pos) {
                None
            } else {
                Some((
                    pos,
                    Tile {
                        pos,
                        visited: false,
                        distance: usize::MAX,
                    },
                ))
            }
        })
        .collect();

    grid.insert(
        IVec2::ZERO,
        Tile {
            pos: IVec2::ZERO,
            visited: false,
            distance: 0,
        },
    );

    loop {
        let next = grid
            .values()
            .filter(|t| !t.visited)
            .min_by_key(|t| t.distance)
            .cloned();
        let Some(next) = next else {
            break;
        };
        grid.insert(
            next.pos,
            Tile {
                visited: true,
                ..next
            },
        );

        for dir in DIRECTIONS4 {
            let neighbor = next.pos + dir;
            let Some(neighbor) = grid.get_mut(&neighbor) else {
                continue;
            };
            if neighbor.visited {
                continue;
            }

            let Some(new_distance) = next.distance.checked_add(1) else {
                continue;
            };

            if new_distance < neighbor.distance {
                neighbor.distance = new_distance;
            }
        }
    }

    grid.get(&IVec2::splat(size)).unwrap().distance
}

#[derive(Clone, Debug)]
struct Tile {
    pos: IVec2,
    visited: bool,
    distance: usize,
}

fn parse(input: &str) -> Vec<IVec2> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> IVec2 {
    let (x, y) = line.split_once(',').unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    IVec2::new(x, y)
}
