use aoc::*;
use glam::IVec2;
use std::collections::HashMap;
use std::iter;

const INPUT: &str = include_str!("../../input/06");

fn main() {
    assert_example!(part1, "06-test", 41);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "06-test", 6);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    Lab::parse(input).patrol().count_visited()
}

fn part2(input: &str) -> usize {
    let lab = Lab::parse(input);

    lab.clone()
        .patrol()
        .tiles
        .into_iter()
        .filter_map(|(pos, tile)| match tile {
            Tile::Visited => Some(pos),
            _ => None,
        })
        .filter(|&modification| modification != lab.start)
        .filter(|&modification| {
            let mut lab = lab.clone();
            lab.place_wall(modification);
            lab.loops()
        })
        .count()
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Tile {
    Start,
    Empty,
    Wall,
    Visited,
}

impl Tile {
    fn parse(input: char) -> Self {
        match input {
            '^' => Tile::Start,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            other => panic!("unexpected tile '{other}'"),
        }
    }
}

#[derive(Clone, Debug)]
struct Lab {
    start: IVec2,
    tiles: HashMap<IVec2, Tile>,
    size: IVec2,
}

impl Lab {
    fn patrol(mut self) -> Self {
        let mut pos = self.start;
        let mut dir = Direction::Up;

        while self.contains(pos) {
            self.mark_visited(pos);
            (pos, dir) = self.advance(pos, dir);
        }

        self
    }

    /// <https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare>
    fn loops(&self) -> bool {
        let (mut tortoise_pos, mut tortoise_dir) = (self.start, Direction::Up);
        let (mut hare_pos, mut hare_dir) = (self.start, Direction::Up);

        loop {
            (tortoise_pos, tortoise_dir) = self.advance(tortoise_pos, tortoise_dir);
            (hare_pos, hare_dir) = self.advance(hare_pos, hare_dir);
            (hare_pos, hare_dir) = self.advance(hare_pos, hare_dir);

            if !self.contains(hare_pos) {
                return false;
            }

            if hare_pos == tortoise_pos && hare_dir == tortoise_dir {
                return true;
            }
        }
    }

    fn advance(&self, pos: IVec2, mut dir: Direction) -> (IVec2, Direction) {
        while self.looking_at_wall(pos, dir) {
            dir = dir.rotate();
        }
        (pos + dir.vec(), dir)
    }

    fn place_wall(&mut self, pos: IVec2) {
        self.tiles.insert(pos, Tile::Wall);
    }

    fn mark_visited(&mut self, pos: IVec2) {
        self.tiles.insert(pos, Tile::Visited);
    }

    fn contains(&self, position: IVec2) -> bool {
        position.x <= self.size.x && position.y <= self.size.y && position.x >= 0 && position.y >= 0
    }

    fn looking_at_wall(&self, position: IVec2, d: Direction) -> bool {
        let check = position + d.vec();
        self.tiles.get(&check) == Some(&Tile::Wall)
    }

    fn count_visited(&self) -> usize {
        self.tiles
            .values()
            .filter(|x| matches!(x, Tile::Visited))
            .count()
    }

    fn parse(input: &str) -> Self {
        let tiles: HashMap<_, _> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().zip(iter::repeat(y)))
            .map(|((x, c), y)| {
                let pos = IVec2::new(x as i32, y as i32);
                let tile = Tile::parse(c);
                (pos, tile)
            })
            .collect();

        let start = *tiles
            .iter()
            .find(|(_, c)| matches!(c, Tile::Start))
            .unwrap()
            .0;

        let size = tiles.keys().fold(IVec2::new(0, 0), |a, b| {
            IVec2::new(a.x.max(b.x), a.y.max(b.y))
        });

        Self { tiles, start, size }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn vec(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Right => IVec2::new(1, 0),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
        }
    }
}
