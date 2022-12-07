use std::collections::HashMap;
use std::str::Lines;
use std::{cmp, env, fs};

struct Dir {
    files: HashMap<String, u64>,
    children: HashMap<String, Dir>,
    size: Option<u64>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            files: HashMap::new(),
            children: HashMap::new(),
            size: None,
        }
    }
    fn add_file(&mut self, name: &str, size: u64) {
        self.files.entry(name.to_string()).or_insert(size);
    }
    fn cd(&mut self, name: &str) -> &mut Dir {
        self.children
            .entry(name.to_string())
            .or_insert_with(Dir::new)
    }
    fn size(&mut self) -> u64 {
        match self.size {
            Some(size) => size,
            None => {
                let size = self.calc_size();
                self.size = Some(size);
                size
            }
        }
    }
    fn calc_size(&self) -> u64 {
        let mut res = 0u64;
        for size in self.files.values() {
            res += size;
        }
        for child in self.children.values() {
            res += child.calc_size();
        }
        res
    }
}

fn build_tree(root: &mut Dir, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            if let [_, path] = line.split(' ').skip(1).collect::<Vec<_>>()[..] {
                if path == ".." {
                    println!("Going back");
                    return;
                }
                println!("Going into {}", path);
                build_tree(root.cd(path), lines);
            }
        } else {
            if let [size, name] = line.split(' ').collect::<Vec<_>>()[..] {
                if let Ok(size) = size.parse::<u64>() {
                    println!("Adding file {} with size {}", name, size);
                    root.add_file(name, size);
                }
            }
        }
    }
}

fn visit_tree(root: &mut Dir) -> u64 {
    let mut res = 0;

    let size = root.size();
    if size <= 100000 {
        res += size;
    }

    for child in root.children.values_mut() {
        res += visit_tree(child);
    }

    res
}

fn find_dir_to_remove(root: &mut Dir, to_free: u64) -> Option<u64> {
    let mut res: Option<u64> = None;

    if root.size() >= to_free {
        res = Some(cmp::min(res.unwrap_or(u64::MAX), root.size()));
    }

    for child in root.children.values_mut() {
        if let Some(value) = find_dir_to_remove(child, to_free) {
            res = Some(cmp::min(res.unwrap_or(u64::MAX), value));
        }
    }

    res
}

fn day7(input: &str) {
    let mut lines = input.lines();
    let mut root = Dir::new();

    // Skip "cd /"
    lines.next();

    build_tree(&mut root, &mut lines);
    println!("{}", visit_tree(&mut root));

    let free_space = 70000000 - root.size();
    let to_free = 30000000 - free_space;

    println!("{}", find_dir_to_remove(&mut root, to_free).unwrap());
}

fn main() {
    let input_name = if env::args().any(|a| a == "--small") {
        "input.small.txt"
    } else {
        "input.txt"
    };

    let s = fs::read_to_string(input_name).unwrap();
    day7(&s);
}
