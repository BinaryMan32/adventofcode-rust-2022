use advent_of_code::{create_runner, named, Named, Runner};
use std::{collections::HashMap, str::Lines};

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

fn yell(monkeys: &HashMap<String, Job>, monkey: &str) -> i64 {
    let job = &monkeys[monkey];
    match job {
        Job::Number(n) => *n,
        Job::Expression { a, b, op } => {
            let a = yell(monkeys, a);
            let b = yell(monkeys, b);
            match op {
                Operation::Add => a + b,
                Operation::Subtract => a - b,
                Operation::Multiply => a * b,
                Operation::Divide => a / b,
            }
        }
    }
}

fn part1(input: Lines) -> String {
    let monkeys = parse_monkeys(input);
    yell(&monkeys, "root").to_string()
}

fn part2(input: Lines) -> String {
    input.take(0).count().to_string()
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
        verify!(part2, input, "0");
    }
}
