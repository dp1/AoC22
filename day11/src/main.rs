use std::collections::{HashMap, VecDeque};
use std::fs;

/// Returns a vec with all the space separated integers present in the line
fn parse_ints(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|x| x.parse::<i64>())
        .filter_map(|x| x.ok())
        .collect()
}

#[derive(Debug)]
enum Operand {
    Const(i64),
    Old,
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn exec(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
        }
    }
}

#[derive(Debug)]
struct Operation {
    a: Operand,
    op: Operator,
    b: Operand,
}

impl Operation {
    fn exec(&self, old: i64) -> i64 {
        let a = match self.a {
            Operand::Const(x) => x,
            Operand::Old => old,
        };
        let b = match self.b {
            Operand::Const(x) => x,
            Operand::Old => old,
        };

        self.op.exec(a, b)
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn parse(lines: &[&str]) -> Monkey {
        let mut res = Monkey {
            items: VecDeque::new(),
            operation: Operation {
                a: Operand::Old,
                op: Operator::Add,
                b: Operand::Old,
            },
            test: 0,
            throw_true: 0,
            throw_false: 0,
        };

        for (i, line) in lines.iter().enumerate() {
            match i {
                1 => res.items = VecDeque::from(parse_ints(&line.replace(',', ""))),
                2 => {
                    let pieces = line.split(' ').rev().take(3).collect::<Vec<_>>();
                    res.operation.b = match pieces[0] {
                        "old" => Operand::Old,
                        s => Operand::Const(s.parse().unwrap()),
                    };
                    res.operation.op = match pieces[1] {
                        "+" => Operator::Add,
                        _ => Operator::Mul,
                    };
                    res.operation.a = match pieces[2] {
                        "old" => Operand::Old,
                        s => Operand::Const(s.parse().unwrap()),
                    };
                }
                3 => res.test = parse_ints(line)[0],
                4 => res.throw_true = parse_ints(line)[0] as usize,
                5 => res.throw_false = parse_ints(line)[0] as usize,
                _ => (),
            }
        }

        res
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut res = Vec::new();
    let lines = input.lines().collect::<Vec<_>>();

    for i in (0..lines.len()).step_by(7) {
        res.push(Monkey::parse(&lines[i..i + 6]));
    }

    res
}

fn day11a(input: &str) {
    let mut monkeys = parse_input(input);
    let mut pending = HashMap::<usize, VecDeque<i64>>::new();
    let mut counter = vec![0; monkeys.len()];

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            while let Some(value) = pending.entry(i).or_default().pop_front() {
                monkey.items.push_back(value);
            }

            while let Some(mut level) = monkey.items.pop_front() {
                counter[i] += 1;

                level = monkey.operation.exec(level);
                level /= 3;

                let index = if (level % monkey.test) == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };

                pending.entry(index).or_default().push_back(level);
            }
        }
    }

    counter.sort_by(|a, b| b.cmp(a));

    println!("{}", counter[0] * counter[1]);
}

fn day11b(input: &str) {
    let mut monkeys = parse_input(input);
    let mut pending = HashMap::<usize, VecDeque<i64>>::new();
    let mut counter = vec![0i64; monkeys.len()];

    let modulo: i64 = monkeys.iter().map(|m| m.test).product();
    assert!(modulo < i32::MAX as i64);

    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            while let Some(value) = pending.entry(i).or_default().pop_front() {
                monkey.items.push_back(value);
            }

            while let Some(mut level) = monkey.items.pop_front() {
                counter[i] += 1;

                level = monkey.operation.exec(level) % modulo;

                let index = if (level % monkey.test) == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };

                pending.entry(index).or_default().push_back(level);
            }
        }
    }

    counter.sort_by(|a, b| b.cmp(a));

    println!("{}", counter[0] * counter[1]);
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    day11a(&s);
    day11b(&s);
}
