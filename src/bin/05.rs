use aoc::*;

const INPUT: &str = include_str!("../../input/05");

fn main() {
    assert_example!(part1, "05-test", 143);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "05-test", 123);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    Puzzle::parse(input).part1()
}

fn part2(input: &str) -> usize {
    Puzzle::parse(input).part2()
}

struct Puzzle {
    rules: Rules,
    updates: Vec<Update>,
}

impl Puzzle {
    fn part1(self) -> usize {
        self.updates
            .iter()
            .filter(|u| u.valid(&self.rules))
            .map(Update::middle)
            .sum()
    }

    fn part2(self) -> usize {
        self.updates
            .into_iter()
            .filter(|u| !u.valid(&self.rules))
            .map(|mut u| {
                u.fix(&self.rules);
                u.middle()
            })
            .sum()
    }

    fn parse(input: &str) -> Self {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let rules = Rules::parse(rules);
        let updates = updates.lines().map(Update::parse).collect();
        Self { rules, updates }
    }
}

struct Rules(Vec<Rule>);

impl Rules {
    fn valid_order(&self, first: usize, second: usize) -> bool {
        !self
            .0
            .iter()
            .any(|&Rule(before, after)| after == first && before == second)
    }

    fn parse(input: &str) -> Self {
        let rules = input.lines().map(Rule::parse).collect();
        Self(rules)
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule(usize, usize);

impl Rule {
    fn parse(input: &str) -> Self {
        let (before, after) = input.split_once('|').unwrap();
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        Self(before, after)
    }
}

#[derive(Debug)]
struct Update(Vec<usize>);

impl Update {
    fn valid(&self, rules: &Rules) -> bool {
        for i in 0..self.0.len() - 1 {
            for j in i + 1..self.0.len() {
                if !rules.valid_order(self.0[i], self.0[j]) {
                    return false;
                }
            }
        }
        true
    }

    fn fix(&mut self, rules: &Rules) {
        for i in 0..self.0.len() - 1 {
            for j in i + 1..self.0.len() {
                if !rules.valid_order(self.0[i], self.0[j]) {
                    self.0.swap(i, j);
                }
            }
        }
    }

    fn middle(&self) -> usize {
        let middle = self.0.len() / 2;
        self.0[middle]
    }

    fn parse(input: &str) -> Self {
        let update = input.split(',').map(|n| n.parse().unwrap()).collect();
        Self(update)
    }
}
