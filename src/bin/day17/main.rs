use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{
    fmt::Display,
    iter::{repeat, Cycle},
    str::Lines,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn push(&self, jet: Jet) -> Self {
        let x = match jet {
            Jet::Left => self.x - 1,
            Jet::Right => self.x + 1,
        };
        Self { x, y: self.y }
    }
    fn drop(&self) -> Option<Self> {
        Some(self.y).filter(|&y| y > 0).map(|y| Self {
            x: self.x,
            y: y - 1,
        })
    }
}

type Row = u8;

#[derive(Clone, PartialEq, Eq, Debug)]
struct RockShape {
    rows: Vec<Row>,
    size: Pos,
}

impl RockShape {
    fn parse_all(input: Lines) -> Vec<Self> {
        input
            .group_by(|line| line.is_empty())
            .into_iter()
            .flat_map(|(empty, lines)| {
                if empty {
                    None
                } else {
                    Some(Self::parse(lines))
                }
            })
            .collect_vec()
    }

    fn parse<'a>(input: impl Iterator<Item = &'a str>) -> Self {
        let mut input = input.peekable();
        let width = input.peek().map(|line| line.len()).unwrap_or_default();
        let mut rows = input.map(Self::parse_row).collect::<Vec<_>>();
        rows.reverse();
        let size = Pos::new(width, rows.len());
        Self { rows, size }
    }

    fn parse_row(line: &str) -> Row {
        line.chars()
            .enumerate()
            .map(|(i, c)| ((c == '#') as Row) << i)
            .sum()
    }
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn parse_all(line: &str) -> Vec<Self> {
        line.chars()
            .flat_map(|c| match c {
                '<' => Some(Self::Left),
                '>' => Some(Self::Right),
                _ => None,
            })
            .collect::<_>()
    }
}

struct Generator<T> {
    iter: Cycle<std::vec::IntoIter<T>>,
}

impl<T: Clone> Generator<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            iter: items.into_iter().cycle(),
        }
    }

    fn get(&mut self) -> T {
        self.iter.next().unwrap()
    }
}

struct Chamber {
    rows: Vec<Row>,
    width: usize,
}

impl Chamber {
    fn new(width: usize) -> Self {
        Self {
            rows: Vec::default(),
            width,
        }
    }
    fn height(&self) -> usize {
        self.rows
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, &row)| if row != 0 { Some(i + 1) } else { None })
            .unwrap_or_default()
    }
    fn push_jet(&self, shape: &RockShape, pos: Pos, jet: Jet) -> Pos {
        let new_pos = if match jet {
            Jet::Left => pos.x > 0,
            Jet::Right => pos.x + shape.size.x < self.width,
        } {
            Some(pos.push(jet))
        } else {
            None
        };
        new_pos
            .filter(|p| !self.intersects(shape, p))
            .unwrap_or(pos)
    }
    fn drop(&self, shape: &RockShape, pos: Pos) -> Option<Pos> {
        pos.drop().filter(|p| !self.intersects(shape, p))
    }
    fn intersects(&self, shape: &RockShape, pos: &Pos) -> bool {
        self.rows[pos.y..pos.y + shape.rows.len()]
            .iter()
            .zip(shape.rows.iter())
            .any(|(&chamber, &rock)| (rock << pos.x) & chamber != 0)
    }
    fn extend(&mut self, shape: &RockShape, pos: &Pos) {
        let required = pos.y + shape.rows.len();
        if self.rows.len() < required {
            self.rows.extend(repeat(0).take(required - self.rows.len()))
        }
    }
    fn add(&mut self, shape: &RockShape, pos: &Pos) {
        for (chamber, rock) in self.rows[pos.y..pos.y + shape.rows.len()]
            .iter_mut()
            .zip(shape.rows.iter())
        {
            *chamber |= rock << pos.x;
        }
    }
    #[allow(dead_code)]
    fn dump(&self) {
        for row in self.rows.iter().rev() {
            let out: String = (0..self.width)
                .map(|i| if (1 << i) & row != 0 { '#' } else { '.' })
                .collect();
            println!("|{out}|");
        }
        let bottom = str::repeat("-", self.width);
        println!("+{bottom}+");
    }
}

struct Simulation {
    rock_shapes: Generator<RockShape>,
    jet_pattern: Generator<Jet>,
    chamber: Chamber,
}

impl Simulation {
    fn new(rock_shapes: Vec<RockShape>, jet_pattern: Vec<Jet>, chamber: Chamber) -> Self {
        Self {
            rock_shapes: Generator::new(rock_shapes),
            jet_pattern: Generator::new(jet_pattern),
            chamber,
        }
    }

    fn drop_rocks(&mut self, count: usize) -> usize {
        for _ in 0..count {
            self.drop_rock()
        }
        self.chamber.height()
    }

    fn drop_rock(&mut self) {
        let shape = self.rock_shapes.get();
        let mut pos = self.appear_pos();
        self.chamber.extend(&shape, &pos);
        loop {
            pos = self.chamber.push_jet(&shape, pos, self.jet_pattern.get());
            if let Some(new_pos) = self.chamber.drop(&shape, pos) {
                pos = new_pos;
            } else {
                self.chamber.add(&shape, &pos);
                //self.chamber.dump();
                break;
            }
        }
    }

    /// Each rock appears so that its
    /// left edge is two units away from the left wall and
    /// its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
    fn appear_pos(&mut self) -> Pos {
        Pos::new(2, self.chamber.height() + 3)
    }
}

fn part1(mut input: Lines) -> String {
    let rock_shapes = RockShape::parse_all(include_str!("rocks.txt").lines());
    let jet_pattern = Jet::parse_all(input.next().unwrap());
    let chamber = Chamber::new(7);
    let mut simulation = Simulation::new(rock_shapes, jet_pattern, chamber);
    let num_rocks = 2022;
    simulation.drop_rocks(num_rocks).to_string()
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
    fn parse_rocks() {
        assert_eq!(
            RockShape::parse("#.\n.#".lines()),
            RockShape {
                rows: vec![2, 1],
                size: Pos::new(2, 2)
            }
        );
        assert_eq!(
            RockShape::parse_all(include_str!("rocks.txt").lines()).len(),
            5
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "3068");
        verify!(part2, input, "0");
    }
}
