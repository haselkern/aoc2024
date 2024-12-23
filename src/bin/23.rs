use aoc::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

const INPUT: &str = include_str!("../../input/23");

fn main() {
    assert_example!(part1, "23-test", 7);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "23-test", "co,de,ka,ta");
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let graph = parse(input);

    let mut result = HashSet::new();

    let starts = graph.keys().filter(|c| c.0 == 't');

    for start in starts {
        for n1 in graph.get(start).unwrap() {
            for n2 in graph.get(n1).unwrap() {
                if n1 == n2 {
                    continue;
                }
                if graph.get(n2).unwrap().contains(start) {
                    let mut group = vec![*n1, *n2, *start];
                    group.sort();
                    result.insert(group);
                }
            }
        }
    }

    result.len()
}

fn part2(input: &str) -> String {
    let graph = parse(input);
    graph
        .keys()
        .map(|&start| clique(&graph, start))
        .max_by_key(|group| group.len())
        .unwrap()
        .into_iter()
        .sorted()
        .join(",")
}

fn clique(graph: &Graph, start: Computer) -> HashSet<Computer> {
    let mut clique = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let mut all_connected = true;
        for c in &clique {
            if !graph.get(c).unwrap().contains(&current) {
                all_connected = false;
                break;
            }
        }
        if !all_connected {
            continue;
        }
        clique.insert(current);
        let next = graph.get(&current).unwrap();
        queue.extend(next);
    }

    clique
}

type Graph = HashMap<Computer, Vec<Computer>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Computer(char, char);

impl Computer {
    fn parse(s: &str) -> Self {
        if s.len() != 2 {
            panic!("computer needs 2 chars, got '{s}'");
        }

        let mut chars = s.chars();
        Self(chars.next().unwrap(), chars.next().unwrap())
    }
}

impl Ord for Computer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}

impl PartialOrd for Computer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

fn parse(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in input.lines() {
        let (left, right) = parse_line(line);
        graph.entry(left).or_default().push(right);
        graph.entry(right).or_default().push(left);
    }

    graph
}

fn parse_line(line: &str) -> (Computer, Computer) {
    let (left, right) = line.split_once('-').unwrap();
    let left = Computer::parse(left);
    let right = Computer::parse(right);
    (left, right)
}
