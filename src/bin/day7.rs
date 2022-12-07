use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug, Default)]
struct Filesystem {
    files: HashMap<PathBuf, File>,
}

impl Filesystem {
    fn touch(&mut self, path: PathBuf, size: usize) {
        self.files.insert(path, File { size });
    }

    fn dirs(&self) -> impl Iterator<Item = &Path> {
        let mut dirs = HashSet::new();
        for path in self.files.keys() {
            let mut dir = path.parent().unwrap_or_else(|| Path::new("/"));
            dirs.insert(dir);
            while let Some(parent) = dir.parent() {
                dirs.insert(parent);
                dir = parent;
            }
        }
        dbg!(&dirs);
        dirs.into_iter()
    }

    fn size(&self, path: &Path) -> usize {
        let mut size = 0;
        for (p, f) in self.files.iter() {
            if p.starts_with(path) {
                size += f.size;
            }
        }
        size
    }
}

fn part1(input: &str) -> String {
    let mut cwd = PathBuf::from("/");

    let mut fs = Filesystem::default();

    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                let arg = parts[2];
                if arg == ".." {
                    cwd.pop();
                } else {
                    cwd.push(arg);
                }
            }
            continue;
        }
        if let Ok(size) = parts[0].parse() {
            fs.touch(cwd.join(parts[1]), size);
        }
    }

    dbg!(&fs);

    let mut total = 0;
    for dir in fs.dirs() {
        let size = fs.size(dir);
        if size <= 100000 {
            total += size;
        }
    }

    total.to_string()
}

fn part2(input: &str) -> String {
    let mut cwd = PathBuf::from("/");

    let mut fs = Filesystem::default();

    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                let arg = parts[2];
                if arg == ".." {
                    cwd.pop();
                } else {
                    cwd.push(arg);
                }
            }
            continue;
        }
        if let Ok(size) = parts[0].parse() {
            fs.touch(cwd.join(parts[1]), size);
        }
    }

    dbg!(&fs);

    let total_space = 70000000;
    let total_used = fs.size(Path::new("/"));
    let required = 30000000;

    let total_free = total_space - total_used;

    let to_free = required - total_free;

    let mut min_size = usize::MAX;
    let mut min_path = PathBuf::new();
    for dir in fs.dirs() {
        let size = fs.size(dir);
        if size >= to_free && size < min_size {
            min_size = size;
            min_path = dir.to_path_buf();
        }
    }

    let size = fs.size(&min_path);

    size.to_string()
}

fn main() {
    let input = include_str!("../input/day7.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
