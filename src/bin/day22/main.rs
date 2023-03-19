use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::{collections::HashMap, str::Lines};

struct Cube {
    size: isize,
    rules: HashMap<State, State>,
}

impl Cube {
    fn new(lines: Vec<&str>) -> Self {
        let size = lines[0].parse::<isize>().unwrap();
        let rules = lines[1..]
            .iter()
            .filter(|line| !line.starts_with("//"))
            .flat_map(|line| {
                let (a, b) = Self::parse_rules(line);
                [(a, b), (Self::flip_rule(&b), Self::flip_rule(&a))]
            })
            .collect();
        Self { size, rules }
    }

    fn parse_rules(line: &str) -> (State, State) {
        let (a, b) = line.split_once(" -> ").unwrap();
        (Self::parse_rule(a), Self::parse_rule(b))
    }

    fn parse_rule(rule: &str) -> State {
        let fields = rule.split_whitespace().collect_vec();
        State {
            row: fields[0].parse::<isize>().unwrap(),
            col: fields[1].parse::<isize>().unwrap(),
            dir: Facing::new(fields[2]).unwrap(),
        }
    }

    fn flip_rule(rule: &State) -> State {
        State {
            dir: rule.dir.flip(),
            ..*rule
        }
    }
}

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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn new(facing: &str) -> Option<Self> {
        match facing {
            "R" => Some(Facing::Right),
            "D" => Some(Facing::Down),
            "L" => Some(Facing::Left),
            "U" => Some(Facing::Up),
            _ => None,
        }
    }

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

    fn flip(self) -> Self {
        match self {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
        }
    }
}

fn wrap_flat(map: &Map, state: &State) -> State {
    match state.dir {
        Facing::Right => State {
            col: (0..state.col)
                .find(|&c| map.get(state.row, c).is_some())
                .unwrap() as isize,
            ..*state
        },
        Facing::Down => State {
            row: (0..state.row)
                .find(|&r| map.get(r, state.col).is_some())
                .unwrap() as isize,
            ..*state
        },
        Facing::Left => State {
            col: (state.col..map.tiles[state.row as usize].len() as isize)
                .rev()
                .find(|&c| map.get(state.row, c).is_some())
                .unwrap() as isize,
            ..*state
        },
        Facing::Up => State {
            row: (state.row..map.tiles.len() as isize)
                .rev()
                .find(|&r| map.get(r, state.col).is_some())
                .unwrap() as isize,
            ..*state
        },
    }
}

fn wrap_cube(map: &Map, state: &State) -> State {
    let cube = &map.cube;
    let cube_state_old = State {
        row: state.row / cube.size,
        col: state.col / cube.size,
        dir: state.dir,
    };
    let edge_dist = match state.dir {
        Facing::Right => state.row % cube.size,
        Facing::Down => cube.size - (state.col % cube.size) - 1,
        Facing::Left => cube.size - (state.row % cube.size) - 1,
        Facing::Up => state.col % cube.size,
    };
    let cube_state_new = cube
        .rules
        .get(&cube_state_old)
        .unwrap_or_else(|| panic!("no rule for {cube_state_old:?}"));
    let (row, col) = match cube_state_new.dir {
        Facing::Right => (edge_dist, 0),
        Facing::Down => (0, cube.size - edge_dist - 1),
        Facing::Left => (cube.size - edge_dist - 1, cube.size - 1),
        Facing::Up => (cube.size - 1, edge_dist),
    };
    State {
        row: cube_state_new.row * cube.size + row,
        col: cube_state_new.col * cube.size + col,
        dir: cube_state_new.dir,
    }
}

type WrapFn = fn(map: &Map, state: &State) -> State;

struct Map {
    tiles: Vec<Vec<Option<MapTile>>>,
    cube: Cube,
    wrap: WrapFn,
}

impl Map {
    fn new(input: Vec<&str>, cube: Cube, wrap: WrapFn) -> Self {
        let tiles = input
            .iter()
            .map(|row| row.chars().map(MapTile::new).collect_vec())
            .collect_vec();
        Map { tiles, cube, wrap }
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
            next = (self.wrap)(self, state);
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

fn parse_input(mut input: Lines, wrap: WrapFn) -> (Map, Vec<PathItem>) {
    let cube = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect_vec();
    let cube = Cube::new(cube);
    let map = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect_vec();
    let map = Map::new(map, cube, wrap);
    let path = input.next().unwrap();
    let path = parse_path(path);
    (map, path)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
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
    let (map, path) = parse_input(input, wrap_flat);
    let state = path
        .into_iter()
        .fold(map.start(), |state, path| state.follow(path, &map));
    state.password().to_string()
}

fn part2(input: Lines) -> String {
    let (map, path) = parse_input(input, wrap_cube);
    let state = path
        .into_iter()
        .fold(map.start(), |state, path| state.follow(path, &map));
    state.password().to_string()
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
        let (map, path) = parse_input(input.lines(), wrap_flat);
        assert_ne!(map.tiles.len(), 0);
        assert_ne!(path.len(), 0);
    }

    #[test]
    fn test_wrap_flat() {
        let input = include_str!("example.txt");
        let (map, _) = parse_input(input.lines(), wrap_flat);
        assert_eq!(
            wrap_flat(
                &map,
                &State {
                    row: 0,
                    col: 8,
                    dir: Facing::Left
                }
            ),
            State {
                row: 0,
                col: 11,
                dir: Facing::Left
            }
        );
        assert_eq!(
            wrap_flat(
                &map,
                &State {
                    row: 4,
                    col: 0,
                    dir: Facing::Left
                }
            ),
            State {
                row: 4,
                col: 11,
                dir: Facing::Left
            }
        );
    }

    #[test]
    fn test_wrap_cube() {
        let input = include_str!("example.txt");
        let (map, _) = parse_input(input.lines(), wrap_cube);
        assert_eq!(
            wrap_cube(
                &map,
                &State {
                    row: 0,
                    col: 8,
                    dir: Facing::Left
                }
            ),
            State {
                row: 4,
                col: 4,
                dir: Facing::Down
            }
        );
        assert_eq!(
            wrap_cube(
                &map,
                &State {
                    row: 4,
                    col: 0,
                    dir: Facing::Left
                }
            ),
            State {
                row: 11,
                col: 15,
                dir: Facing::Up
            }
        );
    }

    #[test]
    fn test_follow() {
        let input = include_str!("example.txt");
        let (map, _) = parse_input(input.lines(), wrap_flat);
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
    fn test_example_state_part1() {
        let input = include_str!("example.txt");
        let (map, path) = parse_input(input.lines(), wrap_flat);
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
        verify!(part2, input, "5031");
    }
}
