use aoc::*;
use glam::IVec2;
use std::collections::{HashMap, HashSet};
use std::{fmt, iter};

const INPUT: &str = include_str!("../../input/16");

fn main() {
    assert_example!(part1, "16-test", 7036);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "16-test", 45);
    // TODO Make part 2 terminate
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut maze = Maze::parse(input);
    maze.solve();
    maze.best_score()
}

fn part2(input: &str) -> usize {
    let mut maze = Maze::parse(input);
    maze.solve();
    maze.good_seats()
}

#[derive(Clone, Debug)]
struct Tile {
    pos: IVec2,
    visited: bool,
    score: usize,
    direction: Direction,
}

impl Tile {
    fn new(pos: IVec2) -> Self {
        Self {
            pos,
            visited: false,
            score: usize::MAX,
            direction: Direction::East,
        }
    }
}

struct Maze {
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
    tiles: HashMap<IVec2, Tile>,
}

impl Maze {
    fn solve(&mut self) {
        loop {
            let next = self
                .tiles
                .values()
                .filter(|t| !t.visited)
                .min_by_key(|t| t.score);
            let Some(next) = next.cloned() else {
                break;
            };

            self.tiles.insert(
                next.pos,
                Tile {
                    visited: true,
                    ..next
                },
            );

            for dir in Direction::ALL {
                let neighbor = next.pos + dir.vec();
                let Some(neighbor) = self.tiles.get_mut(&neighbor) else {
                    continue;
                };
                if neighbor.visited {
                    continue;
                }

                let new_score = if dir == next.direction {
                    next.score + 1
                } else {
                    next.score + 1001
                };

                if new_score < neighbor.score {
                    neighbor.score = new_score;
                    neighbor.direction = dir;
                }
            }
        }

        // println!("===");
        // let size = self.tiles.iter().fold(IVec2::ZERO, |a, b| a.max(*b.0));
        // for y in 1..=size.y {
        //     for x in 1..=size.x {
        //         let pos = IVec2::new(x, y);
        //         let tile = self.tiles.get(&pos).map(|t| t.score).unwrap_or(0);
        //         print!("{tile:>6}");
        //     }
        //     println!();
        // }
    }

    fn best_score(&self) -> usize {
        self.tiles.get(&self.end).unwrap().score
    }

    fn good_seats(&self) -> usize {
        self.good_seats_rec(
            self.best_score(),
            0,
            HashSet::new(),
            Direction::East,
            self.start,
        )
        .len()
    }

    fn good_seats_rec(
        &self,
        target_score: usize,
        score: usize,
        mut visited: HashSet<IVec2>,
        dir: Direction,
        pos: IVec2,
    ) -> HashSet<IVec2> {
        if score > target_score {
            return HashSet::new();
        }

        if !self.tiles.contains_key(&pos) {
            return HashSet::new();
        }

        if visited.contains(&pos) {
            return HashSet::new();
        }
        visited.insert(pos);

        if score == target_score && pos == self.end {
            return visited;
        }

        let mut result = HashSet::new();
        {
            result.extend(self.good_seats_rec(
                target_score,
                score + 1,
                visited.clone(),
                dir,
                pos + dir.vec(),
            ));
        }
        {
            let new_dir = dir.rotate_cw();
            result.extend(self.good_seats_rec(
                target_score,
                score + 1001,
                visited.clone(),
                new_dir,
                pos + new_dir.vec(),
            ));
        }
        {
            let new_dir = dir.rotate_ccw();
            result.extend(self.good_seats_rec(
                target_score,
                score + 1001,
                visited.clone(),
                new_dir,
                pos + new_dir.vec(),
            ));
        }

        result
    }

    fn parse(input: &str) -> Self {
        let mut start = IVec2::ZERO;
        let mut end = IVec2::ZERO;
        let mut tiles = HashMap::new();
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
                    tiles.insert(
                        pos,
                        Tile {
                            pos,
                            visited: false,
                            score: 0,
                            direction: Direction::East,
                        },
                    );
                }
                'E' => {
                    end = pos;
                    tiles.insert(pos, Tile::new(pos));
                }
                '#' => {
                    walls.insert(pos);
                }
                '.' => {
                    tiles.insert(pos, Tile::new(pos));
                }
                other => panic!("unknown tile: '{other}'"),
            }
        }

        Self {
            start,
            end,
            walls,
            tiles,
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.walls.iter().fold(IVec2::ZERO, |a, b| a.max(*b));
        for y in 0..=size.y {
            for x in 0..=size.x {
                let pos = IVec2::new(x, y);
                if self.walls.contains(&pos) {
                    write!(f, "█")?;
                } else if let Some(t) = self.tiles.get(&pos) {
                    write!(f, "{}", t.direction)?;
                } else {
                    write!(f, "?")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    fn rotate_cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn rotate_ccw(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn vec(self) -> IVec2 {
        match self {
            Self::North => IVec2::new(0, -1),
            Self::East => IVec2::new(1, 0),
            Self::South => IVec2::new(0, 1),
            Self::West => IVec2::new(-1, 0),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::North => write!(f, "^"),
            Self::East => write!(f, ">"),
            Self::South => write!(f, "v"),
            Self::West => write!(f, "<"),
        }
    }
}
