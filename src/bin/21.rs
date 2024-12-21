use aoc::*;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter;

const INPUT: &str = include_str!("../../input/21");

fn main() {
    assert_example!(part1, "21-test", 126384);
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let pads = vec![Pad::Num, Pad::Dir, Pad::Dir];
    input
        .lines()
        .map(|code| complexity(code, pads.clone()))
        .sum()
}

fn part2(input: &str) -> usize {
    let robots = iter::repeat(Pad::Dir).take(25);
    let pads = iter::once(Pad::Num).chain(robots).collect_vec();
    input
        .lines()
        .map(|code| complexity(code, pads.clone()))
        .sum()
}

fn complexity(code: &str, pads: Vec<Pad>) -> usize {
    let sequence = expand(code.to_string(), pads);
    let num: usize = code[0..3].parse().unwrap();
    sequence * num
}

/// Return all shortest possible expansions.
#[cached]
fn expand(input: String, mut pads: Vec<Pad>) -> usize {
    let pad = if pads.is_empty() {
        return input.len();
    } else {
        pads.remove(0)
    };

    let mut output = 0;
    let mut current = 'A';

    for target in input.chars() {
        let expansion = shortest_paths(current, target, pad)
            .into_iter()
            .map(|path| {
                let mut required_moves: String = path.into_iter().map(|(_, dir)| dir).collect();
                required_moves.push('A');
                expand(required_moves, pads.clone())
            })
            .min()
            .unwrap();
        output += expansion;
        current = target;
    }

    output
}

#[cached]
fn shortest_paths(start: char, end: char, pad: Pad) -> Vec<Vec<(char, char)>> {
    if start == end {
        return vec![vec![]];
    }

    let next_chars = pad.next_moves();

    let initial_work = next_chars(start).iter().map(|&w| vec![w]);
    let mut work: VecDeque<Vec<(char, char)>> = VecDeque::from_iter(initial_work);

    let mut shortest_paths = Vec::new();
    let mut shortest_length = usize::MAX;

    while let Some(path) = work.pop_front() {
        if path.len() > shortest_length {
            // BFS can only get worse now
            break;
        }

        let head = path.last().copied().unwrap();

        if head.0 == end {
            shortest_length = path.len();
            shortest_paths.push(path);
            continue;
        }

        for &next in next_chars(head.0) {
            if path.iter().any(|&(c, _)| c == next.0) {
                // Already been there
                continue;
            }
            let mut path = path.clone();
            path.push(next);
            work.push_back(path);
        }
    }

    shortest_paths
}

type Moves = &'static [(char, char)];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Pad {
    Num,
    Dir,
}

impl Pad {
    fn next_moves(self) -> fn(char) -> Moves {
        match self {
            Self::Num => numpad_next,
            Self::Dir => dirpad_next,
        }
    }
}

fn numpad_next(current: char) -> Moves {
    match current {
        'A' => &[('0', '<'), ('3', '^')],
        '0' => &[('A', '>'), ('2', '^')],
        '1' => &[('2', '>'), ('4', '^')],
        '2' => &[('0', 'v'), ('1', '<'), ('3', '>'), ('5', '^')],
        '3' => &[('A', 'v'), ('2', '<'), ('6', '^')],
        '4' => &[('1', 'v'), ('5', '>'), ('7', '^')],
        '5' => &[('2', 'v'), ('4', '<'), ('6', '>'), ('8', '^')],
        '6' => &[('3', 'v'), ('5', '<'), ('9', '^')],
        '7' => &[('4', 'v'), ('8', '>')],
        '8' => &[('5', 'v'), ('7', '<'), ('9', '>')],
        '9' => &[('6', 'v'), ('8', '<')],
        other => unreachable!("unknown numpad char: '{other}'"),
    }
}

fn dirpad_next(current: char) -> Moves {
    match current {
        'A' => &[('>', 'v'), ('^', '<')],
        '^' => &[('v', 'v'), ('A', '>')],
        'v' => &[('<', '<'), ('>', '>'), ('^', '^')],
        '<' => &[('v', '>')],
        '>' => &[('v', '<'), ('A', '^')],
        other => unreachable!("unknown dirpad char: '{other}'"),
    }
}
