use aoc::*;
use glam::IVec2;
use std::collections::HashMap;
use std::iter;

const INPUT: &str = include_str!("../../input/15");

fn main() {
    assert_example!(part1, "15-test", 10092);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "15-test", 9021);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let (mut warehouse, directions) = parse(input, false);
    warehouse.process(directions);
    warehouse.gps()
}

fn part2(input: &str) -> i32 {
    let (mut warehouse, directions) = parse(input, true);
    warehouse.process(directions);
    warehouse.gps()
}

fn parse(input: &str, expand: bool) -> (Warehouse, Vec<Direction>) {
    let (warehouse, directions) = input.split_once("\n\n").unwrap();
    let warehouse = if expand {
        let warehouse = expand_warehouse(warehouse);
        Warehouse::parse(&warehouse)
    } else {
        Warehouse::parse(warehouse)
    };
    let directions = directions
        .lines()
        .flat_map(str::chars)
        .map(Direction::from)
        .collect();
    (warehouse, directions)
}

fn expand_warehouse(warehouse: &str) -> String {
    warehouse
        .chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            '\n' => "\n",
            other => panic!("unexpected char '{other}'"),
        })
        .collect()
}

enum Tile {
    Wall,
    Box,
    BoxL,
    BoxR,
}

struct Warehouse {
    tiles: HashMap<IVec2, Tile>,
    robot: IVec2,
}

impl Warehouse {
    fn process(&mut self, directions: Vec<Direction>) {
        for dir in directions {
            self.do_move(dir);
        }
    }

    fn do_move(&mut self, dir: Direction) {
        if !self.can_move(self.robot, dir) {
            return;
        }

        self.robot += dir.vec();
        self.push_boxes(self.robot, dir);
    }

    fn push_boxes(&mut self, at: IVec2, dir: Direction) {
        match self.tiles.get(&at) {
            Some(Tile::Box) => {
                self.push_boxes(at + dir.vec(), dir);
                self.tiles.remove(&at);
                self.tiles.insert(at + dir.vec(), Tile::Box);
            }
            Some(Tile::BoxL) => {
                let r = at + IVec2::X;
                self.push_boxes(at + dir.vec(), dir);
                if dir.vertical() {
                    self.push_boxes(r + dir.vec(), dir);
                }
                self.tiles.remove(&at);
                if dir.vertical() {
                    self.tiles.remove(&r);
                }
                self.tiles.insert(at + dir.vec(), Tile::BoxL);
                if dir.vertical() {
                    self.tiles.insert(r + dir.vec(), Tile::BoxR);
                }
            }
            Some(Tile::BoxR) => {
                let l = at - IVec2::X;
                self.push_boxes(at + dir.vec(), dir);
                if dir.vertical() {
                    self.push_boxes(l + dir.vec(), dir);
                }
                self.tiles.remove(&at);
                if dir.vertical() {
                    self.tiles.remove(&l);
                }
                self.tiles.insert(at + dir.vec(), Tile::BoxR);
                if dir.vertical() {
                    self.tiles.insert(l + dir.vec(), Tile::BoxL);
                }
            }
            _ => (),
        }
    }

    fn can_move(&self, from: IVec2, dir: Direction) -> bool {
        let next = from + dir.vec();
        match self.tiles.get(&next) {
            None => true,
            Some(Tile::Wall) => false,
            Some(Tile::BoxL) if dir.vertical() => {
                self.can_move(next, dir) && self.can_move(next + IVec2::X, dir)
            }
            Some(Tile::BoxR) if dir.vertical() => {
                self.can_move(next, dir) && self.can_move(next - IVec2::X, dir)
            }
            Some(Tile::Box | Tile::BoxL | Tile::BoxR) => self.can_move(next, dir),
        }
    }

    fn gps(&self) -> i32 {
        self.tiles
            .iter()
            .map(|(&p, t)| match t {
                Tile::Box | Tile::BoxL => p.x + p.y * 100,
                _ => 0,
            })
            .sum()
    }

    fn parse(input: &str) -> Self {
        let mut robot = IVec2::splat(-1);

        let tiles = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().zip(iter::repeat(y)))
            .flat_map(|((x, c), y)| {
                let pos = IVec2::new(x as i32, y as i32);
                match c {
                    '@' => {
                        robot = pos;
                        None
                    }
                    '.' => None,
                    '#' => Some((pos, Tile::Wall)),
                    'O' => Some((pos, Tile::Box)),
                    '[' => Some((pos, Tile::BoxL)),
                    ']' => Some((pos, Tile::BoxR)),
                    other => panic!("unexpected tile '{other}' at {pos}"),
                }
            })
            .collect();

        Self { tiles, robot }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vec(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }

    fn vertical(self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            Direction::Left | Direction::Right => false,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            other => panic!("unexpected direction '{other}'"),
        }
    }
}
