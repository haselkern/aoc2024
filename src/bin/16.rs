use aoc::*;
use glam::IVec2;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../input/16");

fn main() {
    assert_example!(part1, "16-test", 7036);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "16-test", 45);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    maze.dijsktra().lowest_score(maze.end)
}

fn part2(input: &str) -> usize {
    Maze::parse(input).good_seats()
}

#[derive(Clone, Debug)]
struct Tile {
    score: usize,
    visited: bool,
    previous: Vec<(IVec2, Direction)>,
}

struct Maze {
    start: IVec2,
    end: IVec2,
    tiles: HashSet<IVec2>,
}

impl Maze {
    fn good_seats(&self) -> usize {
        let dijkstra = self.dijsktra();
        let endings = dijkstra
            .0
            .iter()
            .filter(|((pos, _), _)| *pos == self.end)
            .min_set_by_key(|(_, tile)| tile.score);
        endings
            .into_iter()
            .map(|(end, _)| dijkstra.good_seats(*end))
            .fold(HashSet::new(), |mut a, b| {
                a.extend(b);
                a
            })
            .len()
    }

    fn dijsktra(&self) -> Dijkstra {
        let mut maze: HashMap<(IVec2, Direction), Tile> = self
            .tiles
            .iter()
            .copied()
            .cartesian_product(Direction::ALL)
            .zip(iter::repeat(Tile {
                score: usize::MAX,
                visited: false,
                previous: Vec::new(),
            }))
            .collect();

        maze.insert(
            (self.start, Direction::East),
            Tile {
                score: 0,
                visited: false,
                previous: Vec::new(),
            },
        );

        loop {
            // Pick unvisited with minimal distance
            let next = maze
                .iter()
                .filter(|(_, tile)| !tile.visited)
                .min_by_key(|(_, tile)| tile.score)
                .map(|(&k, v)| (k, v.clone()));
            let Some(((pos, dir), current)) = next else {
                break;
            };

            // Mark visited
            maze.insert(
                (pos, dir),
                Tile {
                    visited: true,
                    ..current
                },
            );

            // Update neighbors
            let update_neighbor = |add: usize| {
                let new_score = current.score + add;
                move |neighbor: &mut Tile| match new_score.cmp(&neighbor.score) {
                    Ordering::Less => {
                        neighbor.score = new_score;
                        neighbor.previous = vec![(pos, dir)];
                    }
                    Ordering::Equal => {
                        neighbor.previous.push((pos, dir));
                    }
                    Ordering::Greater => {}
                }
            };

            maze.entry((pos + dir.vec(), dir))
                .and_modify(update_neighbor(1));
            maze.entry((pos, dir.rotate_cw()))
                .and_modify(update_neighbor(1000));
            maze.entry((pos, dir.rotate_ccw()))
                .and_modify(update_neighbor(1000));
        }

        Dijkstra(maze)
    }

    fn parse(input: &str) -> Self {
        let mut start = IVec2::ZERO;
        let mut end = IVec2::ZERO;
        let mut tiles = HashSet::new();

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
                '#' => {}
                '.' => {
                    tiles.insert(pos);
                }
                other => panic!("unknown tile: '{other}'"),
            }
        }

        Self { start, end, tiles }
    }
}

struct Dijkstra(HashMap<(IVec2, Direction), Tile>);

impl Dijkstra {
    fn lowest_score(&self, at: IVec2) -> usize {
        Direction::ALL
            .into_iter()
            .map(|dir| self.0.get(&(at, dir)).unwrap().score)
            .min()
            .unwrap()
    }

    fn good_seats(&self, current: (IVec2, Direction)) -> HashSet<IVec2> {
        let current_tile = self.0.get(&current).unwrap();

        let mut seats = HashSet::from([current.0]);

        for &prev in &current_tile.previous {
            seats.extend(self.good_seats(prev));
        }

        seats
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
