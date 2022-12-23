use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::str::Lines;

type WorryLevel = u64;

struct MonkeyBehavior {
    starting_items: Vec<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    divisible_by: WorryLevel,
    throw_to: Box<dyn Fn(WorryLevel) -> usize>,
}

impl MonkeyBehavior {
    fn parse(input: String) -> Option<Self> {
        let (_, starting_items, op, arg, divisible_by, to_true, to_false) = regex_captures!(
            r"Monkey \d+:
  Starting items: ([0-9, ]+)
  Operation: new = old (\+|\*) (old|\d+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
            &input
        )?;
        let starting_items = starting_items
            .split(", ")
            .map(|x| x.parse::<WorryLevel>().unwrap())
            .collect_vec();
        let operation: Box<dyn Fn(WorryLevel) -> WorryLevel> =
            match (op, arg.parse::<WorryLevel>().ok()) {
                ("+", None) => Box::new(|x| x + x),
                ("+", Some(y)) => Box::new(move |x| x + y),
                ("*", None) => Box::new(|x| x * x),
                ("*", Some(y)) => Box::new(move |x| x * y),
                _ => panic!("unexpected op={op}"),
            };
        let divisible_by = divisible_by.parse::<WorryLevel>().unwrap();
        let to_true = to_true.parse::<usize>().unwrap();
        let to_false = to_false.parse::<usize>().unwrap();
        let throw_to = Box::new(move |x| {
            if x % divisible_by == 0 {
                to_true
            } else {
                to_false
            }
        });
        Some(Self {
            starting_items,
            operation,
            divisible_by,
            throw_to,
        })
    }

    fn parse_all(input: Lines) -> Vec<Self> {
        input
            .group_by(|line| line.is_empty())
            .into_iter()
            .flat_map(|(empty, mut lines)| if empty { None } else { Some(lines.join("\n")) })
            .flat_map(Self::parse)
            .collect_vec()
    }
}

struct Thrown {
    worry: WorryLevel,
    destination: usize,
}

struct Monkey {
    behavior: MonkeyBehavior,
    items: Vec<WorryLevel>,
    inspected: usize,
}

type ManageWorry = dyn Fn(WorryLevel) -> WorryLevel;
impl Monkey {
    fn new(behavior: MonkeyBehavior) -> Self {
        let items = behavior.starting_items.clone();
        Self {
            behavior,
            items,
            inspected: 0,
        }
    }

    fn inspect_items(&mut self, manage_worry: &ManageWorry) -> Vec<Thrown> {
        self.inspected += self.items.len();
        let thrown = self
            .items
            .iter()
            .map(|item| {
                let worry = manage_worry((self.behavior.operation)(*item));
                let destination = (self.behavior.throw_to)(worry);
                Thrown { worry, destination }
            })
            .collect_vec();
        self.items.clear();
        thrown
    }
}

struct KeepAway {
    monkeys: Vec<Monkey>,
}

impl KeepAway {
    fn new(behavior: Vec<MonkeyBehavior>) -> Self {
        let monkeys = behavior.into_iter().map(Monkey::new).collect_vec();
        Self { monkeys }
    }

    fn do_round(&mut self, manage_worry: &ManageWorry) {
        for monkey in 0..self.monkeys.len() {
            for thrown in self.monkeys[monkey].inspect_items(manage_worry) {
                self.monkeys[thrown.destination].items.push(thrown.worry);
            }
        }
    }

    fn inspected(&self) -> Vec<usize> {
        self.monkeys.iter().map(|m| m.inspected).collect_vec()
    }

    fn monkey_business(&self) -> usize {
        let mut inspected = self.inspected();
        inspected.sort();
        inspected.reverse();
        println!("inspected={inspected:?}");
        inspected.iter().take(2).product::<usize>()
    }
}

fn part1(input: Lines) -> String {
    let behavior = MonkeyBehavior::parse_all(input);
    for (index, monkey) in behavior.iter().enumerate() {
        let items = &monkey.starting_items;
        println!("Monkey {index}: {items:?}");
    }
    let manage_worry = |x| x / 3;
    let mut keep_away = KeepAway::new(behavior);
    for _ in 0..20 {
        keep_away.do_round(&manage_worry)
    }
    keep_away.monkey_business().to_string()
}

fn part2(input: Lines) -> String {
    let behavior = MonkeyBehavior::parse_all(input);
    for (index, monkey) in behavior.iter().enumerate() {
        let items = &monkey.starting_items;
        println!("Monkey {index}: {items:?}");
    }
    let divisble_by_product = behavior
        .iter()
        .map(|b| b.divisible_by)
        .product::<WorryLevel>();
    println!("divisble_by_product={divisble_by_product}");
    let manage_worry = move |x| x % divisble_by_product;
    let mut keep_away = KeepAway::new(behavior);
    for _ in 0..10000 {
        keep_away.do_round(&manage_worry)
    }
    keep_away.monkey_business().to_string()
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
        verify!(part1, input, "10605");
        verify!(part2, input, "2713310158");
    }
}
