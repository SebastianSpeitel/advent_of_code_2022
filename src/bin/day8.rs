use std::{fmt::Display, ops::Index, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Forest {
    width: usize,
    height: usize,
    tree_heights: Vec<u32>,
}

impl Index<(usize, usize)> for Forest {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tree_heights[y * self.width + x]
    }
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = Vec::new();
        let mut width = 0;

        for line in s.lines() {
            let h: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            width = h.len();
            heights.extend(h);
        }

        Ok(Forest {
            width,
            height: heights.len() / width,
            tree_heights: heights,
        })
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, h) in self.tree_heights.iter().enumerate() {
            write!(f, "{}", h)?;
            if i % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Forest {
    fn line_of_sight(&self, x: usize, y: usize, dir: Direction) -> usize {
        let mut los = 0;
        let tree = self[(x, y)];
        match dir {
            Direction::North => {
                for i in (0..y).rev() {
                    los += 1;
                    if self[(x, i)] >= tree {
                        break;
                    }
                }
            }
            Direction::East => {
                for i in x + 1..self.width {
                    los += 1;
                    if self[(i, y)] >= tree {
                        break;
                    }
                }
            }
            Direction::South => {
                for i in y + 1..self.height {
                    los += 1;
                    if self[(x, i)] >= tree {
                        break;
                    }
                }
            }
            Direction::West => {
                for i in (0..x).rev() {
                    los += 1;
                    if self[(i, y)] >= tree {
                        break;
                    }
                }
            }
        }
        dbg!(x, y, dir, los);
        los
    }

    fn visible_from_outside(&self, x: usize, y: usize) -> bool {
        // This doesn't seem to work, but I'm not sure why.
        // if self.line_of_sight(x, y, Direction::North) == y + 1 {
        //     return true;
        // }
        // if self.line_of_sight(x, y, Direction::East) == self.width - x {
        //     return true;
        // }
        // if self.line_of_sight(x, y, Direction::South) == self.height - y {
        //     return true;
        // }
        // if self.line_of_sight(x, y, Direction::West) == x + 1 {
        //     return true;
        // }
        // false

        let tree = self[(x, y)];
        if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
            return true;
        }

        let mut west = true;
        for i in 0..x {
            if self[(i, y)] >= tree {
                west = false;
                break;
            }
        }
        if west {
            return true;
        }
        let mut east = true;
        for i in x + 1..self.width {
            if self[(i, y)] >= tree {
                east = false;
                break;
            }
        }
        if east {
            return true;
        }
        let mut north = true;
        for i in 0..y {
            if self[(x, i)] >= tree {
                north = false;
                break;
            }
        }
        if north {
            return true;
        }
        let mut south = true;
        for i in y + 1..self.height {
            if self[(x, i)] >= tree {
                south = false;
                break;
            }
        }
        if south {
            return true;
        }

        false
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let mut score = 1;
        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        {
            score *= self.line_of_sight(x, y, dir.to_owned());
        }
        score
    }
}

fn part1(input: &str) -> String {
    let forest: Forest = input.parse().unwrap();
    println!("{forest}");

    let visible_trees = (0..forest.tree_heights.len())
        .filter(|i| forest.visible_from_outside(i % forest.width, i / forest.width))
        .count();

    visible_trees.to_string()
}

fn part2(input: &str) -> String {
    let forest: Forest = input.parse().unwrap();
    println!("{forest}");

    let max_scenic_score = (0..forest.tree_heights.len())
        .map(|i| forest.scenic_score(i % forest.width, i / forest.width))
        .max()
        .unwrap();

    max_scenic_score.to_string()
}

fn main() {
    let input = include_str!("../input/day8.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
