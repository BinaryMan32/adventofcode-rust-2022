use advent_of_code::{create_runner, named, Named, Runner};
use std::{cmp::Ordering, ops::Range, str::Lines};

// consider using Interval from https://docs.rs/intervallum/1.4.0/interval/interval/index.html

fn parse_range(range: &str) -> Option<Range<u32>> {
    range
        .split_once('-')
        .and_then(|(a, b)| a.parse::<u32>().ok().zip(b.parse::<u32>().ok()))
        .map(|(a, b)| a..b + 1)
}

fn parse_ranges(line: &str) -> Option<(Range<u32>, Range<u32>)> {
    line.split_once(',')
        .and_then(|(a, b)| parse_range(a).zip(parse_range(b)))
}

fn either_contains(pair: &(Range<u32>, Range<u32>)) -> bool {
    let (a, b) = pair;
    let start_cmp = a.start.cmp(&b.start);
    let end_cmp = a.end.cmp(&b.end);
    start_cmp != end_cmp || start_cmp == Ordering::Equal
}

fn overlaps(pair: &(Range<u32>, Range<u32>)) -> bool {
    let (a, b) = pair;
    a.end > b.start && b.end > a.start
}

fn part1(input: Lines) -> String {
    input
        .filter_map(parse_ranges)
        .filter(either_contains)
        .count()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .filter_map(parse_ranges)
        .filter(overlaps)
        .count()
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
    fn test_contains() {
        assert!(either_contains(&(0..3, 0..1)));
        assert!(either_contains(&(0..1, 0..3)));
        assert!(either_contains(&(0..3, 2..3)));
        assert!(either_contains(&(2..3, 0..3)));
        assert!(either_contains(&(0..3, 0..3)));
        assert!(!either_contains(&(0..2, 1..3)));
        assert!(!either_contains(&(1..3, 0..2)));
        assert!(!either_contains(&(0..1, 1..2)));
        assert!(!either_contains(&(1..2, 0..1)));
    }

    #[test]
    fn test_overlaps() {
        assert!(overlaps(&(0..3, 0..1)));
        assert!(overlaps(&(0..1, 0..3)));
        assert!(overlaps(&(0..3, 2..3)));
        assert!(overlaps(&(2..3, 0..3)));
        assert!(overlaps(&(0..3, 0..3)));
        assert!(overlaps(&(0..2, 1..3)));
        assert!(overlaps(&(1..3, 0..2)));
        assert!(!overlaps(&(0..1, 1..2)));
        assert!(!overlaps(&(1..2, 0..1)));
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "2");
        verify!(part2, input, "4");
    }
}
