use std::fs;

fn day1a(input: &str) {
    let res = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("{}", res);
}

fn day1b(input: &str) {
    let mut sums: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    sums.sort();

    let res: i32 = sums.iter().rev().take(3).sum();

    println!("{}", res);
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    day1a(&s);
    day1b(&s);
}
