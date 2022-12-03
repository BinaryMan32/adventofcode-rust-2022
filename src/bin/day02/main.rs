use advent_of_code::{create_runner, named, Named, Runner};
use std::str::Lines;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn from_char(choice: char) -> Option<Self> {
        match choice {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn from_char(choice: char) -> Option<Self> {
        match choice {
            'A' | 'X' => Some(Choice::Rock),
            'B' | 'Y' => Some(Choice::Paper),
            'C' | 'Z' => Some(Choice::Scissors),
            _ => None,
        }
    }

    fn desired(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            Outcome::Lose => match self {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::Draw => self,
        }
    }
}

struct Round {
    opponent: Choice,
    response: Choice,
}

impl Round {
    fn from_part1_str(round: &str) -> Option<Self> {
        let opponent = round.chars().next().and_then(Choice::from_char)?;
        let response = round.chars().nth(2).and_then(Choice::from_char)?;
        Some(Self { opponent, response })
    }

    fn from_part2_str(round: &str) -> Option<Self> {
        let opponent = round.chars().next().and_then(Choice::from_char)?;
        let outcome = round.chars().nth(2).and_then(Outcome::from_char)?;
        Some(Self {
            opponent,
            response: opponent.desired(outcome),
        })
    }

    fn outcome(&self) -> Outcome {
        if self.opponent == self.response {
            Outcome::Draw
        } else if self.opponent.desired(Outcome::Win) == self.response {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn score(&self) -> u32 {
        self.outcome() as u32 + self.response as u32
    }
}

fn part1(input: Lines) -> String {
    input
        .filter_map(Round::from_part1_str)
        .map(|round| round.score())
        .sum::<u32>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .filter_map(Round::from_part2_str)
        .map(|round| round.score())
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
        verify!(part1, input, "15");
        verify!(part2, input, "12");
    }
}
