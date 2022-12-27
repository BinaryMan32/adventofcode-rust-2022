use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::fmt::Display;
use std::ops::Range;
use std::str::Lines;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    fn parse(input: &str) -> Option<Self> {
        let (x, y) = input.split_once(',')?;
        let x = x.parse::<usize>().ok()?;
        let y = y.parse::<usize>().ok()?;
        Some(Self { x, y })
    }
}

fn parse_line(input: &str) -> Vec<Pos> {
    input
        .split(" -> ")
        .into_iter()
        .flat_map(Pos::parse)
        .collect_vec()
}

struct PosBounds {
    min: Pos,
    max: Pos,
}

impl PosBounds {
    fn from_segments(segments: &[Vec<Pos>]) -> Self {
        let (min_x, max_x) = segments
            .iter()
            .flat_map(|s| s.iter().map(|p| p.x))
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = segments
            .iter()
            .flat_map(|s| s.iter().map(|p| p.y))
            .minmax()
            .into_option()
            .unwrap();
        Self {
            min: Pos::new(min_x, min_y),
            max: Pos::new(max_x, max_y),
        }
    }
}

struct Cave {
    x_offset: usize,
    cells: Vec<Vec<bool>>,
}

impl Cave {
    fn new(x_offset: usize, x_size: usize, y_size: usize) -> Self {
        let cells = std::iter::repeat(std::iter::repeat(false).take(x_size).collect_vec())
            .take(y_size)
            .collect_vec();
        Self { x_offset, cells }
    }
    fn from_segments(segments: Vec<Vec<Pos>>) -> Self {
        let bounds = PosBounds::from_segments(&segments);
        let mut cave = Cave::new(
            bounds.min.x - 1,
            bounds.max.x + 3 - bounds.min.x,
            bounds.max.y + 1,
        );
        for points in segments {
            for (a, b) in points.iter().zip(points.iter().skip(1)) {
                cave.mark_segment(a, b)
            }
        }
        cave
    }

    fn get(&self, p: &Pos) -> bool {
        self.cells[p.y][p.x - self.x_offset]
    }

    fn set(&mut self, p: &Pos) {
        self.cells[p.y][p.x - self.x_offset] = true
    }

    fn range(a: usize, b: usize) -> Range<usize> {
        if b < a {
            b..a + 1
        } else {
            a..b + 1
        }
    }

    fn mark_segment(&mut self, a: &Pos, b: &Pos) {
        if a.x == b.x {
            Self::range(a.y, b.y)
                .map(move |y| Pos { x: a.x, y })
                .for_each(|p| self.set(&p))
        } else if a.y == b.y {
            Self::range(a.x, b.x)
                .map(move |x| Pos { x, y: a.y })
                .for_each(|p| self.set(&p))
        } else {
            panic!("Line from {a} to {b} not horizontal or vertical")
        }
    }

    fn drop_next(&self, p: &Pos) -> Option<Pos> {
        [p.x, p.x - 1, p.x + 1]
            .map(|x| Pos { x, y: p.y + 1 })
            .into_iter()
            .find(|p| !self.get(p))
    }

    fn drop_one(&self, start: &Pos) -> Option<Pos> {
        let mut p = start.clone();
        if self.get(start) {
            return None;
        };
        while let Some(next_p) = self.drop_next(&p) {
            if next_p.y >= self.cells.len() - 1 {
                return None;
            };
            p = next_p;
        }
        Some(p)
    }

    fn drop_all(&mut self, p: &Pos) -> usize {
        std::iter::repeat(p)
            .map(|p| {
                let last = self.drop_one(p);
                last.iter().for_each(|np| self.set(np));
                last
            })
            .enumerate()
            .find_map(|(index, p)| if p.is_none() { Some(index) } else { None })
            .unwrap()
    }
}

fn part1(input: Lines) -> String {
    let segments = input.map(parse_line).collect_vec();
    Cave::from_segments(segments)
        .drop_all(&Pos::new(500, 0))
        .to_string()
}

fn part2(input: Lines) -> String {
    let mut segments = input.map(parse_line).collect_vec();
    let bounds = PosBounds::from_segments(&segments);
    let start = Pos::new(500, 0);
    let bottom = bounds.max.y + 2;
    segments.push(vec![
        Pos::new(start.x - bottom, bottom),
        Pos::new(start.x + bottom, bottom),
    ]);
    Cave::from_segments(segments)
        .drop_all(&Pos::new(500, 0))
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
    fn segment_parse() {
        assert_eq!(
            parse_line("1,2 -> 3,4"),
            vec!(Pos::new(1, 2), Pos::new(3, 4))
        );
    }

    #[test]
    fn segment_horizontal() {
        let mut segments = vec![vec![Pos::new(1, 1), Pos::new(3, 1)]];
        for _ in 0..2 {
            let cave = Cave::from_segments(segments.clone());
            assert!(cave.get(&Pos::new(1, 1)));
            assert!(cave.get(&Pos::new(2, 1)));
            assert!(cave.get(&Pos::new(3, 1)));
            segments.reverse();
        }
    }

    #[test]
    fn segment_vertical() {
        let mut segments = vec![vec![Pos::new(1, 1), Pos::new(1, 3)]];
        for _ in 0..2 {
            let cave = Cave::from_segments(segments.clone());
            assert!(cave.get(&Pos::new(1, 1)));
            assert!(cave.get(&Pos::new(1, 2)));
            assert!(cave.get(&Pos::new(1, 3)));
            segments.reverse();
        }
    }

    #[test]
    fn drop_next() {
        let segments = vec![vec![Pos::new(1, 2), Pos::new(3, 2)]];
        let cave = Cave::from_segments(segments);
        assert_eq!(
            cave.drop_next(&Pos::new(2, 0)),
            Some(Pos::new(2, 1)),
            "nothing underneath falls straight down"
        );
        assert_eq!(
            cave.drop_next(&Pos::new(2, 1)),
            None,
            "all 3 spaces underneath stops"
        );
        assert_eq!(
            cave.drop_next(&Pos::new(1, 1)),
            Some(Pos::new(0, 2)),
            "falls left when unoccupied"
        );
        assert_eq!(
            cave.drop_next(&Pos::new(3, 1)),
            Some(Pos::new(4, 2)),
            "falls right when unoccupied"
        );
    }

    #[test]
    fn drop_one() {
        let segments = vec![vec![Pos::new(1, 2), Pos::new(3, 2)]];
        let mut cave = Cave::from_segments(segments);
        assert_eq!(
            cave.drop_one(&Pos::new(2, 0)),
            Some(Pos::new(2, 1)),
            "stops at segment"
        );
        assert_eq!(cave.drop_one(&Pos::new(1, 0)), None, "falls left forever");
        assert_eq!(cave.drop_one(&Pos::new(3, 0)), None, "falls right forever");
        cave.set(&Pos::new(2, 0));
        assert_eq!(cave.drop_one(&Pos::new(2, 0)), None, "start blocked");
    }

    #[test]
    fn drop_all() {
        let segments = vec![vec![Pos::new(1, 2), Pos::new(3, 2)]];
        let mut cave = Cave::from_segments(segments);
        assert_eq!(
            cave.drop_all(&Pos::new(2, 0)),
            1,
            "one drop stays in center of short segment"
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "24");
        verify!(part2, input, "93");
    }
}
