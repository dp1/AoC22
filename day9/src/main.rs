use std::collections::HashSet;
use std::fs;

fn day9(input: &str, snake_len: usize) {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut snake = vec![(0, 0); snake_len];

    for line in input.lines() {
        if let [dir, steps] = line.split(' ').collect::<Vec<_>>()[..] {
            if let Ok(steps) = steps.parse::<i32>() {
                for _ in 0..steps {
                    match dir {
                        "R" => snake[0].0 += 1,
                        "L" => snake[0].0 -= 1,
                        "U" => snake[0].1 += 1,
                        "D" => snake[0].1 -= 1,
                        _ => (),
                    }

                    for i in 1..snake_len {
                        let prev = snake[i - 1];
                        let cur = &mut snake[i];

                        while i32::abs(cur.0 - prev.0) > 1 || i32::abs(cur.1 - prev.1) > 1 {
                            cur.0 += (prev.0 - cur.0).clamp(-1, 1);
                            cur.1 += (prev.1 - cur.1).clamp(-1, 1);
                        }
                    }

                    visited.insert(*snake.last().unwrap());
                }
            }
        }
    }

    println!("{}", visited.len());
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    day9(&s, 2);
    day9(&s, 10);
}
