use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Assignement {
    min: u32,
    max: u32,
}

impl FromStr for Assignement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let min = parts.next().unwrap().parse::<u32>().unwrap();
        let max = parts.next().unwrap().parse::<u32>().unwrap();
        Ok(Self { min, max })
    }
}

fn part1(input: &str) {
    let ass_pairs = input
        .lines()
        .map(|l| {
            let asses: Vec<Assignement> = l.split(',').map(|a| a.parse().unwrap()).collect();
            (asses[0], asses[1])
        })
        .collect::<Vec<_>>();

    let mut total_overlaps = 0;
    for (a1, a2) in &ass_pairs {
        if a1.min <= a2.min && a1.max >= a2.max {
            total_overlaps += 1;
        } else if a2.min <= a1.min && a2.max >= a1.max {
            total_overlaps += 1;
        }
    }

    println!("Part 1: {}", total_overlaps);
}

fn part2(input: &str) {
    // check how many pairs overlap at all
    let ass_pairs = input
        .lines()
        .map(|l| {
            let asses: Vec<Assignement> = l.split(',').map(|a| a.parse().unwrap()).collect();
            (asses[0], asses[1])
        })
        .collect::<Vec<_>>();

    let mut overlaps = 0;
    for (a1, a2) in &ass_pairs {
        if a1.max >= a2.min && a1.min <= a2.max {
            overlaps += 1;
        } else if a2.max >= a1.min && a2.min <= a1.max {
            overlaps += 1;
        }
    }

    println!("Overlaps: {}", overlaps);
}

fn main() {
    let input = include_str!("../input/day4.txt");
    part1(input);
    part2(input);
}
