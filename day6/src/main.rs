use std::{io, fs};

fn day6(input: &str, threshold: i32) {
    let input = input.as_bytes();

    let mut ctr = [0; 26];
    let mut sum = 0;

    for (i,c) in input.iter().enumerate() {
        let idx = (c - b'a') as usize;

        if ctr[idx] == 0 {
            sum += 1;
        }
        ctr[idx] += 1;

        if i >= threshold as usize {
            let idx = (input[i - threshold as usize] - b'a') as usize;
            ctr[idx] -= 1;
            if ctr[idx] == 0 {
                sum -= 1;
            }
        }

        if sum == threshold {
            println!("{}", i+1);
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input.txt")?;

    day6(&s, 4);
    day6(&s, 14);
    Ok(())
}
