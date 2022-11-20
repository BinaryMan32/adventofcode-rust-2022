use advent_of_code::run;
use std::str::Lines;

fn part1(input: Lines) -> String {
    input.count().to_string()
}

fn part2(input: Lines) -> String {
    input.count().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    run!(part1, input);
    run!(part2, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "1");
        verify!(part2, input, "1");
    }
}
