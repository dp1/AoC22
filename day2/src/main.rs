use std::fs;

fn score(a: i32, b: i32) -> i32 {
    let mut res = b + 1;

    match b - a {
        0 => res += 3,
        1 | -2 => res += 6,
        _ => (),
    };

    res
}

fn day2a(input: &str) -> i32 {
    let mut res = 0;

    for line in input.lines() {
        let (mut a, mut b) = (
            line.bytes().nth(0).unwrap() as i32,
            line.bytes().nth(2).unwrap() as i32,
        );
        a -= b'A' as i32;
        b -= b'X' as i32;

        res += score(a, b);
    }

    res
}

fn day2b(input: &str) -> i32 {
    let mut res = 0;

    for line in input.lines() {
        let (mut a, mut c) = (
            line.bytes().nth(0).unwrap() as i32,
            line.bytes().nth(2).unwrap() as i32,
        );
        a -= b'A' as i32;
        c -= b'X' as i32;

        let b = match c {
            0 => (a - 1 + 3) % 3,
            1 => a,
            2 => (a + 1) % 3,
            _ => unreachable!(),
        };

        res += score(a, b);
    }

    res
}

const TEST: &'static str = "A Y
B X
C Z";

fn main() {
    println!("Small testcase");
    println!("{}", day2a(&TEST));
    println!("{}", day2b(&TEST));

    let s = fs::read_to_string("input.txt").unwrap();
    println!("Big testcase");
    println!("{}", day2a(&s));
    println!("{}", day2b(&s));
}
