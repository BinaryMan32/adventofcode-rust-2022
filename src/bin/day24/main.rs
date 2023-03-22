use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
#[cfg(test)]
use std::iter::once;
use std::str::Lines;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

fn manhattan_distance(a: &Pos, b: &Pos) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    #[cfg(test)]
    fn to_char(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

struct Input {
    start: Pos,
    end: Pos,
    end_from: Pos,
    min: Pos,
    max: Pos,
    blizzards: Vec<Vec<Option<Direction>>>,
}

impl Input {
    fn parse(input: Lines) -> Self {
        let input = input.collect_vec();
        let first = input.first().unwrap();
        let last = input.last().unwrap();
        let min = Pos { x: 1, y: 1 };
        let max = Pos {
            x: first.len() - 2,
            y: input.len() - 2,
        };
        let start = Pos {
            x: first.chars().position(|c| c == '.').unwrap(),
            y: 0,
        };
        let end = Pos {
            x: last.chars().position(|c| c == '.').unwrap(),
            y: input.len() - 1,
        };
        let end_from = Pos {
            x: end.x,
            y: end.y - 1,
        };
        let blizzards = input
            .iter()
            .skip(min.y)
            .take(max.y)
            .map(|row| {
                row.chars()
                    .skip(min.x)
                    .take(max.x)
                    .map(Direction::from_char)
                    .collect_vec()
            })
            .collect_vec();
        Self {
            start,
            end,
            end_from,
            min,
            max,
            blizzards,
        }
    }

    fn at_start_x(&self, x: usize, dx: isize) -> usize {
        (x as isize + dx - self.min.x as isize).rem_euclid(self.max.x as isize) as usize
    }

    fn at_start_y(&self, y: usize, dy: isize) -> usize {
        (y as isize + dy - self.min.y as isize).rem_euclid(self.max.y as isize) as usize
    }

    fn is_blizzard_at_dir(&self, pos: &Pos, t: usize, check_dir: Direction) -> bool {
        let dir = match check_dir {
            Direction::Up => self.blizzards[self.at_start_y(pos.y, t as isize)][pos.x - self.min.x],
            Direction::Down => {
                self.blizzards[self.at_start_y(pos.y, -(t as isize))][pos.x - self.min.x]
            }
            Direction::Left => {
                self.blizzards[pos.y - self.min.y][self.at_start_x(pos.x, t as isize)]
            }
            Direction::Right => {
                self.blizzards[pos.y - self.min.y][self.at_start_x(pos.x, -(t as isize))]
            }
        };
        dir == Some(check_dir)
    }

    fn is_blizzard_at(&self, pos: &Pos, t: usize) -> bool {
        DIRECTIONS
            .iter()
            .any(|&dir| self.is_blizzard_at_dir(pos, t, dir))
    }

    #[cfg(test)]
    fn render_blizzards_at(&self, pos: &Pos, t: usize) -> char {
        let blizzards = DIRECTIONS
            .iter()
            .flat_map(|&dir| {
                Some(dir)
                    .filter(|&dir| self.is_blizzard_at_dir(pos, t, dir))
                    .map(|dir| dir.to_char())
            })
            .collect_vec();
        match blizzards.len() {
            0 => '.',
            1 => blizzards[0],
            n => char::from_digit(n as u32, 4u32).unwrap(),
        }
    }

    #[cfg(test)]
    fn render(&self, t: usize) -> String {
        let first = (0..self.max.x + 2).map(|x| if x == self.start.x { '.' } else { '#' });
        let blizzards = (self.min.y..self.max.y + 1).flat_map(|y| {
            once('\n')
                .chain(once('#'))
                .chain(
                    (self.min.x..self.max.x + 1)
                        .map(move |x| self.render_blizzards_at(&Pos { x, y }, t)),
                )
                .chain(once('#'))
        });
        let last = (0..self.max.x + 2).map(|x| if x == self.end.x { '.' } else { '#' });
        first
            .chain(blizzards)
            .chain(once('\n'))
            .chain(last)
            .collect()
    }

    fn possible_moves(&self, p: &Pos, t: usize) -> Vec<Pos> {
        let y_range = p.y.saturating_sub(1).max(self.min.y)..(p.y + 1).min(self.max.y) + 1;
        let x_range = p.x.saturating_sub(1).max(self.min.x)..(p.x + 1).min(self.max.x) + 1;
        let possible = y_range.flat_map(|y| {
            x_range
                .clone()
                .filter_map(move |x| Some(Pos { x, y }).filter(|p| self.is_blizzard_at(p, t)))
        });
        let maybe_end = if p == &self.end_from {
            Some(self.end.clone())
        } else {
            None
        };
        possible.chain(maybe_end).collect_vec()
    }

    fn best_time_rec(&self, p: &Pos, t: usize, best_t: &mut usize) -> usize {
        if p == &self.end {
            *best_t = t.min(*best_t);
            t
        } else if t + manhattan_distance(p, &self.end) > *best_t {
            usize::MAX
        } else {
            let mut moves = self.possible_moves(p, t + 1);
            println!(
                "check p={p:?} t={t} best_t={best_t} dist={} moves={}",
                manhattan_distance(p, &self.end),
                moves.len()
            );
            moves.sort_unstable_by_key(|pos| manhattan_distance(pos, &self.end));
            moves
                .into_iter()
                .map(|pos| self.best_time_rec(&pos, t + 1, best_t))
                .min()
                .unwrap()
        }
    }

    fn best_time(&self) -> usize {
        let mut best_t = usize::MAX;
        self.best_time_rec(&self.start, 0, &mut best_t)
    }
}

fn part1(input: Lines) -> String {
    Input::parse(input).best_time().to_string()
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
    fn parse() {
        let input = include_str!("example.txt");
        let input = Input::parse(input.lines());
        assert_eq!(input.start, Pos { x: 1, y: 0 });
        assert_eq!(input.end, Pos { x: 6, y: 5 });
        assert_eq!(input.end_from, Pos { x: 6, y: 4 });
        assert_eq!(input.min, Pos { x: 1, y: 1 });
        assert_eq!(input.max, Pos { x: 6, y: 4 });
    }

    #[test]
    fn simple_example() {
        let rounds: Vec<String> = include_str!("simple_example.txt")
            .lines()
            .group_by(|line| line.is_empty())
            .into_iter()
            .filter_map(|(is_empty, group)| {
                if is_empty {
                    None
                } else {
                    Some(group.collect_vec().join("\n"))
                }
            })
            .collect();
        assert_eq!(rounds.len(), 6);
        let input = Input::parse(rounds[0].lines());
        for (t, expected) in rounds.into_iter().enumerate() {
            assert_eq!(input.render(t), expected, "t={t}");
        }
    }

    #[test]
    fn simple_example_rev() {
        let rounds: Vec<String> = include_str!("simple_example_rev.txt")
            .lines()
            .group_by(|line| line.is_empty())
            .into_iter()
            .filter_map(|(is_empty, group)| {
                if is_empty {
                    None
                } else {
                    Some(group.collect_vec().join("\n"))
                }
            })
            .collect();
        assert_eq!(rounds.len(), 6);
        let input = Input::parse(rounds[0].lines());
        for (t, expected) in rounds.into_iter().enumerate() {
            assert_eq!(input.render(t), expected, "t={t}");
        }
    }

    #[test]
    fn example_turns() {
        let rounds: Vec<String> = include_str!("example_turns.txt")
            .lines()
            .map(|line| line.replace('E', "."))
            .group_by(|line| line.starts_with('#'))
            .into_iter()
            .filter_map(|(is_map, group)| {
                if is_map {
                    Some(group.collect_vec().join("\n"))
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(rounds.len(), 19);
        let input = Input::parse(rounds[0].lines());
        for (t, expected) in rounds.into_iter().enumerate() {
            assert_eq!(input.render(t), expected);
        }
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "18");
        verify!(part2, input, "0");
    }
}
