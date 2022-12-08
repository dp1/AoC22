use std::{cmp, fs};

fn day8a(input: &str) {
    let grid: Vec<Vec<i32>> = input.lines().map(|line| {
        line.chars().map(|ch| {
            ch.to_string().parse::<i32>().unwrap()
        }).collect()
    }).collect();

    let (n, m) = (grid.len(), grid[0].len());

    let mut visible = vec![vec![false; m]; n];

    for i in 0..n {
        let mut top = -1;
        for j in 0..m {
            if grid[i][j] > top {
                visible[i][j] = true;
            }
            top = cmp::max(top, grid[i][j])
        }
        let mut top = -1;
        for j in (0..m).rev() {
            if grid[i][j] > top {
                visible[i][j] = true;
            }
            top = cmp::max(top, grid[i][j])
        }
    }

    for j in 0..m {
        let mut top = -1;
        for i in 0..n {
            if grid[i][j] > top {
                visible[i][j] = true;
            }
            top = cmp::max(top, grid[i][j])
        }
        let mut top = -1;
        for i in (0..n).rev() {
            if grid[i][j] > top {
                visible[i][j] = true;
            }
            top = cmp::max(top, grid[i][j])
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if visible[i][j] {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

fn day8b(input: &str) {
    let grid: Vec<Vec<i32>> = input.lines().map(|line| {
        line.chars().map(|ch| {
            ch.to_string().parse::<i32>().unwrap()
        }).collect()
    }).collect();

    let (n, m) = (grid.len(), grid[0].len());

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {

            let mut acc = 1;

            let mut x = 1;
            while i >= x {
                if grid[i - x][j] >= grid[i][j] {
                    x += 1;
                    break;
                }
                x += 1;
            }
            acc *= x - 1;

            let mut x = 1;
            while i + x < n {
                if grid[i + x][j] >= grid[i][j] {
                    x += 1;
                    break;
                }
                x += 1;
            }
            acc *= x - 1;

            let mut x = 1;
            while j >= x {
                if grid[i][j - x] >= grid[i][j] {
                    x += 1;
                    break;
                }
                x += 1;
            }
            acc *= x - 1;

            let mut x = 1;
            while j + x < m {
                if grid[i][j + x] >= grid[i][j] {
                    x += 1;
                    break;
                }
                x += 1;
            }
            acc *= x - 1;

            res = cmp::max(res, acc);
        }
    }
    println!("{}", res);
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    day8a(&s);
    day8b(&s);
}
