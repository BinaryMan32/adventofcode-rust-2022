use advent_of_code::{create_runner, named, Named, Runner};
use lazy_regex::regex_captures;
use std::str::Lines;

struct Action {
    count: usize,
    from: usize,
    to: usize,
}

impl Action {
    fn parse(line: &str) -> Option<Self> {
        let (_, count, from, to) = regex_captures!(r"move (\d+) from (\d+) to (\d+)", line)?;
        Some(Self {
            count: count.parse::<usize>().ok()?,
            from: from.parse::<usize>().ok()? - 1,
            to: to.parse::<usize>().ok()? - 1,
        })
    }
}

struct Stacks {
    supplies: Vec<Vec<char>>,
}

impl Stacks {
    fn new() -> Self {
        Self {
            supplies: Vec::new(),
        }
    }

    fn read_line(&mut self, line: &str) {
        if line.contains('[') {
            let mut crates =
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .map(|c| if c == ' ' { None } else { Some(c) });
            self.supplies
                .iter_mut()
                .zip(crates.by_ref())
                .for_each(|(stack, c)| stack.extend(c.into_iter()));
            self.supplies
                .extend(crates.map(|c| c.into_iter().collect::<Vec<_>>()));
        }
    }

    fn flip_all(&mut self) {
        self.supplies = self
            .supplies
            .iter()
            .map(|s| s.iter().cloned().rev().collect())
            .collect()
    }

    fn do_single(&mut self, action: &Action) {
        let from_stack = &mut self.supplies[action.from];
        let removed = from_stack.split_off(from_stack.len() - action.count);
        self.supplies[action.to].extend(removed.into_iter().rev());
    }

    fn do_batch(&mut self, action: &Action) {
        let from_stack = &mut self.supplies[action.from];
        let removed = from_stack.split_off(from_stack.len() - action.count);
        self.supplies[action.to].extend(removed);
    }

    fn top_all(&self) -> String {
        self.supplies.iter().filter_map(|s| s.last()).collect()
    }
}

fn part1(input: Lines) -> String {
    let mut stacks = Stacks::new();
    let mut iter = input;

    iter.by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| stacks.read_line(line));
    stacks.flip_all();

    iter.filter_map(Action::parse)
        .for_each(|action| stacks.do_single(&action));
    stacks.top_all()
}

fn part2(input: Lines) -> String {
    let mut stacks = Stacks::new();
    let mut iter = input;

    iter.by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| stacks.read_line(line));
    stacks.flip_all();

    iter.filter_map(Action::parse)
        .for_each(|action| stacks.do_batch(&action));
    stacks.top_all()
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
        verify!(part1, input, "CMZ");
        verify!(part2, input, "MCD");
    }
}
