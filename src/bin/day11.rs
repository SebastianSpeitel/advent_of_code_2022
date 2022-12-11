use std::{
    collections::{HashSet, VecDeque},
    ops::{AddAssign, DivAssign, MulAssign, Rem},
    str::FromStr,
};

#[derive(Debug)]
enum Operation<I> {
    Mul(I),
    Add(I),
    Square,
}

#[derive(Debug)]
struct Monkey<I> {
    id: usize,
    items: VecDeque<I>,
    operation: Operation<I>,
    test_value: I,
    test_true: usize,
    test_false: usize,
    inspected_items: usize,
}

type ThrownItem<I> = (usize, I);

impl<I> Monkey<I> {
    fn inspect<const D: u64>(&self, mut item: I) -> ThrownItem<I>
    where
        I: Clone,
        I: for<'a> AddAssign<&'a I>,
        I: for<'a> MulAssign<&'a I>,
        I: Rem,
        I: Default,
        <I as Rem>::Output: PartialEq<I>,
        I: DivAssign<u64>,
    {
        match &self.operation {
            Operation::Mul(x) => item *= x,
            Operation::Add(x) => item += x,
            Operation::Square => item *= &item.clone(),
        }
        item /= D;
        match item.clone() % self.test_value.clone() == I::default() {
            true => (self.test_true, item.to_owned()),
            false => (self.test_false, item.to_owned()),
        }
    }

    fn inspect_all<const D: u64>(&mut self) -> Vec<ThrownItem<I>>
    where
        I: Clone,
        I: for<'a> AddAssign<&'a I>,
        I: for<'a> MulAssign<&'a I>,
        I: Rem,
        I: Default,
        <I as Rem>::Output: PartialEq<I>,
        I: DivAssign<u64>,
    {
        let mut result = Vec::new();
        for item in self.items.drain(..).collect::<Vec<_>>() {
            self.inspected_items += 1;
            result.push(self.inspect::<D>(item));
        }
        result
    }
}

impl<I> FromStr for Monkey<I>
where
    I: FromStr,
    <I as FromStr>::Err: std::fmt::Debug,
{
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
    let mut monkeys: Vec<Monkey<u64>> = input.split("\n\n").map(|x| x.parse().unwrap()).collect();
    dbg!(&monkeys);
    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect_all::<3>();
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
