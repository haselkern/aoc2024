use aoc::assert_example;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/17");

fn main() {
    assert_example!(part1, "17-test", "5,7,3,0");
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "17-test", 117440);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> String {
    let mut computer = Computer::parse(input);
    computer.run();
    computer.output.iter().map(usize::to_string).join(",")
}

fn part2(_input: &str) -> usize {
    0
}

#[derive(Clone)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    ip: usize,
    output: Vec<usize>,
}

impl Computer {
    fn run(&mut self) {
        while !self.is_halted() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.opcode() {
            0 => {
                let exp = self.combo_operand().try_into().unwrap();
                self.a /= 2usize.pow(exp);
                self.ip += 2;
            }
            1 => {
                self.b ^= self.literal_operand();
                self.ip += 2;
            }
            2 => {
                self.b = self.combo_operand() % 8;
                self.ip += 2;
            }
            3 => {
                if self.a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = self.literal_operand();
                }
            }
            4 => {
                self.b ^= self.c;
                self.ip += 2;
            }
            5 => {
                self.output.push(self.combo_operand() % 8);
                self.ip += 2;
            }
            6 => {
                let exp = self.combo_operand().try_into().unwrap();
                self.b = self.a / 2usize.pow(exp);
                self.ip += 2;
            }
            7 => {
                let exp = self.combo_operand().try_into().unwrap();
                self.c = self.a / 2usize.pow(exp);
                self.ip += 2;
            }
            other => panic!("invalid instruction {other}"),
        }
    }

    fn is_halted(&self) -> bool {
        self.ip >= self.program.len()
    }

    fn opcode(&self) -> usize {
        self.program[self.ip]
    }

    fn literal_operand(&self) -> usize {
        self.program[self.ip + 1]
    }

    fn combo_operand(&self) -> usize {
        match self.literal_operand() {
            n @ 0..=3 => n,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            other => panic!("invalid combo operand {other}"),
        }
    }

    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let a = lines
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap();
        let c = lines
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap();
        lines.next();
        let program = lines
            .next()
            .unwrap()
            .strip_prefix("Program: ")
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            a,
            b,
            c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }
}
