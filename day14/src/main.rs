use std::cmp;
use std::collections::HashMap;
use std::fs;

fn day14a(mut grid: HashMap<(i32, i32), char>, height: i32) {
    let mut ctr = 0;
    loop {
        ctr += 1;
        let (mut x, mut y) = (500, 0);

        while y <= height {
            if !grid.contains_key(&(x, y + 1)) {
                y += 1;
            } else if !grid.contains_key(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
            } else if !grid.contains_key(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }

        if y > height {
            break;
        } else {
            grid.insert((x, y), 'o');
        }
    }

    println!("{}", ctr - 1);
}

fn day14b(mut grid: HashMap<(i32, i32), char>, height: i32) {
    let mut ctr = 0;
    while !grid.contains_key(&(500, 0)) {
        ctr += 1;
        let (mut x, mut y) = (500, 0);

        while y <= height {
            if !grid.contains_key(&(x, y + 1)) {
                y += 1;
            } else if !grid.contains_key(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
            } else if !grid.contains_key(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }

        grid.insert((x, y), 'o');
    }

    println!("{}", ctr);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut grid = HashMap::<(i32, i32), char>::new();
    let mut height = 0;

    for path in input.lines() {
        let mut prev = None;
        for point in path.split(" -> ") {
            if let [x, y] = point
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                let point = (x, y);
                height = cmp::max(height, y);

                if let Some((mut a, mut b)) = prev {
                    while a != x || b != y {
                        if a < x {
                            a += 1;
                        }
                        if a > x {
                            a -= 1;
                        }
                        if b < y {
                            b += 1;
                        }
                        if b > y {
                            b -= 1;
                        }

                        grid.insert((a, b), '#');
                    }
                }
                grid.insert(point, '#');

                prev = Some(point);
            }
        }
    }

    day14a(grid.clone(), height);
    day14b(grid, height);
}
