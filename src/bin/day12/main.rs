use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

#[derive(Clone)]
struct Pos {
    r: usize,
    c: usize,
}

impl Pos {
    fn new(r: usize, c: usize) -> Self {
        Self { r, c }
    }
}

struct HeightMap {
    elevation: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
}

impl HeightMap {
    fn parse(input: Lines) -> Self {
        let mut start = None;
        let mut end = None;
        let elevation = input
            .into_iter()
            .enumerate()
            .map(|(r, elevations)| {
                elevations
                    .chars()
                    .into_iter()
                    .enumerate()
                    .map(|(c, elevation)| match elevation {
                        'S' => {
                            start = Some(Pos::new(r, c));
                            0
                        }
                        'E' => {
                            end = Some(Pos::new(r, c));
                            25
                        }
                        _ => elevation as u8 - b'a',
                    })
                    .collect_vec()
            })
            .collect_vec();
        Self {
            elevation,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn elevation(&self, pos: &Pos) -> u8 {
        self.elevation[pos.r][pos.c]
    }

    fn possible_moves(&self, from: Pos) -> impl Iterator<Item = Pos> + '_ {
        let from_elevation = self.elevation(&from);
        [
            if from.c > 0 {
                Some(Pos::new(from.r, from.c - 1))
            } else {
                None
            },
            if from.c + 1 < self.elevation[0].len() {
                Some(Pos::new(from.r, from.c + 1))
            } else {
                None
            },
            if from.r > 0 {
                Some(Pos::new(from.r - 1, from.c))
            } else {
                None
            },
            if from.r + 1 < self.elevation.len() {
                Some(Pos::new(from.r + 1, from.c))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .flat_map(move |p| {
            Some(p).filter(|p| self.elevation(p).saturating_sub(from_elevation) <= 1)
        })
    }

    fn all_points_with_elevation(&self, query: u8) -> impl Iterator<Item = Pos> + '_ {
        self.elevation.iter().enumerate().flat_map(move |(r, row)| {
            row.iter().enumerate().flat_map(move |(c, elevation)| {
                if *elevation == query {
                    Some(Pos::new(r, c))
                } else {
                    None
                }
            })
        })
    }
}

struct Solver<'a> {
    map: &'a HeightMap,
    steps: Vec<Vec<u16>>,
}

impl<'a> Solver<'a> {
    fn new(map: &'a HeightMap) -> Self {
        let steps = map
            .elevation
            .iter()
            .map(|row| row.iter().map(|_| u16::MAX).collect_vec())
            .collect_vec();
        Self { map, steps }
    }

    fn find_shortest_len(&mut self) -> u16 {
        self.find_shortest_len_from(self.map.start.clone())
    }

    fn find_shortest_len_from(&mut self, pos: Pos) -> u16 {
        self.find_shortest_len_internal(pos, 0);
        self.steps[self.map.end.r][self.map.end.c]
    }

    fn find_shortest_len_internal(&mut self, pos: Pos, length: u16) {
        if length < self.steps[pos.r][pos.c] {
            self.steps[pos.r][pos.c] = length;
            let new_length = length + 1;
            for new_pos in self.map.possible_moves(pos) {
                self.find_shortest_len_internal(new_pos, new_length)
            }
        }
    }
}

fn part1(input: Lines) -> String {
    let map = HeightMap::parse(input);
    let mut solver = Solver::new(&map);
    solver.find_shortest_len().to_string()
}

fn part2(input: Lines) -> String {
    let map = HeightMap::parse(input);
    map.all_points_with_elevation(0)
        .map(|p| {
            let mut solver = Solver::new(&map);
            solver.find_shortest_len_from(p)
        })
        .min()
        .unwrap()
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
        verify!(part1, input, "31");
        verify!(part2, input, "29");
    }
}
