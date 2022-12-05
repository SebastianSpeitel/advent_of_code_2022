use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[derive(Debug)]
struct Crate(char);

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Debug)]
struct Stack(VecDeque<Crate>);

#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl Index<usize> for Stacks {
    type Output = Stack;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Stacks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, s) in self.0.iter().enumerate() {
            write!(f, "{}: ", i + 1)?;
            for c in s.0.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Stack {
    fn top(&self) -> Option<&Crate> {
        self.0.back()
    }
}

impl Deref for Stack {
    type Target = VecDeque<Crate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    source: usize,
    target: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} --{}-> {}", self.source, self.count, self.target)
    }
}

impl Move {
    fn apply_single(&self, stacks: &mut Stacks) {
        println!("{stacks}{self}");

        for _ in 0..self.count {
            let c = stacks[self.source - 1].pop_back().unwrap();
            stacks[self.target - 1].push_back(c);
        }
    }

    fn apply_multi(&self, stacks: &mut Stacks) {
        println!("{stacks}{self}");

        let source_begin = stacks[self.source - 1].len() - self.count;
        let crates: Vec<_> = stacks[self.source - 1].drain(source_begin..).collect();
        stacks[self.target - 1].extend(crates);
    }
}

fn parse_stacks(s: &str) -> Stacks {
    let stack_count = (s.lines().next().unwrap().len() / 4) + 1;
    let mut stacks = Stacks(Vec::with_capacity(stack_count));
    for _ in 0..stack_count {
        stacks.0.push(Stack(VecDeque::new()));
    }

    for l in s.lines().rev().skip(1) {
        for (i, c) in l.chars().enumerate() {
            if i % 4 == 1 && c != ' ' {
                stacks.0[i / 4].0.push_back(Crate(c));
            }
        }
    }

    stacks
}

fn parse_moves(s: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    for l in s.lines() {
        let parts: Vec<_> = l.split_whitespace().collect();
        let count = parts[1].parse().unwrap();
        let source = parts[3].parse().unwrap();
        let target = parts[5].parse().unwrap();
        moves.push(Move {
            count,
            source,
            target,
        });
    }
    moves
}

fn parse(input: &str) -> (Stacks, Vec<Move>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks);
    let moves = parse_moves(moves);
    (stacks, moves)
}

fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse(input);

    dbg!(&stacks);
    dbg!(&moves);

    for m in moves {
        m.apply_single(&mut stacks);
    }

    let mut result = String::new();
    for s in stacks.0 {
        result.push(s.top().unwrap().0);
    }
    result
}

fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse(input);

    dbg!(&stacks);
    dbg!(&moves);

    for m in moves {
        m.apply_multi(&mut stacks);
    }

    let mut result = String::new();
    for s in stacks.0 {
        result.push(s.top().unwrap().0);
    }
    result
}

fn main() {
    let input = include_str!("../input/day5.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
