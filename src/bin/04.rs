use aoc::*;
use glam::IVec2;

const INPUT: &str = include_str!("../../input/04");

fn main() {
    assert_example!(part1, "04-test", 18);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "04-test", 9);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let field = parse(input);
    count_over_field(&field, count_xmas_at)
}

fn part2(input: &str) -> usize {
    let field = parse(input);
    count_over_field(&field, check_x_mas)
}

fn count_over_field(field: &Field, count_at: fn(IVec2, &Field) -> usize) -> usize {
    let mut found = 0;
    for y in 0..field.data.len() {
        for x in 0..field.data[y].len() {
            let start = IVec2::new(x as i32, y as i32);
            found += count_at(start, field);
        }
    }
    found
}

fn check_x_mas(start: IVec2, field: &Field) -> usize {
    if field.get(start) != 'A' {
        return 0;
    }

    let up1 =
        field.get(start + IVec2::new(-1, -1)) == 'M' && field.get(start + IVec2::new(1, 1)) == 'S';
    let up2 =
        field.get(start + IVec2::new(-1, -1)) == 'S' && field.get(start + IVec2::new(1, 1)) == 'M';
    let up = up1 || up2;

    let down1 =
        field.get(start + IVec2::new(1, -1)) == 'M' && field.get(start + IVec2::new(-1, 1)) == 'S';
    let down2 =
        field.get(start + IVec2::new(1, -1)) == 'S' && field.get(start + IVec2::new(-1, 1)) == 'M';
    let down = down1 || down2;

    if up && down {
        1
    } else {
        0
    }
}

fn count_xmas_at(start: IVec2, field: &Field) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&&dir| check_xmas(start, dir, field))
        .count()
}

fn check_xmas(start: IVec2, direction: IVec2, field: &Field) -> bool {
    field.get(start) == 'X'
        && field.get(start + direction) == 'M'
        && field.get(start + direction * 2) == 'A'
        && field.get(start + direction * 3) == 'S'
}

const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(1, 1),
    IVec2::new(0, 1),
    IVec2::new(-1, 1),
    IVec2::new(-1, 0),
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
];

struct Field {
    data: Vec<Vec<char>>,
}

impl Field {
    fn get(&self, at: IVec2) -> char {
        self.data
            .get(at.y as usize)
            .and_then(|row| row.get(at.x as usize))
            .copied()
            .unwrap_or('.')
    }
}

fn parse(input: &str) -> Field {
    let data = input.lines().map(|l| l.chars().collect()).collect();
    Field { data }
}
