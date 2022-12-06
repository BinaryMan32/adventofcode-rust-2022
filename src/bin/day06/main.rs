use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    str::Lines,
};

fn add(counts: &mut HashMap<char, usize>, c: char) -> bool {
    *counts
        .entry(c)
        .and_modify(|count| *count += 1)
        .or_insert(1usize)
        == 1
}

fn remove(counts: &mut HashMap<char, usize>, c: char) -> bool {
    if let Entry::Occupied(mut o) = counts.entry(c) {
        *o.get_mut() -= 1;
        if *o.get() == 0 {
            o.remove_entry();
            return true;
        }
    }
    false
}

fn check_distinct(line: &str, count: usize) -> Option<usize> {
    let mut buffer: VecDeque<char> = VecDeque::from_iter(line.chars().take(count - 1));
    let mut counter: HashMap<char, usize> = HashMap::new();
    let mut unique: usize = buffer
        .iter()
        .map(|c| add(&mut counter, *c))
        .filter(|c| *c)
        .count();
    line.chars()
        .skip(count - 1)
        .find_position(|c| {
            buffer.push_back(*c);
            if add(&mut counter, *c) {
                unique += 1
            }
            let distinct = unique == count;
            buffer.pop_front().into_iter().for_each(|c| {
                if remove(&mut counter, c) {
                    unique -= 1
                }
            });
            distinct
        })
        .map(|(i, _)| i + count)
}

fn part1(input: Lines) -> String {
    let buffer = input.into_iter().next().unwrap_or_default();
    check_distinct(buffer, 4).unwrap().to_string()
}

fn part2(input: Lines) -> String {
    let buffer = input.into_iter().next().unwrap_or_default();
    check_distinct(buffer, 14).unwrap().to_string()
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
        verify!(part1, input, "7");
        verify!(part2, input, "19");
    }
}
