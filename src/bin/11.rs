use aoc::*;
use cached::proc_macro::cached;

const INPUT: &str = include_str!("../../input/11");

fn main() {
    assert_example!(part1, "11-test", 55312);
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse_ws_separated(input)
        .map(|stone| blink(stone, 25))
        .sum()
}

fn part2(input: &str) -> usize {
    parse_ws_separated(input)
        .map(|stone| blink(stone, 75))
        .sum()
}

#[cached]
fn blink(stone: usize, times: usize) -> usize {
    if times == 0 {
        return 1;
    }

    if stone == 0 {
        blink(1, times - 1)
    } else if even_digits(stone) {
        let (left, right) = split(stone);
        blink(left, times - 1) + blink(right, times - 1)
    } else {
        blink(stone * 2024, times - 1)
    }
}

fn even_digits(n: usize) -> bool {
    digits(n) % 2 == 0
}

fn digits(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n.ilog10() as usize + 1
    }
}

#[test]
fn test_digits() {
    assert_eq!(digits(0), 1);
    assert_eq!(digits(1), 1);
    assert_eq!(digits(9), 1);
    assert_eq!(digits(10), 2);
    assert_eq!(digits(99), 2);
    assert_eq!(digits(100), 3);
    assert_eq!(digits(999), 3);
}

fn split(n: usize) -> (usize, usize) {
    let half = digits(n) / 2;
    let split = 10usize.pow(half as u32);
    let left = n / split;
    let right = n % split;
    (left, right)
}

#[test]
fn test_split() {
    assert_eq!(split(10), (1, 0));
    assert_eq!(split(99), (9, 9));
    assert_eq!(split(253000), (253, 0));
}
