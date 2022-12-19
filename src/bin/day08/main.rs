use advent_of_code::{create_runner, named, Named, Runner};
use std::str::Lines;

struct Forest {
    rows: usize,
    cols: usize,
    trees: Vec<Vec<i8>>,
}

impl Forest {
    fn parse(input: Lines) -> Self {
        let trees = input
            .into_iter()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            rows: trees.len(),
            cols: trees[0].len(),
            trees,
        }
    }

    fn is_visible(&self, ro: usize, co: usize) -> bool {
        let height = self.trees[ro][co];
        (0..ro).all(|r| self.trees[r][co] < height)
            || (ro + 1..self.rows).all(|r| self.trees[r][co] < height)
            || (0..co).all(|c| self.trees[ro][c] < height)
            || (co + 1..self.cols).all(|c| self.trees[ro][c] < height)
    }

    fn count_visible(&self) -> usize {
        let mut count = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.is_visible(r, c) {
                    count += 1
                }
            }
        }
        count
    }

    fn scan<I>(it: I) -> usize
    where
        I: Iterator<Item = bool>,
    {
        let mut count = 0;
        for v in it {
            count += 1;
            if v {
                break;
            }
        }
        count
    }

    fn scenic_score(&self, ro: usize, co: usize) -> usize {
        let height = self.trees[ro][co];
        [
            Self::scan((0..ro).rev().map(|r| self.trees[r][co] >= height)),
            Self::scan((ro + 1..self.rows).map(|r| self.trees[r][co] >= height)),
            Self::scan((0..co).rev().map(|c| self.trees[ro][c] >= height)),
            Self::scan((co + 1..self.cols).map(|c| self.trees[ro][c] >= height)),
        ]
        .into_iter()
        .product()
    }

    fn best_scenic_score(&self) -> usize {
        let mut best_score = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                best_score = best_score.max(self.scenic_score(r, c))
            }
        }
        best_score
    }
}

fn part1(input: Lines) -> String {
    Forest::parse(input).count_visible().to_string()
}

fn part2(input: Lines) -> String {
    Forest::parse(input).best_scenic_score().to_string()
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
        verify!(part1, input, "21");
        verify!(part2, input, "8");
    }
}
