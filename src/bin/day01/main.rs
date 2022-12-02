use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

fn part1(input: Lines) -> String {
    input
        .map(|line| line.parse::<i32>().ok())
        .group_by(|x| x.is_some())
        .into_iter()
        .filter_map(|(_, grp)| grp.while_some().sum1())
        .max()
        .unwrap_or(0)
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .map(|line| line.parse::<i32>().ok())
        .group_by(|x| x.is_some())
        .into_iter()
        .filter_map(|(_, grp)| grp.while_some().sum1())
        .map(|x: i32| -x)
        .k_smallest(3)
        .sum::<i32>()
        .abs()
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
        verify!(part1, input, "24000");
        verify!(part2, input, "45000");
    }
}
