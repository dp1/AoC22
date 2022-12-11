use std::fs;

fn day10a(input: &str) {
    let mut acc = 1;
    let mut cycle = 1;
    let mut next = 20;
    let mut strength = 0;

    for line in input.lines() {
        let pieces = line.split(' ').collect::<Vec<_>>();

        let (duration, delta) = match pieces[0] {
            "noop" => (1, 0),
            "addx" => (2, pieces[1].parse::<i32>().unwrap()),
            _ => unreachable!(),
        };

        if next < cycle + duration {
            strength += acc * next;
            next += 40;
        }

        cycle += duration;
        acc += delta;
    }

    println!("{}", strength);
}

fn day10b(input: &str) {
    let mut acc = 1;
    let mut pixel = 0;
    let mut screen = [['.'; 40]; 6];

    for line in input.lines() {
        let pieces = line.split(' ').collect::<Vec<_>>();

        let (duration, delta) = match pieces[0] {
            "noop" => (1, 0),
            "addx" => (2, pieces[1].parse::<i32>().unwrap()),
            _ => unreachable!(),
        };

        for _ in 0..duration {
            if (acc - 1..=acc + 1).contains(&(pixel as i32 % 40)) {
                screen[pixel / 40][pixel % 40] = '#';
            }
            pixel = (pixel + 1) % 240;
        }

        acc += delta;
    }

    for line in screen {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    day10a(&s);
    day10b(&s);
}
