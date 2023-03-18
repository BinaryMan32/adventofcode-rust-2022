use advent_of_code::{create_runner, named, Named, Runner};
use std::{collections::HashMap, str::Lines};

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn parse(text: &str) -> Option<Self> {
        match text {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None,
        }
    }
}

enum Job {
    Number(i64),
    Expression { a: String, b: String, op: Operation },
}

impl Job {
    fn parse(text: &str) -> Option<Self> {
        text.parse::<i64>().ok().map(Job::Number).or_else(|| {
            let parts: Vec<&str> = text.split_whitespace().collect();
            Operation::parse(parts[1]).map(|op| Job::Expression {
                a: parts[0].to_owned(),
                b: parts[2].to_owned(),
                op,
            })
        })
    }
}

fn parse_monkeys(input: Lines) -> HashMap<String, Job> {
    input
        .map(|line| {
            let (name, job) = line.split_once(": ").unwrap();
            let job = Job::parse(job).unwrap();
            (name.to_owned(), job)
        })
        .collect()
}

fn math_op(op: Operation, a: i64, b: i64) -> i64 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => a / b,
    }
}

fn yell(monkeys: &HashMap<String, Job>, monkey: &str) -> i64 {
    let job = &monkeys[monkey];
    match job {
        Job::Number(n) => *n,
        Job::Expression { a, b, op } => math_op(*op, yell(monkeys, a), yell(monkeys, b)),
    }
}

fn part1(input: Lines) -> String {
    let monkeys = parse_monkeys(input);
    yell(&monkeys, "root").to_string()
}

fn is_human(name: &str) -> bool {
    name == "humn"
}

fn maybe_yell(monkeys: &HashMap<String, Job>, monkey: &str) -> Option<i64> {
    if is_human(monkey) {
        None
    } else {
        let job = &monkeys[monkey];
        match job {
            Job::Number(n) => Some(*n),
            Job::Expression { a, b, op } => maybe_yell(monkeys, a)
                .zip(maybe_yell(monkeys, b))
                .map(|(a, b)| math_op(*op, a, b)),
        }
    }
}

fn solve(monkeys: &HashMap<String, Job>, monkey: &str, want: i64) -> i64 {
    if is_human(monkey) {
        want
    } else {
        let job = &monkeys[monkey];
        match job {
            Job::Number(_) => panic!("impossible to reach a number"),
            Job::Expression {
                a: a_name,
                b: b_name,
                op,
            } => {
                let a = maybe_yell(monkeys, a_name);
                let b = maybe_yell(monkeys, b_name);
                if let Some(a) = a {
                    solve(
                        monkeys,
                        b_name,
                        match op {
                            Operation::Add => want - a,      // a + b = want
                            Operation::Subtract => a - want, // a - b = want
                            Operation::Multiply => want / a, // a * b = want
                            Operation::Divide => a / want,   // a / b = want
                        },
                    )
                } else if let Some(b) = b {
                    solve(
                        monkeys,
                        a_name,
                        match op {
                            Operation::Add => want - b,      // a + b = want
                            Operation::Subtract => want + b, // a - b = want
                            Operation::Multiply => want / b, // a * b = want
                            Operation::Divide => want * b,   // a / b = want
                        },
                    )
                } else {
                    panic!("both sides can't be human")
                }
            }
        }
    }
}

fn part2(input: Lines) -> String {
    let monkeys = parse_monkeys(input);
    let (a_name, b_name) = match &monkeys["root"] {
        Job::Expression { a, b, op: _ } => Some((a, b)),
        _ => None,
    }
    .expect("root monkey must be an expression");
    let a = maybe_yell(&monkeys, a_name);
    let b = maybe_yell(&monkeys, b_name);
    if let Some(a) = a {
        solve(&monkeys, b_name, a)
    } else if let Some(b) = b {
        solve(&monkeys, a_name, b)
    } else {
        panic!()
    }
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
        verify!(part1, input, "152");
        verify!(part2, input, "301");
    }
}
