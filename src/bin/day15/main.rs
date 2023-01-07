use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::{ops::Range, str::Lines};

#[derive(Debug, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Pos) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq)]
struct Sensor {
    sensor: Pos,
    closest_beacon: Pos,
}

impl Sensor {
    fn parse(line: &str) -> Option<Self> {
        let (_, sx, sy, cx, cy) = regex_captures!(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
            line
        )?;
        let sx = sx.parse::<isize>().ok()?;
        let sy = sy.parse::<isize>().ok()?;
        let cx = cx.parse::<isize>().ok()?;
        let cy = cy.parse::<isize>().ok()?;
        Some(Self {
            sensor: Pos::new(sx, sy),
            closest_beacon: Pos::new(cx, cy),
        })
    }
    fn closest_beacon_distance(&self) -> isize {
        self.sensor.manhattan_distance(&self.closest_beacon)
    }
    fn scan_range_for_row(&self, row: isize) -> Option<Range<isize>> {
        let beacon_distance = self.closest_beacon_distance();
        let row_distance = (row - self.sensor.y).abs();
        let scan_radius = beacon_distance - row_distance;
        if scan_radius >= 0 {
            Some((self.sensor.x - scan_radius)..(self.sensor.x + scan_radius + 1))
        } else {
            None
        }
    }
}

fn combined_range(a: &Range<isize>, b: &Range<isize>) -> Option<Range<isize>> {
    if a.end < b.start || b.end < a.start {
        None
    } else {
        Some(a.start.min(b.start)..a.end.max(b.end))
    }
}

fn add_range(existing_ranges: Vec<Range<isize>>, mut new_range: Range<isize>) -> Vec<Range<isize>> {
    let mut result = existing_ranges
        .into_iter()
        .filter(|existing_range| {
            if let Some(combined) = combined_range(existing_range, &new_range) {
                println!("combining a={existing_range:?} b={new_range:?} c={combined:?}");
                new_range = combined;
                false
            } else {
                true
            }
        })
        .collect_vec();
    result.push(new_range);
    result
}

fn part1(mut input: Lines) -> String {
    let row = input.next().unwrap().parse::<isize>().unwrap();
    println!("row={row}");
    let sensors = input.into_iter().flat_map(Sensor::parse).collect_vec();
    let beacon_scan_size = sensors
        .iter()
        .flat_map(|s| {
            let r = s.scan_range_for_row(row);
            let d = s.closest_beacon_distance();
            let len = r.as_ref().map(|x| x.len()).unwrap_or(0);
            println!("sensor={s:?} dist={d} scan={r:?} size={len}");
            r
        })
        .fold(Vec::new(), add_range)
        .into_iter()
        .map(|r| r.len())
        .sum::<usize>();
    let beacons_in_row = sensors
        .into_iter()
        .filter_map(|s| Some(s.closest_beacon).filter(|b| b.y == row).map(|b| b.x))
        .unique()
        .count();
    println!("beacons_in_row={beacons_in_row}");
    (beacon_scan_size - beacons_in_row).to_string()
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
            Sensor::parse("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Some(Sensor {
                sensor: Pos::new(2, 18),
                closest_beacon: Pos::new(-2, 15)
            })
        )
    }

    #[test]
    fn combine() {
        assert_eq!(combined_range(&(0..2), &(2..3)), Some(0..3));
        assert_eq!(combined_range(&(2..3), &(0..2)), Some(0..3));
        assert_eq!(combined_range(&(0..1), &(2..3)), None);
        assert_eq!(combined_range(&(2..3), &(0..1)), None);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "26");
        verify!(part2, input, "0");
    }
}
