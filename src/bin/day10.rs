use std::{
    collections::VecDeque,
    fmt::{Display, Write},
    str::FromStr,
};

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = s.split_ascii_whitespace();
        match args.next() {
            Some("noop") => Ok(Self::Noop),
            Some("addx") => {
                let x = args.next().unwrap().parse().unwrap();
                Ok(Self::Addx(x))
            }
            Some(_) => Err("invalid instruction"),
            None => Err("missing instruction"),
        }
    }
}

enum Operation {
    Work,
    Set(i32),
}

struct CPU {
    instructions: Vec<Instruction>,
    queue: VecDeque<Operation>,
    instruction_index: usize,
    register: i32,
}

impl FromStr for CPU {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Ok(Self {
            instructions,
            queue: Default::default(),
            instruction_index: 0,
            register: 1,
        })
    }
}

impl CPU {
    fn cycle(&mut self) {
        if let Some(op) = self.queue.pop_front() {
            match op {
                Work => {}
                Set(x) => self.register = x,
            }
        }

        if self.queue.len() > 0 {
            return;
        }

        use Instruction::*;
        use Operation::*;
        match self.instructions[self.instruction_index] {
            Noop => self.queue.push_back(Work),
            Addx(x) => {
                self.queue.push_back(Work);
                self.queue.push_back(Set(self.register + x));
            }
        }
        self.instruction_index += 1;
    }
}

impl Iterator for CPU {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.instruction_index >= self.instructions.len() {
            return None;
        }
        self.cycle();
        Some(self.register)
    }
}

struct Canvas<const W: usize, const H: usize>(Vec<bool>);

impl<const W: usize, const H: usize> Canvas<W, H> {
    fn new() -> Self {
        Self(vec![false; W * H])
    }

    fn draw(&mut self, X: i32, cycle: usize) {
        let x = cycle % W;
        let y = cycle / W;
        let lit = ((X - 1)..=(X + 1)).contains(&(x as i32));

        self.0[x + y * W] = lit;
    }
}

impl<const W: usize, const H: usize> Display for Canvas<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.chunks_exact(W) {
            for px in row {
                if *px {
                    f.write_char('#')?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn part1(input: &str) -> String {
    let cpu: CPU = input.parse().unwrap();
    let mut res = 0;
    for (cycle, reg) in cpu.enumerate() {
        let cycle = cycle + 1;
        dbg!(cycle, reg);
        if (cycle + 20) % 40 == 0 {
            res += reg * cycle as i32;
        }
        if cycle > 220 {
            break;
        }
    }

    res.to_string()
}

fn part2(input: &str) -> String {
    let cpu: CPU = input.parse().unwrap();
    let mut canvas = Canvas::<40, 6>::new();
    for (cycle, reg) in cpu.enumerate() {
        dbg!(cycle, reg);
        canvas.draw(reg, cycle);
    }

    println!("{canvas}");

    "read for yourself /\\".to_string()
}

fn main() {
    let input = include_str!("../input/day10.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
