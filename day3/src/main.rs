use std::{collections::BTreeSet, fs::File, io::Read};

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => 42,
    }
}

fn day3a(input: &str) {
    let mut res = 0;

    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let a: BTreeSet<_> = left.chars().collect();
        let b: BTreeSet<_> = right.chars().collect();

        let c: Vec<_> = a.intersection(&b).collect();
        if let Some(x) = c.first() {
            res += priority(**x);
        }
    }

    println!("{}", res);
}

fn day3b(input: &str) {
    let mut res = 0;

    let lines: Vec<_> = input.lines().collect();

    for i in (0..lines.len()).step_by(3) {
        if let (Some(&a), Some(&b), Some(&c)) = (lines.get(i), lines.get(i + 1), lines.get(i + 2)) {
            let x: BTreeSet<_> = a.chars().collect();
            let y: BTreeSet<_> = b.chars().collect();
            let z: BTreeSet<_> = c.chars().collect();

            let xy: BTreeSet<_> = x.intersection(&y).cloned().collect();
            let xyz: Vec<_> = xy.intersection(&z).cloned().collect();

            if let Some(elem) = xyz.first() {
                res += priority(*elem);
            }
        }
    }

    println!("{}", res);
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    day3a(&s);
    day3b(&s);
}
