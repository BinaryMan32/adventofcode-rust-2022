use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
    str::Lines,
};

#[derive(Debug, PartialEq)]
struct Valve {
    name: String,
    rate: usize,
    tunnels: HashMap<String, usize>,
}

impl Valve {
    fn parse(line: &str) -> Option<Self> {
        let (_, name, rate, tunnels) = regex_captures!(
            r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$",
            line
        )?;
        let name = name.to_owned();
        let rate = rate.parse::<usize>().ok()?;
        let tunnels = tunnels
            .split(", ")
            .map(|s| (s.to_owned(), 1))
            .collect::<HashMap<_, _>>();
        Some(Self {
            name,
            rate,
            tunnels,
        })
    }
}

struct Distance {
    indices: HashMap<String, usize>,
    distance: Vec<Vec<usize>>,
}

impl Distance {
    fn new(names: Vec<String>) -> Self {
        let n = names.len();
        let mut distance = repeat(repeat(usize::MAX).take(n).collect_vec())
            .take(n)
            .collect_vec();
        for (c, row) in distance.iter_mut().enumerate() {
            row[c] = 0;
        }
        Self {
            indices: names
                .into_iter()
                .enumerate()
                .map(|(i, n)| (n, i))
                .collect::<HashMap<_, _>>(),
            distance,
        }
    }
    fn add(&mut self, a: &str, b: &str, new_distance: usize) {
        if let Some(a) = self.indices.get(a) {
            if let Some(b) = self.indices.get(b) {
                self.set_min(*a, *b, new_distance);
            }
        }
    }
    fn compute(&mut self) {
        let n = self.indices.len();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    self.set_min(
                        i,
                        j,
                        self.distance[i][k].saturating_add(self.distance[k][j]),
                    )
                }
            }
        }
    }
    fn get_mut(&mut self, a: usize, b: usize) -> Option<&mut usize> {
        self.distance.get_mut(a).and_then(|bs| bs.get_mut(b))
    }
    fn set_min(&mut self, a: usize, b: usize, new_distance: usize) {
        if let Some(distance) = self.get_mut(a, b) {
            if new_distance < *distance {
                *distance = new_distance
            }
        }
    }
    fn dump(&self) {
        for (a, &i) in self.indices.iter() {
            for (b, &j) in self.indices.iter() {
                let d = self.distance[i][j];
                if d > 0 {
                    println!("{a} -> {b} = {d}")
                }
            }
        }
    }
    fn get(&self, a: &str, b: &str) -> usize {
        self.distance[self.indices[a]][self.indices[b]]
    }
}

fn distance_from_valves(valves: &HashMap<String, Valve>) -> Distance {
    let mut distance = Distance::new(valves.keys().cloned().collect_vec());
    for a in valves.values() {
        for (b, d) in a.tunnels.iter() {
            distance.add(&a.name, b, *d)
        }
    }
    distance.compute();
    distance
}

#[derive(Debug)]
struct State<'a> {
    time: usize,
    released: usize,
    valve: &'a str,
    unopened: HashSet<&'a str>,
}

impl<'a> State<'a> {
    fn new(time: usize, start: &'a str, valves: &'a HashMap<String, Valve>) -> Self {
        Self {
            time,
            released: 0,
            valve: start,
            unopened: valves
                .values()
                .filter(|v| v.rate > 0)
                .map(|v| &v.name[..])
                .collect::<HashSet<_>>(),
        }
    }

    fn maybe_open(&self, valve: &'a Valve, distance: usize) -> Option<Self> {
        if self.time > distance {
            let time = self.time - distance - 1;
            let mut unopened = self.unopened.clone();
            unopened.remove(&valve.name[..]);
            Some(Self {
                time,
                released: self.released + time * valve.rate,
                valve: &valve.name,
                unopened,
            })
        } else {
            None
        }
    }
}

fn remove_uninteresting_valve(valves: &mut HashMap<String, Valve>, keep: &str) -> Option<Valve> {
    valves
        .values()
        .find(|v| v.rate == 0 && v.name != keep)
        .map(|v| v.name.clone())
        .and_then(|name| valves.remove(&name))
}

fn simplify_valves(valves: &mut HashMap<String, Valve>, keep: &str) {
    while let Some(removed) = remove_uninteresting_valve(valves, keep) {
        for v in valves.values_mut() {
            if let Some(distance_to_removed) = v.tunnels.remove(&removed.name) {
                for (dest, d2) in removed.tunnels.iter() {
                    if dest != &v.name {
                        let distance = distance_to_removed + d2;
                        v.tunnels
                            .entry(dest.clone())
                            .and_modify(|d| *d = distance.min(*d))
                            .or_insert(distance);
                    }
                }
            }
        }
    }
}

fn find_most_pressure_released(
    state: State,
    valves: &HashMap<String, Valve>,
    distance: &Distance,
) -> usize {
    state
        .unopened
        .iter()
        .flat_map(|&unopened| {
            state
                .maybe_open(&valves[unopened], distance.get(state.valve, unopened))
                .map(|s| find_most_pressure_released(s, valves, distance))
        })
        .max()
        .unwrap_or(0)
        .max(state.released)
}

fn part1(input: Lines) -> String {
    let valve_vec = input.flat_map(Valve::parse).collect_vec();
    let mut valves = valve_vec
        .into_iter()
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    println!("original valves");
    valves.iter().for_each(|v| println!("{v:?}"));

    let start = "AA";
    simplify_valves(&mut valves, start);
    println!("simplified valves");
    valves.iter().for_each(|v| println!("{v:?}"));

    println!("distances");
    let distance = distance_from_valves(&valves);
    distance.dump();

    let start = State::new(30, start, &valves);
    find_most_pressure_released(start, &valves, &distance).to_string()
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
        assert_eq!(
            Valve::parse("Valve AA has flow rate=0; tunnels lead to valves DD, BB"),
            Some(Valve {
                name: "AA".to_owned(),
                rate: 0,
                tunnels: HashMap::from([("DD".to_owned(), 1), ("BB".to_owned(), 1)]),
            })
        );
        assert_eq!(
            Valve::parse("Valve HH has flow rate=22; tunnel leads to valve GG"),
            Some(Valve {
                name: "HH".to_owned(),
                rate: 22,
                tunnels: HashMap::from([("GG".to_owned(), 1)]),
            })
        );
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "1651");
        verify!(part2, input, "0");
    }
}
