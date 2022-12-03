use advent_of_code::{create_runner, named, Named, Runner};
use itertools::{Chunk, Itertools};
use std::{collections::HashSet, str::Lines};

fn find_common_item_compartments(line: &str) -> Option<char> {
    let chunks = line
        .chars()
        .chunks(line.len() / 2)
        .into_iter()
        .map(|c| c.collect::<HashSet<_>>())
        .take(2)
        .collect::<Vec<_>>();
    chunks[0].intersection(&chunks[1]).next().copied()
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn part1(input: Lines) -> String {
    input
        .filter_map(find_common_item_compartments)
        .map(priority)
        .sum::<u32>()
        .to_string()
}

fn find_common_item_group(group: Chunk<Lines>) -> Option<char> {
    group
        .into_iter()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).into_iter().copied().collect())
        .and_then(|c| c.into_iter().next())
}

fn part2(input: Lines) -> String {
    input
        .chunks(3)
        .into_iter()
        .filter_map(find_common_item_group)
        .map(priority)
        .sum::<u32>()
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
        verify!(part1, input, "157");
        verify!(part2, input, "70");
    }
}
