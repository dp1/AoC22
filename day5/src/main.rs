use std::fs;

struct State {
    stacks: [Vec<char>; 9],
}

/*
[F]         [L]     [M]
[T]     [H] [V] [G] [V]
[N]     [T] [D] [R] [N]     [D]
[Z]     [B] [C] [P] [B] [R] [Z]
[M]     [J] [N] [M] [F] [M] [V] [H]
[G] [J] [L] [J] [S] [C] [G] [M] [F]
[H] [W] [V] [P] [W] [H] [H] [N] [N]
[J] [V] [G] [B] [F] [G] [D] [H] [G]
 1   2   3   4   5   6   7   8   9
 */

impl State {
    fn new() -> State {
        return State {
            stacks: [
                "JHGMZNTF".chars().collect(),
                "VWJ".chars().collect(),
                "GVLJBTH".chars().collect(),
                "BPJNCDVL".chars().collect(),
                "FWSMPRG".chars().collect(),
                "GHCFBNVM".chars().collect(),
                "DHGMR".chars().collect(),
                "HNMVZD".chars().collect(),
                "GNFH".chars().collect(),
            ],
        };
    }
}

fn day5a(input: &str) {
    let mut state = State::new();

    for line in input.lines() {
        let pieces: Vec<_> = line
            .split(' ')
            .map(|x| x.parse::<usize>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        if let [k, from, to] = pieces[..] {
            for _ in 0..k {
                if let Some(value) = state.stacks[from - 1].pop() {
                    state.stacks[to - 1].push(value);
                }
            }
        }
    }

    let res = state.stacks.map(|s| s.last().unwrap().to_string()).concat();
    println!("{}", res);
}

fn day5b(input: &str) {
    let mut state = State::new();
    let mut temp: Vec<char> = Vec::new();

    for line in input.lines() {
        let pieces: Vec<_> = line
            .split(' ')
            .map(|x| x.parse::<usize>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        if let [k, from, to] = pieces[..] {
            for _ in 0..k {
                if let Some(value) = state.stacks[from - 1].pop() {
                    temp.push(value);
                }
            }
            while let Some(value) = temp.pop() {
                state.stacks[to - 1].push(value);
            }
        }
    }

    let res = state.stacks.map(|s| s.last().unwrap().to_string()).concat();
    println!("{}", res);
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    day5a(&s);
    day5b(&s);
}
