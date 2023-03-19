use advent_of_code::{create_runner, named, Named, Runner};
use itertools::{iterate, unfold, Either, Itertools};
use std::{collections::HashMap, collections::HashSet, ops::Add, str::Lines};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_elves(input: Lines) -> HashSet<Pos> {
    input
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| Some(x as isize).filter(|_| c == '#'))
                .map(move |x| Pos { y: y as isize, x })
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}

const DIRECTION_OFFSETS: [Pos; 8] = [
    Pos { x: 1, y: 0 },   // E
    Pos { x: 1, y: -1 },  // NE
    Pos { x: 0, y: -1 },  // N
    Pos { x: -1, y: -1 }, // NW
    Pos { x: -1, y: 0 },  // W
    Pos { x: -1, y: 1 },  // SW
    Pos { x: 0, y: 1 },   // S
    Pos { x: 1, y: 1 },   // SE
];

fn get_neighbors(p: &Pos, occupied: &HashSet<Pos>) -> [bool; 8] {
    DIRECTION_OFFSETS.map(|off| occupied.contains(&(*p + off)))
}

#[derive(Clone)]
struct Proposal {
    check: Vec<Direction>,
    propose: Direction,
}

#[derive(Default)]
struct Proposed {
    no_move: Vec<Pos>,
    move_to: HashMap<Pos, Vec<Pos>>,
}

#[derive(Clone)]
struct State {
    proposals: Vec<Proposal>,
    elves: HashSet<Pos>,
}

impl State {
    fn default_proposals() -> Vec<Proposal> {
        vec![
            Proposal {
                check: vec![Direction::N, Direction::NE, Direction::NW],
                propose: Direction::N,
            },
            Proposal {
                check: vec![Direction::S, Direction::SE, Direction::SW],
                propose: Direction::S,
            },
            Proposal {
                check: vec![Direction::W, Direction::NW, Direction::SW],
                propose: Direction::W,
            },
            Proposal {
                check: vec![Direction::E, Direction::NE, Direction::SE],
                propose: Direction::E,
            },
        ]
    }

    fn new(elves: HashSet<Pos>) -> Self {
        Self {
            proposals: Self::default_proposals(),
            elves,
        }
    }

    fn do_round(&self) -> Option<State> {
        let proposed_moves = self.propose_moves();
        self.do_moves(proposed_moves).map(|elves| {
            let proposals = Self::rotate_proposals(self.proposals.clone());
            Self { elves, proposals }
        })
    }

    fn propose_moves(&self) -> Proposed {
        let mut proposed: Proposed = Default::default();
        for &pos in self.elves.iter() {
            if let Some(dest) = self.propose_move(&pos) {
                proposed.move_to.entry(dest).or_default().push(pos);
            } else {
                proposed.no_move.push(pos);
            }
        }
        proposed
    }

    fn propose_move(&self, elf: &Pos) -> Option<Pos> {
        let neighbors = get_neighbors(elf, &self.elves);
        if neighbors.iter().any(|&x| x) {
            self.proposals.iter().find_map(|p| {
                Some(p.propose)
                    .filter(|_| p.check.iter().all(|&dir| !neighbors[dir as usize]))
                    .map(|dir| *elf + DIRECTION_OFFSETS[dir as usize])
            })
        } else {
            None
        }
    }

    fn do_moves(&self, proposed: Proposed) -> Option<HashSet<Pos>> {
        let (unique_dest, duplicate_src): (Vec<_>, Vec<_>) =
            proposed.move_to.into_iter().partition_map(|(to, from)| {
                if from.len() == 1 {
                    Either::Left(to)
                } else {
                    Either::Right(from)
                }
            });

        Some(unique_dest.into_iter().collect_vec())
            .filter(|moved| !moved.is_empty())
            .map(|moved| {
                proposed
                    .no_move
                    .into_iter()
                    .chain(moved)
                    .chain(duplicate_src.into_iter().flatten())
                    .collect()
            })
    }

    fn rotate_proposals(proposals: Vec<Proposal>) -> Vec<Proposal> {
        let first = proposals.first().expect("proposals not empty").clone();
        proposals
            .into_iter()
            .skip(1)
            .chain(std::iter::once(first))
            .collect_vec()
    }

    fn empty_tiles_in_smallest_rectangle(&self) -> usize {
        self.count_empty(self.rectangle_contains())
    }

    fn rectangle_contains(&self) -> (Pos, Pos) {
        let (xmin, xmax) = self
            .elves
            .iter()
            .map(|p| p.x)
            .minmax()
            .into_option()
            .unwrap();
        let (ymin, ymax) = self
            .elves
            .iter()
            .map(|p| p.y)
            .minmax()
            .into_option()
            .unwrap();
        (Pos { x: xmin, y: ymin }, Pos { x: xmax, y: ymax })
    }

    fn count_empty(&self, rectangle: (Pos, Pos)) -> usize {
        let (min, max) = rectangle;
        (min.y..max.y + 1)
            .flat_map(|y| (min.x..max.x + 1).map(move |x| Pos { x, y }))
            .filter(|pos| !self.elves.contains(pos))
            .count()
    }
}

fn part1(input: Lines) -> String {
    let elves = parse_elves(input);

    iterate(State::new(elves), |state| state.do_round().unwrap())
        .nth(10)
        .unwrap()
        .empty_tiles_in_smallest_rectangle()
        .to_string()
}

fn part2(input: Lines) -> String {
    let elves = parse_elves(input);
    let last_round_moved = unfold(State::new(elves), |state| {
        let next = state.do_round();
        if let Some(next_state) = next.clone() {
            *state = next_state
        }
        next
    })
    .enumerate()
    .last()
    .unwrap()
    .0 + 1;
    (last_round_moved + 1).to_string()
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
        verify!(part1, input, "110");
        verify!(part2, input, "20");
    }
}
