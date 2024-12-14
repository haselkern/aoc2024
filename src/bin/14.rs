use glam::IVec2;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

const INPUT: &str = include_str!("../../input/14");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut area = Area {
        robots: parse(input),
        size: IVec2::new(101, 103),
    };
    for _ in 0..100 {
        area.step();
    }
    area.safety_factor()
}

fn part2(input: &str) -> usize {
    let mut area = Area {
        robots: parse(input),
        size: IVec2::new(101, 103),
    };

    let mut seconds = 0;
    loop {
        seconds += 1;
        area.step();
        let s = area.to_string();
        if s.contains("###############################") {
            println!("{area}");
            return seconds;
        }
    }
}

struct Area {
    robots: Vec<Robot>,
    size: IVec2,
}

impl Area {
    fn middle(&self) -> IVec2 {
        (self.size - IVec2::ONE) / 2
    }

    fn step(&mut self) {
        self.robots.iter_mut().for_each(|robot| {
            robot.p = (robot.p + robot.v).rem_euclid(self.size);
        });
    }

    fn safety_factor(&self) -> usize {
        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
        let middle = self.middle();
        for robot in &self.robots {
            match (robot.p.x.cmp(&middle.x), robot.p.y.cmp(&middle.y)) {
                (Ordering::Equal, _) => continue,
                (_, Ordering::Equal) => continue,
                (Ordering::Less, Ordering::Less) => a += 1,
                (Ordering::Greater, Ordering::Less) => b += 1,
                (Ordering::Less, Ordering::Greater) => c += 1,
                (Ordering::Greater, Ordering::Greater) => d += 1,
            }
        }

        a * b * c * d
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let robots: HashSet<_> = self.robots.iter().map(|robot| robot.p).collect();

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let robot = robots.contains(&IVec2::new(x, y));
                let c = if robot { '#' } else { ' ' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct Robot {
    p: IVec2,
    v: IVec2,
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Robot {
    let line = line.strip_prefix("p=").unwrap();
    let (p, v) = line.split_once(" v=").unwrap();
    let p = parse_vec(p);
    let v = parse_vec(v);
    Robot { p, v }
}

fn parse_vec(s: &str) -> IVec2 {
    let (x, y) = s.split_once(',').unwrap();
    IVec2::new(x.parse().unwrap(), y.parse().unwrap())
}
