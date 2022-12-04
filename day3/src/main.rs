use std::{fs::File, io::Read};

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => 42,
    }
}

fn to_bitmask(x: &str) -> u64 {
    let mut mask: u64 = 0;
    for ch in x.chars() {
        mask |= 1u64 << (ch as u8 - b'A');
    }
    mask
}

fn day3a(input: &str) {
    let mut res = 0;

    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let mask = to_bitmask(left) & to_bitmask(right);
        res += priority((mask.trailing_zeros() as u8 + b'A') as char);
    }

    println!("{}", res);
}

fn day3b(input: &str) {
    let mut res = 0;

    let lines: Vec<_> = input.lines().collect();

    for chunk in lines.chunks_exact(3) {
        if let [a, b, c] = chunk[..] {
            let mask = to_bitmask(a) & to_bitmask(b) & to_bitmask(c);
            res += priority((mask.trailing_zeros() as u8 + b'A') as char);
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
