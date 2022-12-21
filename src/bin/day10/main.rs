use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

fn parse_program(input: Lines) -> Vec<i64> {
    input
        .flat_map(|line| {
            if line == "noop" {
                vec![0]
            } else {
                vec![0, line[5..].parse::<i64>().unwrap()]
            }
        })
        .collect::<Vec<_>>()
}

fn part1(input: Lines) -> String {
    let instructions = parse_program(input);
    let cycles = instructions.iter().enumerate().scan(1, |x, (cycle, dx)| {
        let result = *x * (cycle + 1) as i64;
        *x += dx;
        Some(result)
    });
    cycles.skip(19).step_by(40).take(6).sum::<i64>().to_string()
}

fn part2(input: Lines) -> String {
    let instructions = parse_program(input);
    let pixels = (0..40i64).cycle();
    let cycles = instructions.iter().zip(pixels).scan(1, |x, (dx, pixel)| {
        let draw = if pixel.abs_diff(*x) <= 1 { '#' } else { '.' };
        *x += dx;
        Some(draw)
    });
    cycles
        .chunks(40)
        .into_iter()
        .map(|c| c.collect::<String>())
        .take(6)
        .chain(std::iter::once(String::new()))
        .join("\n")
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
        verify!(part1, input, "13140");
        let expected = include_str!("part2_expected.txt");
        verify!(part2, input, expected);
    }
}
