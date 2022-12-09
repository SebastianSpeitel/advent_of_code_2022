use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct RopeSegment {
    head: (i32, i32),
    tail: (i32, i32),
}

#[derive(Debug)]
struct Rope(Vec<RopeSegment>);

impl Rope {
    fn with_len(len: usize) -> Self {
        let segs = (0..len)
            .map(|_| RopeSegment {
                head: (0, 0),
                tail: (0, 0),
            })
            .collect();
        Self(segs)
    }

    fn follow(&mut self) {
        let head = self.0.first().unwrap().head;
        self.0.iter_mut().fold(head, |head, mut segment| {
            segment.head = head;
            segment.follow();
            segment.tail
        });
    }

    fn head(&mut self) -> &mut (i32, i32) {
        &mut self.0.first_mut().unwrap().head
    }

    fn tail(&self) -> (i32, i32) {
        self.0.last().unwrap().tail
    }
}

impl RopeSegment {
    fn follow(&mut self) {
        match (self.head.0 - self.tail.0, self.head.1 - self.tail.1) {
            (0, 0) => {}
            (dx, 0) if dx >= 2 => self.tail.0 += 1,
            (0, dy) if dy >= 2 => self.tail.1 += 1,
            (dx, 0) if dx <= -2 => self.tail.0 -= 1,
            (0, dy) if dy <= -2 => self.tail.1 -= 1,
            (dx, dy) if dx.abs() > 1 || dy.abs() > 1 => {
                self.tail.0 += dx.signum();
                self.tail.1 += dy.signum();
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Step {
    dx: i32,
    dy: i32,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = s.split_once(' ').unwrap();
        let len: i32 = len.parse().unwrap();
        let (dx, dy) = match dir {
            "U" => (0, len),
            "D" => (0, -len),
            "L" => (-len, 0),
            "R" => (len, 0),
            _ => unreachable!(),
        };
        Ok(Self { dx, dy })
    }
}

fn part1(input: &str) -> String {
    let steps: Vec<Step> = input.lines().map(|l| l.parse().unwrap()).collect();
    dbg!(&steps);
    let mut visited = HashSet::new();
    let mut rope = RopeSegment {
        head: (0, 0),
        tail: (0, 0),
    };
    visited.insert(rope.tail);
    for step in steps {
        dbg!(&rope);
        let Step { dx, dy } = step;
        for _ in 0..dx.abs() {
            dbg!(&rope, &step);
            rope.head.0 += dx.signum();
            rope.follow();
            visited.insert(rope.tail);
        }
        for _ in 0..dy.abs() {
            rope.head.1 += dy.signum();
            rope.follow();
            visited.insert(rope.tail);
        }
    }
    visited.len().to_string()
}

fn part2(input: &str) -> String {
    let steps: Vec<Step> = input.lines().map(|l| l.parse().unwrap()).collect();
    dbg!(&steps);
    let mut visited = HashSet::new();
    let mut rope = Rope::with_len(9);
    visited.insert(rope.tail());
    for step in steps {
        dbg!(&rope);
        let Step { dx, dy } = step;
        for _ in 0..dx.abs() {
            dbg!(&rope, &step);
            rope.head().0 += dx.signum();
            rope.follow();
            visited.insert(rope.tail());
        }
        for _ in 0..dy.abs() {
            rope.head().1 += dy.signum();
            rope.follow();
            visited.insert(rope.tail());
        }
    }
    visited.len().to_string()
}

fn main() {
    let input = include_str!("../input/day9.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
