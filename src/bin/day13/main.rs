use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use logos::{Lexer, Logos};
use std::{cmp::Ordering, str::Lines};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    #[regex(r"[,]+", logos::skip)]
    Error,

    #[token("[")]
    BeginList,

    #[token("]")]
    EndList,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(u64),
}

#[derive(Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(u64),
}

fn parse_tokens(tokens: &mut Lexer<Token>) -> Option<Packet> {
    match tokens.next() {
        Some(Token::Number(x)) => Some(Packet::Number(x)),
        Some(Token::BeginList) => Some(Packet::List(
            std::iter::from_fn(|| parse_tokens(tokens)).collect_vec(),
        )),
        _ => None,
    }
}

fn parse(line: &str) -> Option<Packet> {
    let mut lex = Token::lexer(line);
    parse_tokens(&mut lex)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: Lines) -> String {
    input
        .map(parse)
        .group_by(|p| p.is_some())
        .into_iter()
        .filter_map(|(is_some, packets)| {
            if is_some {
                Some(packets.flatten().collect_vec())
            } else {
                None
            }
        })
        .enumerate()
        .filter_map(|(index, packets)| {
            if packets[0].cmp(&packets[1]) == Ordering::Less {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let dividers = vec![parse("[[2]]").unwrap(), parse("[[6]]").unwrap()];
    let mut packets = input
        .filter_map(parse)
        .chain(dividers.clone())
        .collect_vec();
    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter_map(|(index, p)| {
            if dividers.contains(&p) {
                Some(index + 1)
            } else {
                None
            }
        })
        .product::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "13");
        verify!(part2, input, "140");
    }
}
