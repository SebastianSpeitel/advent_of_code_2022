fn part1(input: &str) {
    let mut maximum = 0;
    let mut current = 0;

    for line in input.lines() {
        match line.parse::<usize>() {
            Ok(n) => current += n,
            Err(_) => {
                maximum = std::cmp::max(maximum, current);
                current = 0;
            }
        }
    }

    println!("Maximum Calories: {}", maximum);
}

fn part2(input: &str) {
    let mut top = vec![];

    let mut current = 0;

    for line in input.lines() {
        match line.parse::<usize>() {
            Ok(n) => current += n,
            Err(_) => {
                top.push(current);
                current = 0;
            }
        }
    }

    top.sort();

    let top3 = top.iter().rev().take(3).sum::<usize>();

    println!("Total Calories of top three elves: {}", top3);
}

fn main() {
    let input = include_str!("../input/day1.txt");
    part1(input);
    part2(input);
}
