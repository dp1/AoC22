use std::{
    cmp::{max, min},
    fs, io,
};

fn day4a(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line.split([',', '-']).map(|x| x.parse().unwrap()).collect();
            if let [a, b, c, d] = nums[..] {
                (a >= c && b <= d) || (a <= c && b >= d)
            } else {
                unreachable!();
            }
        })
        .count() as i32
}

fn day4b(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line.split([',', '-']).map(|x| x.parse().unwrap()).collect();
            if let [a, b, c, d] = nums[..] {
                max(a, c) <= min(b, d)
            } else {
                unreachable!();
            }
        })
        .count() as i32
}

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input.txt")?;

    println!("{}", day4a(&s));
    println!("{}", day4b(&s));

    Ok(())
}
