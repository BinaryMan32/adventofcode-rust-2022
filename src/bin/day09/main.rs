use advent_of_code::{create_runner, named, Named, Runner};
use std::{collections::HashSet, str::Lines};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Offset;

    fn sub(self, rhs: Position) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add<Offset> for Position {
    type Output = Position;

    fn add(self, rhs: Offset) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, Copy)]
struct Offset {
    x: isize,
    y: isize,
}

impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Offset {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct State {
    knots: Vec<Position>,
}

impl State {
    fn new(num_knots: usize) -> Self {
        let knots = std::iter::repeat(Position::new(0, 0))
            .take(num_knots)
            .collect::<Vec<_>>();
        Self { knots }
    }

    fn tail(&self) -> &Position {
        self.knots.last().unwrap()
    }

    fn tail_move(tail_diff: Offset) -> Offset {
        if tail_diff.x.abs() >= 2 || tail_diff.y.abs() >= 2 {
            Offset::new(tail_diff.x.signum(), tail_diff.y.signum())
        } else {
            Offset::new(0, 0)
        }
    }

    fn next(&self, offset: &Offset) -> Self {
        let new_head = self.knots[0] + *offset;
        let knots = Some(new_head)
            .into_iter()
            .chain(
                self.knots
                    .iter()
                    .skip(1)
                    .copied()
                    .scan(new_head, |head, old_tail| {
                        let tail_diff = *head - old_tail;
                        let tail_move = Self::tail_move(tail_diff);
                        let new_tail = old_tail + tail_move;
                        *head = new_tail;
                        Some(new_tail)
                    }),
            )
            .collect::<Vec<_>>();
        Self { knots }
    }
}

struct Command {
    offset: Offset,
    count: usize,
}

impl Command {
    fn parse(line: &str) -> Option<Self> {
        let (direction, count) = line.split_once(' ')?;
        let count = count.parse::<usize>().ok()?;
        let offset = match direction {
            "R" => Some(Offset::new(1, 0)),
            "L" => Some(Offset::new(-1, 0)),
            "U" => Some(Offset::new(0, 1)),
            "D" => Some(Offset::new(0, -1)),
            _ => None,
        }?;
        Some(Self { offset, count })
    }
}

fn run(mut state: State, input: Lines) -> String {
    let mut tail_positions: HashSet<Position> = HashSet::new();
    for command in input.flat_map(Command::parse) {
        for _ in 0..command.count {
            state = state.next(&command.offset);
            tail_positions.insert(*state.tail());
        }
    }
    tail_positions.len().to_string()
}

fn part1(input: Lines) -> String {
    run(State::new(2), input)
}

fn part2(input: Lines) -> String {
    run(State::new(10), input)
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
        verify!(part1, input, "13");
        verify!(part2, input, "1");
    }
}
