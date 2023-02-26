use advent_of_code::{create_runner, named, Named, Runner};
use std::{
    collections::HashSet,
    ops::Add,
    str::{FromStr, Lines},
};

type ScalarType = isize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: ScalarType,
    y: ScalarType,
    z: ScalarType,
}

impl Pos {
    fn new(x: ScalarType, y: ScalarType, z: ScalarType) -> Pos {
        Pos { x, y, z }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePosError;

impl FromStr for Pos {
    type Err = ParsePosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(',').ok_or(ParsePosError)?;
        let (y, z) = yz.split_once(',').ok_or(ParsePosError)?;

        Ok(Self {
            x: x.parse::<ScalarType>().map_err(|_| ParsePosError)?,
            y: y.parse::<ScalarType>().map_err(|_| ParsePosError)?,
            z: z.parse::<ScalarType>().map_err(|_| ParsePosError)?,
        })
    }
}

fn part1(input: Lines) -> String {
    let coords = input
        .flat_map(|line| line.parse::<Pos>().ok())
        .collect::<HashSet<Pos>>();
    let sides = vec![
        Pos::new(-1, 0, 0),
        Pos::new(1, 0, 0),
        Pos::new(0, -1, 0),
        Pos::new(0, 1, 0),
        Pos::new(0, 0, -1),
        Pos::new(0, 0, 1),
    ];
    coords
        .iter()
        .copied()
        .map(|pos| {
            sides
                .iter()
                .copied()
                .filter(|&s| !coords.contains(&(pos + s)))
                .count()
        })
        .sum::<usize>()
        .to_string()
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
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "64");
        verify!(part2, input, "0");
    }
}
