use std::cmp;
use std::collections::VecDeque;
use std::fs;

const INF: i32 = i32::MAX / 2;

fn day12(grid: &str) {
    let mut grid: Vec<_> = grid
        .lines()
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect();
    let (n, m) = (grid.len(), grid[0].len());
    let mut dist = vec![vec![INF; m]; n];

    let (mut start, mut end) = ((0, 0), (0, 0));
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == b'S' {
                start = (i, j);
                grid[i][j] = b'a';
            }
            if grid[i][j] == b'E' {
                end = (i, j);
                grid[i][j] = b'z';
            }
        }
    }

    let mut q = VecDeque::<(usize, usize)>::from(vec![start]);
    dist[start.0][start.1] = 0;

    while let Some((a, b)) = q.pop_front() {
        for (i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            match (a as i32 + i, b as i32 + j) {
                (x, y) if x >= 0 && y >= 0 && x < n as i32 && y < m as i32 => {
                    let (x, y) = (x as usize, y as usize);
                    if grid[a][b] + 1 >= grid[x][y] && dist[x][y] > dist[a][b] + 1 {
                        dist[x][y] = dist[a][b] + 1;
                        q.push_back((x, y));
                    }
                }
                _ => (),
            }
        }
    }

    println!("Part 1: {}", dist[end.0][end.1]);

    let mut dist = vec![vec![INF; m]; n];
    q.push_back(end);
    dist[end.0][end.1] = 0;

    while let Some((a, b)) = q.pop_front() {
        for (i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            match (a as i32 + i, b as i32 + j) {
                (x, y) if x >= 0 && y >= 0 && x < n as i32 && y < m as i32 => {
                    let (x, y) = (x as usize, y as usize);
                    if grid[x][y] + 1 >= grid[a][b] && dist[x][y] > dist[a][b] + 1 {
                        dist[x][y] = dist[a][b] + 1;
                        q.push_back((x, y));
                    }
                }
                _ => (),
            }
        }
    }

    let mut res = INF;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == b'a' {
                res = cmp::min(res, dist[i][j]);
            }
        }
    }

    println!("Part 2: {}", res);
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    day12(&s);
}
