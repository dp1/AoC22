use std::cmp::{Ord, Ordering};
use std::fs;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::List(b)) => vec![Packet::Integer(*a)].cmp(b),
            (Packet::List(a), Packet::Integer(b)) => a.cmp(&vec![Packet::Integer(*b)]),
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

impl PartialEq for Packet {
    #[allow(clippy::double_comparisons)]
    fn eq(&self, other: &Self) -> bool {
        self >= other && self <= other
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse(s: &str) -> Packet {
        Packet::parse_rec(&mut s.chars().peekable())
    }

    fn parse_rec(s: &mut Peekable<Chars>) -> Packet {
        match s.next() {
            Some('[') => {
                let mut contents = Vec::new();

                if s.peek() == Some(&']') {
                    s.next();
                    return Packet::List(contents);
                }

                loop {
                    contents.push(Packet::parse_rec(s));

                    match s.next() {
                        Some(']') => break,
                        Some(',') => (),
                        _ => unreachable!(),
                    }
                }

                Packet::List(contents)
            }
            Some(digit) if digit.is_ascii_digit() => {
                let mut number = String::from(digit);

                loop {
                    match s.peek() {
                        Some(&x) if x.is_ascii_digit() => {
                            number.push(x);
                            s.next();
                        }
                        _ => break,
                    }
                }

                Packet::Integer(number.parse().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    dbg!(Packet::parse("[[1,[]],1,2]"));

    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();

    let mut chunks = lines.chunks_exact(2).enumerate();
    let mut res = 0;
    let mut packets = Vec::new();

    while let Some((i, [a, b])) = &chunks.next() {
        let a = Packet::parse(a);
        let b = Packet::parse(b);

        if a < b {
            res += i + 1;
        }

        packets.push(a);
        packets.push(b);
    }

    println!("{}", res);

    let dividers = [Packet::parse("[[2]]"), Packet::parse("[[6]]")];
    packets.extend_from_slice(&dividers);

    packets.sort();

    let found: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == dividers[0] || **x == dividers[1])
        .map(|(i, _)| i + 1)
        .product();

    println!("{}", found);
}
