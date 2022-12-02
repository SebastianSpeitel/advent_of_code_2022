use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Hand {
    type Error = ();
    fn try_from(c: char) -> Result<Hand, Self::Error> {
        let h = match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => return Err(()),
        };
        Ok(h)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Paper) => Some(Ordering::Less),
            (Hand::Rock, Hand::Scissors) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Rock) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Scissors) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Rock) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::Rock, Hand::Paper) => Ordering::Less,
            (Hand::Rock, Hand::Scissors) => Ordering::Greater,
            (Hand::Paper, Hand::Rock) => Ordering::Greater,
            (Hand::Paper, Hand::Scissors) => Ordering::Less,
            (Hand::Scissors, Hand::Rock) => Ordering::Less,
            (Hand::Scissors, Hand::Paper) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

fn score(my: &Hand, other: &Hand) -> usize {
    (match my.cmp(other) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }) + match my {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn get_match(other: &Hand, goal: char) -> Hand {
    use Hand::*;
    match (goal, other) {
        ('X', Rock) => Scissors,
        ('X', Paper) => Rock,
        ('X', Scissors) => Paper,
        ('Y', Rock) => Rock,
        ('Y', Paper) => Paper,
        ('Y', Scissors) => Scissors,
        ('Z', Rock) => Paper,
        ('Z', Paper) => Scissors,
        ('Z', Scissors) => Rock,
        _ => panic!("Invalid goal"),
    }
}

fn part1(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        let other = Hand::try_from(chars[0]).unwrap();
        let my = Hand::try_from(chars[2]).unwrap();
        let s = score(&my, &other);
        dbg!(&my, &other, &s);
        total += s;
    }
    println!("Part 1: {}", total);
}

fn part2(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        let other = Hand::try_from(chars[0]).unwrap();
        let my = get_match(&other, chars[2]);
        let s = score(&my, &other);
        dbg!(&my, &other, &s, &chars[2]);
        total += s;
    }
    println!("Part 2: {}", total);
}

fn main() {
    let input = include_str!("../input/day2.txt");
    part1(input);
    part2(input);
}
