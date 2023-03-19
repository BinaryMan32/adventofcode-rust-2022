use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum MapTile {
    Open,
    Solid,
}

impl MapTile {
    fn new(c: char) -> Option<Self> {
        match c {
            '.' => Some(MapTile::Open),
            '#' => Some(MapTile::Solid),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn turn(self, dir: TurnDir) -> Self {
        match dir {
            TurnDir::Left => match self {
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Up => Facing::Left,
            },
            TurnDir::Right => match self {
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Up => Facing::Right,
            },
        }
    }
}

struct Map {
    tiles: Vec<Vec<Option<MapTile>>>,
}

impl Map {
    fn new(input: Vec<&str>) -> Self {
        let tiles = input
            .iter()
            .map(|row| row.chars().map(MapTile::new).collect_vec())
            .collect_vec();
        Map { tiles }
    }

    fn get(&self, row: isize, col: isize) -> Option<MapTile> {
        if row >= 0 && col >= 0 {
            self.tiles
                .get(row as usize)
                .and_then(|row| row.get(col as usize))
                .and_then(|&x| x)
        } else {
            None
        }
    }

    fn get_tile(&self, state: &State) -> Option<MapTile> {
        self.get(state.row, state.col)
    }

    fn wrap(&self, state: State) -> State {
        match state.dir {
            Facing::Right => State {
                col: (0..state.col)
                    .find(|&c| self.get(state.row, c).is_some())
                    .unwrap() as isize,
                ..state
            },
            Facing::Down => State {
                row: (0..state.row)
                    .find(|&r| self.get(r, state.col).is_some())
                    .unwrap() as isize,
                ..state
            },
            Facing::Left => State {
                col: (state.col..self.tiles[state.row as usize].len() as isize)
                    .rev()
                    .find(|&c| self.get(state.row, c).is_some())
                    .unwrap() as isize,
                ..state
            },
            Facing::Up => State {
                row: (state.row..self.tiles.len() as isize)
                    .rev()
                    .find(|&r| self.get(r, state.col).is_some())
                    .unwrap() as isize,
                ..state
            },
        }
    }

    fn start(&self) -> State {
        let col = self.tiles[0]
            .iter()
            .position(|c| c == &Some(MapTile::Open))
            .unwrap() as isize;
        State::new(col)
    }

    fn forward(&self, state: &State) -> State {
        let mut next = state.forward();
        let next_tile = if let Some(tile) = self.get_tile(&next) {
            tile
        } else {
            next = self.wrap(next);
            self.get_tile(&next).unwrap()
        };
        match next_tile {
            MapTile::Open => next,
            MapTile::Solid => *state,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum TurnDir {
    Right, // clockwise
    Left,  // counterclockwise
}

impl TurnDir {
    fn new(input: &str) -> Option<Self> {
        match input {
            "R" => Some(Self::Right),
            "L" => Some(Self::Left),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum PathItem {
    Move(usize),
    Turn(TurnDir),
}

impl PathItem {
    fn new(input: &str) -> Self {
        TurnDir::new(input)
            .map(PathItem::Turn)
            .unwrap_or_else(|| PathItem::Move(input.parse::<usize>().unwrap()))
    }
}

fn parse_path(input: &str) -> Vec<PathItem> {
    lazy_regex::regex!(r"R|L|([0-9]+)")
        .find_iter(input)
        .map(|m| PathItem::new(m.as_str()))
        .collect_vec()
}

fn parse_input(mut input: Lines) -> (Map, Vec<PathItem>) {
    let map = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect_vec();
    let map = Map::new(map);
    let path = input.next().unwrap();
    let path = parse_path(path);
    (map, path)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    row: isize,
    col: isize,
    dir: Facing,
}

impl State {
    fn new(col: isize) -> Self {
        Self {
            row: 0,
            col,
            dir: Facing::Right,
        }
    }

    fn forward(&self) -> Self {
        match self.dir {
            Facing::Right => Self {
                col: self.col + 1,
                ..*self
            },
            Facing::Down => Self {
                row: self.row + 1,
                ..*self
            },
            Facing::Left => Self {
                col: self.col - 1,
                ..*self
            },
            Facing::Up => Self {
                row: self.row - 1,
                ..*self
            },
        }
    }

    fn turn(&self, dir: TurnDir) -> Self {
        Self {
            dir: self.dir.turn(dir),
            ..*self
        }
    }

    fn follow(&self, path: PathItem, map: &Map) -> Self {
        match path {
            PathItem::Move(n) => itertools::iterate(*self, |&s| map.forward(&s))
                .nth(n)
                .unwrap(),
            PathItem::Turn(dir) => self.turn(dir),
        }
    }

    fn password(&self) -> isize {
        1000 * (self.row + 1) + 4 * (self.col + 1) + (self.dir as isize)
    }
}

fn part1(input: Lines) -> String {
    let (map, path) = parse_input(input);
    let state = path
        .into_iter()
        .fold(map.start(), |state, path| state.follow(path, &map));
    state.password().to_string()
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
    fn test_parse_path() {
        assert_eq!(
            parse_path("5L7"),
            vec![
                PathItem::Move(5),
                PathItem::Turn(TurnDir::Left),
                PathItem::Move(7)
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("example.txt");
        let (map, path) = parse_input(input.lines());
        assert_ne!(map.tiles.len(), 0);
        assert_ne!(path.len(), 0);
    }

    #[test]
    fn test_wrap() {
        let input = include_str!("example.txt");
        let (map, _) = parse_input(input.lines());
        assert_eq!(
            map.wrap(State {
                row: 0,
                col: 7,
                dir: Facing::Left
            }),
            State {
                row: 0,
                col: 11,
                dir: Facing::Left
            }
        );
        assert_eq!(
            map.wrap(State {
                row: 4,
                col: -1,
                dir: Facing::Left
            }),
            State {
                row: 4,
                col: 11,
                dir: Facing::Left
            }
        );
    }

    #[test]
    fn test_follow() {
        let input = include_str!("example.txt");
        let (map, _) = parse_input(input.lines());
        assert_eq!(
            State {
                row: 4,
                col: 0,
                dir: Facing::Left
            }
            .follow(PathItem::Move(1), &map),
            State {
                row: 4,
                col: 0,
                dir: Facing::Left
            }
        );
    }

    #[test]
    fn test_example_state() {
        let input = include_str!("example.txt");
        let (map, path) = parse_input(input.lines());
        let state = path
            .into_iter()
            .fold(map.start(), |state, path| state.follow(path, &map));
        assert_eq!(
            state,
            State {
                row: 5,
                col: 7,
                dir: Facing::Right
            }
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "6032");
        verify!(part2, input, "0");
    }
}
