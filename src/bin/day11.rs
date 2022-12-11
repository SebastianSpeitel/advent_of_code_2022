use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
enum Operation {
    Mul(u64),
    Add(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test_value: u64,
    test_true: usize,
    test_false: usize,
    inspected_items: usize,
}

type ThrownItem = (usize, u64);

impl Monkey {
    fn inspect(&self, mut item: u64) -> ThrownItem {
        match self.operation {
            Operation::Mul(x) => item *= x,
            Operation::Add(x) => item += x,
            Operation::Square => item *= item,
        }
        item /= 3;
        match item % self.test_value {
            0 => (self.test_true, item),
            _ => (self.test_false, item),
        }
    }

    fn inspect_all(&mut self) -> Vec<ThrownItem> {
        let mut result = Vec::new();
        for item in self.items.drain(..).collect::<Vec<_>>() {
            self.inspected_items += 1;
            result.push(self.inspect(item));
        }
        result
    }
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse()
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(2)
            .map(|x| x.trim_end_matches(',').parse().unwrap())
            .collect();
        let op: Vec<_> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(4)
            .collect();
        dbg!(&op);
        let operation = match op[0] {
            "*" if op[1] == "old" => Operation::Square,
            "*" => Operation::Mul(op[1].parse().unwrap()),
            "+" => Operation::Add(op[1].parse().unwrap()),
            _ => unreachable!("{op:?}"),
        };
        let test_value = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let test_true = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        let test_false = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();

        Ok(Self {
            id,
            items,
            operation,
            test_value,
            test_true,
            test_false,
            inspected_items: 0,
        })
    }
}

fn part1(input: &str) -> String {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|x| x.parse().unwrap()).collect();
    dbg!(&monkeys);
    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect_all();
            dbg!(&throws);
            for (id, item) in throws {
                monkeys[id].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    dbg!(&monkeys);

    (monkeys[0].inspected_items * monkeys[1].inspected_items).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() {
    let input = include_str!("../input/day11.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
