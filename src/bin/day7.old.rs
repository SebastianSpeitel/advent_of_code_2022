use std::{
    borrow::BorrowMut,
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
enum File {
    File {
        path: PathBuf,
        size: usize,
    },
    Dir {
        path: PathBuf,
        children: Vec<Arc<Mutex<File>>>,
    },
}

impl File {
    fn path(&self) -> &Path {
        match self {
            File::File { path, .. } => path,
            File::Dir { path, .. } => path,
        }
    }

    fn size(&self) -> usize {
        match self {
            File::File { size, .. } => *size,
            File::Dir { children, .. } => children
                .iter()
                .map(|f| f.deref().deref().lock().unwrap().size())
                .sum(),
        }
    }
}

fn part1(input: &str) -> String {
    let mut fs = HashMap::new();
    let mut cwd = Arc::new(Mutex::new(File::Dir {
        path: PathBuf::from("/"),
        children: Vec::new(),
    }));
    let path = cwd.lock().unwrap().path().to_owned();
    fs.insert(path, cwd.clone());
    for line in input.lines() {
        println!("{cwd:?}");
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                // println!("{line:?}");
                let arg = parts[2];
                if arg == ".." {
                    let path = cwd.lock().unwrap().path().to_owned();
                    cwd = fs.get(&path).unwrap().clone();
                } else {
                    let dir = fs
                        .entry(cwd.lock().unwrap().path().join(arg))
                        .or_insert_with(|| {
                            let dir = Arc::new(Mutex::new(File::Dir {
                                path: cwd.lock().unwrap().path().join(arg),
                                children: Vec::new(),
                            }));
                            if let File::Dir { children, .. } = cwd.lock().unwrap().deref_mut() {
                                children.push(dir.clone())
                            }
                            dir
                        })
                        .clone();
                    cwd = dir;
                }
            }

            continue;
        }

        if let Ok(size) = parts[0].parse::<usize>() {
            let file = Arc::new(Mutex::new(File::File {
                path: cwd.lock().unwrap().path().join(parts[1]),
                size,
            }));
            let path = file.lock().unwrap().path().to_owned();
            fs.insert(path, file);
        }
    }

    dbg!(fs.get(&PathBuf::from("/")).unwrap());

    let mut sum_size = 0;

    for (path, file) in fs {
        if matches!(file.lock().unwrap().deref(), File::File { .. }) {
            continue;
        }
        let size = file.lock().unwrap().size();
        if size <= 100000 {
            sum_size += size;
            // println!("{} {}", path.display(), file.size());
        }
    }

    sum_size.to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() {
    let input = include_str!("../input/day7.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
