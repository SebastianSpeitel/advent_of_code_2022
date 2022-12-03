use std::{collections::HashSet, str::FromStr};

fn priority(c: char) -> usize {
    // a => 1, b => 2, c => 3
    // A => 27, B => 28, C => 29

    if c.is_ascii_uppercase() {
        c as usize - 64 + 26
    } else {
        c as usize - 96
    }
}

struct Rucksack {
    comp1: HashSet<char>,
    comp2: HashSet<char>,
}

impl Rucksack {
    fn shared(&self) -> HashSet<char> {
        self.comp1.intersection(&self.comp2).copied().collect()
    }
    fn union(&self) -> HashSet<char> {
        self.comp1.union(&self.comp2).copied().collect()
    }
}

impl FromStr for Rucksack {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = s.len();
        let (c1, c2) = s.split_at(l / 2);

        Ok(Self {
            comp1: c1.chars().collect(),
            comp2: c2.chars().collect(),
        })
    }
}

fn part1(input: &str) {
    let rucksacks = input
        .lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;
    for r in &rucksacks {
        total += r.shared().iter().map(|c| priority(*c)).sum::<usize>();
    }

    println!("Part 1: {}", total);
}

fn part2(input: &str) {
    let rucksacks = input
        .lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;
    for r in rucksacks.chunks_exact(3) {
        let mut shared = r[0].union();
        for i in 1..3 {
            shared = shared.intersection(&&r[i].union()).copied().collect();
        }
        total += shared.iter().map(|c| priority(*c)).sum::<usize>();
    }

    println!("Part 2: {}", total);
}

fn main() {
    let input = include_str!("../input/day3.txt");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('c'), 3);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('B'), 28);
        assert_eq!(priority('C'), 29);
    }
}
