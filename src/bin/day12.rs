use petgraph::{algo::astar, prelude::DiGraph};
use std::{fmt::Display, ops::Index, str::FromStr};

type Tree = char;

trait TreeExt {
    fn h(&self) -> i8;
}

impl TreeExt for Tree {
    fn h(&self) -> i8 {
        if *self == 'S' {
            return 0;
        }
        if *self == 'E' {
            return 25;
        }
        // a => 0
        // z => 25
        return *self as i8 - 97;
    }
}

#[derive(Debug, Clone)]
struct Forest {
    width: usize,
    trees: Vec<Tree>,
}

impl Forest {
    fn start(&self) -> usize {
        self.trees.iter().position(|&c| c == 'S').unwrap()
    }

    fn end(&self) -> usize {
        self.trees.iter().position(|&c| c == 'E').unwrap()
    }

    fn build_graph(&self) -> DiGraph<Tree, i8, usize> {
        let mut graph = DiGraph::default();

        let w = self.width;
        let height = self.trees.len() / w;

        for tree in self.trees.iter().cloned() {
            graph.add_node(tree);
        }

        dbg!(self.width, height);
        for i in 0..self.trees.len() {
            let idx = i.into();
            let tree = self[i];
            let (x, y) = i2c(i, w);
            // dbg!(i, x, y, tree);
            if x > 0 {
                let i_left = c2i(x - 1, y, w);
                let diff = self[i_left].h() - tree.h();
                if diff <= 1 {
                    graph.update_edge(idx, i_left.into(), diff);
                }
            }
            if x < w as u32 - 1 {
                let i_right = c2i(x + 1, y, w);
                let diff = self[i_right].h() - tree.h();
                if diff <= 1 {
                    graph.update_edge(idx, i_right.into(), diff);
                }
            }
            if y > 0 {
                let i_up = c2i(x, y - 1, w);
                let diff = self[i_up].h() - tree.h();
                if diff <= 1 {
                    graph.update_edge(idx, i_up.into(), diff);
                }
            }
            if y < height as u32 - 1 {
                let i_down = c2i(x, y + 1, w);
                let diff = self[i_down].h() - tree.h();
                if diff <= 1 {
                    graph.update_edge(idx, i_down.into(), diff);
                }
            }
        }

        graph
    }

    fn route(&self, from: usize, to: usize) -> Option<(usize, Vec<usize>)> {
        dbg!(from, to);

        // let from_edges: Vec<_> = self.graph.edges(from.into()).collect();
        // dbg!(&from_edges);
        // let to_edges: Vec<_> = self.graph.edges(to.into()).collect();
        // dbg!(&to_edges);

        // let to = to-7;

        let (tx, ty) = i2c(to, self.width);
        dbg!(tx, ty);

        let graph = self.build_graph();

        let path = astar(
            &graph,
            from.into(),
            |n| n.index() == to,
            |_| 1f32,
            |t| {
                let (x, y) = i2c(t.index(), self.width);
                let dist = ((tx as f32 - x as f32).powi(2) + (ty as f32 - y as f32).powi(2)).sqrt();
                dist
            },
        );

        // dbg!(&path);

        if let Some((dist, path)) = path {
            let mut map = self.clone();
            for node in &path {
                map.trees[node.index()] = '.';
            }
            println!("{map}");

            let route = path.into_iter().map(|n| n.index()).collect::<Vec<_>>();

            return Some((dist as usize, route));
        }

        // let paths = dijkstra(&self.graph, from.into(), Some(to.into()), |_| 1);
        // // dbg!(&paths);
        // paths.get(&to.into()).unwrap().to_owned()

        None
    }

    fn route_len(&self, from: usize, to: usize) -> Option<usize> {
        let (dist, _) = self.route(from, to)?;
        Some(dist)
    }
}

impl Index<(u32, u32)> for Forest {
    type Output = Tree;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.trees[c2i(index.0, index.1, self.width)]
    }
}

impl Index<usize> for Forest {
    type Output = Tree;

    fn index(&self, index: usize) -> &Self::Output {
        &self.trees[index]
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tree) in self.trees.iter().enumerate() {
            write!(f, "{}", tree)?;
            if i % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn i2c(index: usize, width: usize) -> (u32, u32) {
    let x = (index % width) as u32;
    let y = (index / width) as u32;
    (x, y)
}

fn c2i(x: u32, y: u32, width: usize) -> usize {
    (y as usize * width) + x as usize
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = Vec::new();
        let mut width = 0;
        for line in s.lines() {
            width = line.len();
            for tree in line.chars() {
                trees.push(tree);
            }
        }

        Ok(Forest { width, trees })
    }
}

fn part1(input: &str) -> String {
    let forest = input.parse::<Forest>().unwrap();
    println!("{}", forest);

    // println!(
    //     "{:?}",
    //     Dot::with_config(
    //         &forest.build_graph(),
    //         &[Config::EdgeNoLabel, Config::NodeIndexLabel]
    //     )
    // );

    let start = forest.start();
    let end = forest.end();
    let dist = forest.route_len(start, end).unwrap();

    dist.to_string()
}

fn part2(input: &str) -> String {
    let forest = input.parse::<Forest>().unwrap();
    println!("{}", forest);

    // println!(
    //     "{:?}",
    //     Dot::with_config(
    //         &forest.build_graph(),
    //         &[Config::EdgeNoLabel, Config::NodeIndexLabel]
    //     )
    // );

    let end = forest.end();

    let min = (0..forest.trees.len())
        .into_iter()
        .filter(|i| forest[*i].h() == 0)
        .flat_map(|i| forest.route_len(i, end))
        .min()
        .unwrap();

    min.to_string()
}

fn main() {
    let input = include_str!("../input/day12.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        assert_eq!('a'.h(), 0);
        assert_eq!('z'.h(), 25);
        assert_eq!('S'.h(), 0);
        assert_eq!('E'.h(), 25);
    }
}
