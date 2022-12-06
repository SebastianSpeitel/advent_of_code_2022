use std::collections::{HashSet, VecDeque};

fn subroutine<const L: usize>(input: &str) -> Option<usize> {
    // find the index of the first time 4 different characters appear after each other
    let mut sequence = VecDeque::from(vec![' '; L]);
    for (i, c) in input.chars().enumerate() {
        assert_eq!(sequence.len(), L);
        sequence.pop_front();
        sequence.push_back(c);
        // check if all characters are different
        let set: HashSet<_> = sequence.iter().collect();
        if set.len() == L {
            dbg!(set);
            return Some(i + 1);
        }
    }

    None
}

fn part1(input: &str) -> String {
    let index = subroutine::<4>(input).unwrap();
    index.to_string()
}

fn part2(input: &str) -> String {
    let index = subroutine::<14>(input).unwrap();
    index.to_string()
}

fn main() {
    let input = include_str!("../input/day6.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
