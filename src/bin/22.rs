use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter;

const INPUT: &str = include_str!("../../input/22");

fn main() {
    assert_example!(part1, "22-test", 37327623);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "22-test", 24);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse(input)
        .map(|secret| secret_numbers(secret).last().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    let buyers = parse(input);
    let possible_sales = buyers.flat_map(possible_sales);

    let mut sequences = HashMap::new();
    for (seq, price) in possible_sales {
        let price = price as usize;
        let entry = sequences.entry(seq).or_insert(0);
        *entry += price;
    }

    sequences.into_values().max().unwrap()
}

fn possible_sales(secret: usize) -> HashMap<[i8; 4], u8> {
    let mut sales = HashMap::new();
    for (a, b, c, d) in price_changes(secret).tuple_windows() {
        let seq = [a.change, b.change, c.change, d.change];
        sales.entry(seq).or_insert(d.price);
    }
    sales
}

fn price_changes(secret: usize) -> impl Iterator<Item = PriceChange> {
    let prices = secret_numbers(secret).map(|s| (s % 10) as u8);
    prices.tuple_windows().map(|(a, b)| PriceChange {
        price: b,
        change: b as i8 - a as i8,
    })
}

#[derive(Debug, Copy, Clone)]
struct PriceChange {
    price: u8,
    change: i8,
}

fn secret_numbers(mut secret: usize) -> impl Iterator<Item = usize> {
    iter::from_fn(move || {
        let s = secret;
        secret = mix_prune(secret);
        Some(s)
    })
    .take(2001)
}

fn mix_prune(mut secret: usize) -> usize {
    const MOD: usize = 16777216;

    let n = secret * 64;
    secret = (secret ^ n) % MOD;

    let n = secret / 32;
    secret = (secret ^ n) % MOD;

    let n = secret * 2048;
    secret = (secret ^ n) % MOD;

    secret
}

fn parse(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    input.lines().map(|line| line.parse().unwrap())
}
