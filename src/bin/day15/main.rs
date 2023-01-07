use advent_of_code::{create_runner, named, Named, Runner};
use lazy_regex::regex_captures;
use std::str::Lines;

#[derive(Debug, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Sensor {
    sensor: Pos,
    closest_beacon: Pos,
}

impl Sensor {
    fn parse(line: &str) -> Option<Self> {
        let (_, sx, sy, cx, cy) = regex_captures!(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
            line
        )?;
        let sx = sx.parse::<isize>().ok()?;
        let sy = sy.parse::<isize>().ok()?;
        let cx = cx.parse::<isize>().ok()?;
        let cy = cy.parse::<isize>().ok()?;
        Some(Self {
            sensor: Pos::new(sx, sy),
            closest_beacon: Pos::new(cx, cy),
        })
    }
}

fn part1(input: Lines) -> String {
    input.take(0).count().to_string()
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
    fn parse() {
        assert_eq!(
            Sensor::parse("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Some(Sensor {
                sensor: Pos::new(2, 18),
                closest_beacon: Pos::new(-2, 15)
            })
        )
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "0");
        verify!(part2, input, "0");
    }
}
