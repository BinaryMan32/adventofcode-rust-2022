use advent_of_code::{create_runner, named, Named, Runner};
use std::{
    collections::{HashSet, VecDeque},
    iter::repeat,
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

fn all_sides() -> Vec<Pos> {
    vec![
        Pos::new(-1, 0, 0),
        Pos::new(1, 0, 0),
        Pos::new(0, -1, 0),
        Pos::new(0, 1, 0),
        Pos::new(0, 0, -1),
        Pos::new(0, 0, 1),
    ]
}

fn part1(input: Lines) -> String {
    let coords = input
        .flat_map(|line| line.parse::<Pos>().ok())
        .collect::<HashSet<Pos>>();

    let sides = all_sides();
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

struct Matrix3<T: Copy> {
    size: Pos,
    elements: Vec<T>,
}

impl<T: Copy> Matrix3<T> {
    fn new(size: Pos, value: T) -> Self {
        let num_elements = size.x as usize * size.y as usize * size.z as usize;
        println!("Maxtrix3::new size={size:?} num_elements={num_elements}");
        let elements = Vec::from_iter(repeat(value).take(num_elements));
        Self { size, elements }
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.z >= 0
            && pos.x < self.size.x
            && pos.y < self.size.y
            && pos.z < self.size.z
    }

    fn offset(&self, pos: Pos) -> usize {
        ((pos.z as usize) * (self.size.y as usize) + (pos.y as usize)) * (self.size.x as usize)
            + (pos.x as usize)
    }
    fn get(&self, pos: Pos) -> Option<T> {
        if self.in_bounds(pos) {
            Some(self.elements[self.offset(pos)])
        } else {
            None
        }
    }
    fn set(&mut self, pos: Pos, value: T) {
        let offset = self.offset(pos);
        self.elements[offset] = value;
    }
}

fn mark_outside(coords: &HashSet<Pos>) -> Matrix3<bool> {
    let size = Pos::new(
        coords.iter().map(|c| c.x).max().unwrap_or(0) + 2,
        coords.iter().map(|c| c.y).max().unwrap_or(0) + 2,
        coords.iter().map(|c| c.z).max().unwrap_or(0) + 2,
    );
    let sides = all_sides();
    let mut outside = Matrix3::<bool>::new(size, false);
    let mut to_visit: VecDeque<Pos> = VecDeque::new();
    to_visit.push_back(Pos::new(0, 0, 0));
    while let Some(pos) = to_visit.pop_back() {
        if outside.get(pos) == Some(false) && !coords.contains(&pos) {
            outside.set(pos, true);
            for &offset in sides.iter() {
                let new_pos = pos + offset;
                if outside.get(new_pos) == Some(false) {
                    to_visit.push_back(new_pos)
                }
            }
        }
    }
    outside
}

fn part2(input: Lines) -> String {
    let coords = input
        .flat_map(|line| line.parse::<Pos>().ok())
        .collect::<HashSet<Pos>>();

    let outside = mark_outside(&coords);
    let sides = all_sides();

    coords
        .iter()
        .copied()
        .map(|pos| {
            sides
                .iter()
                .copied()
                .filter(|&s| outside.get(pos + s).unwrap_or(true))
                .count()
        })
        .sum::<usize>()
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
        verify!(part1, input, "64");
        verify!(part2, input, "58");
    }
}
