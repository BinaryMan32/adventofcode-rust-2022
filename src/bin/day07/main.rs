use advent_of_code::{create_runner, named, Named, Runner};
use std::{cmp::min, str::Lines};

#[derive(Debug)]
struct TotalSize {
    dir_size: u64,
    total_size: u64,
}

impl TotalSize {
    fn empty() -> Self {
        Self {
            dir_size: 0,
            total_size: 0,
        }
    }

    fn add_file_size(&mut self, size: u64) {
        self.dir_size += size;
    }

    fn add_dir(&mut self, other: Self) {
        self.dir_size += other.dir_size;
        self.total_size += other.total_size;
    }

    fn done(self, threshold: u64) -> Self {
        if self.dir_size <= threshold {
            Self {
                total_size: self.total_size + self.dir_size,
                ..self
            }
        } else {
            self
        }
    }
}

fn find_total_size_helper(input: &mut Lines, threshold: u64) -> TotalSize {
    let mut total_size = TotalSize::empty();
    while let Some(line) = input.next() {
        if line == "$ cd .." {
            break;
        } else if line.starts_with("$ cd ") {
            total_size.add_dir(find_total_size_helper(input, threshold));
        } else if !line.starts_with("dir ") {
            if let Some((size_raw, _)) = line.split_once(' ') {
                if let Ok(file_size) = size_raw.parse::<u64>() {
                    total_size.add_file_size(file_size);
                }
            }
        }
    }
    total_size.done(threshold)
}

fn find_total_size(mut input: Lines, threshold: u64) -> u64 {
    find_total_size_helper(&mut input, threshold).total_size
}

fn part1(input: Lines) -> String {
    find_total_size(input, 100000).to_string()
}

#[derive(Debug)]
struct SmallestSize {
    dir_size: u64,
    smallest_size: u64,
}

impl SmallestSize {
    fn empty() -> Self {
        Self {
            dir_size: 0,
            smallest_size: u64::MAX,
        }
    }

    fn add_file_size(&mut self, size: u64) {
        self.dir_size += size;
    }

    fn add_dir(&mut self, other: Self) {
        self.dir_size += other.dir_size;
        self.smallest_size = min(self.smallest_size, other.smallest_size);
    }

    fn done(self, threshold: u64) -> Self {
        if self.dir_size > threshold {
            Self {
                smallest_size: min(self.smallest_size, self.dir_size),
                ..self
            }
        } else {
            self
        }
    }
}

fn find_smallest_size_helper(input: &mut Lines, threshold: u64) -> SmallestSize {
    let mut total_size = SmallestSize::empty();
    while let Some(line) = input.next() {
        if line == "$ cd .." {
            break;
        } else if line.starts_with("$ cd ") {
            total_size.add_dir(find_smallest_size_helper(input, threshold));
        } else if !line.starts_with("dir ") {
            if let Some((size_raw, _)) = line.split_once(' ') {
                if let Ok(file_size) = size_raw.parse::<u64>() {
                    total_size.add_file_size(file_size);
                }
            }
        }
    }
    total_size.done(threshold)
}

fn find_smallest_size(mut input: Lines, threshold: u64) -> u64 {
    find_smallest_size_helper(&mut input, threshold).smallest_size
}

fn part2(input: Lines) -> String {
    let total_size = find_smallest_size_helper(&mut input.clone(), 0).dir_size;
    let unused_space = 70000000 - total_size;
    let threshold = 30000000 - unused_space;
    find_smallest_size(input, threshold).to_string()
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
        verify!(part1, input, "95437");
        verify!(part2, input, "24933642");
    }
}
