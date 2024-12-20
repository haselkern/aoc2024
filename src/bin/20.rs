use aoc::*;
use glam::IVec2;
use std::collections::{BTreeMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../input/20");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    count_cheats(&maze, 2)
}

fn part2(input: &str) -> usize {
    let maze = Maze::parse(input);
    count_cheats(&maze, 20)
}

fn count_cheats(maze: &Maze, max_cheat: u32) -> usize {
    let path = maze.path();
    let mut cheats = BTreeMap::new();
    for from_i in 0..(path.len() - 1) {
        for to_i in (from_i + 1)..path.len() {
            let from = path[from_i];
            let to = path[to_i];
            let distance = distance(from, to);
            if distance > max_cheat {
                // Too far to cheat
                continue;
            }
            if distance == 1 {
                // Not really a cheat
                continue;
            }
            let saved = to_i - from_i - distance as usize;
            if saved == 0 {
                // Nothing saved
                continue;
            }
            *cheats.entry(saved).or_insert(0usize) += 1;
        }
    }
    //println!("{cheats:#?}");
    cheats
        .iter()
        .filter_map(|(&saved, &times)| if saved >= 100 { Some(times) } else { None })
        .sum()
}

fn distance(a: IVec2, b: IVec2) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[derive(Clone)]
struct Maze {
    start: IVec2,
    end: IVec2,
    tiles: HashSet<IVec2>,
}

impl Maze {
    fn path(&self) -> Vec<IVec2> {
        let mut path = vec![self.start];
        loop {
            let head = *path.last().unwrap();
            if head == self.end {
                return path;
            }
            for dir in DIRECTIONS4 {
                let next = head + dir;
                if self.tiles.contains(&next) && !path.contains(&next) {
                    path.push(next);
                    break;
                }
            }
        }
    }

    fn parse(input: &str) -> Self {
        let mut start = IVec2::ZERO;
        let mut end = IVec2::ZERO;
        let mut tiles = HashSet::new();
        let mut walls = HashSet::new();

        let input = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().zip(iter::repeat(y)))
            .map(|((x, c), y)| (IVec2::new(x as i32, y as i32), c));
        for (pos, c) in input {
            match c {
                'S' => {
                    start = pos;
                    tiles.insert(pos);
                }
                'E' => {
                    end = pos;
                    tiles.insert(pos);
                }
                '#' => {
                    walls.insert(pos);
                }
                '.' => {
                    tiles.insert(pos);
                }
                other => panic!("unknown tile: '{other}'"),
            }
        }

        Self { start, end, tiles }
    }
}
