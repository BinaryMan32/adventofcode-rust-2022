use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use std::str::Lines;

fn find<T: PartialEq>(vec: &[T], num: T) -> usize {
    vec.iter().position(|x| *x == num).unwrap()
}

fn mix(vec: &mut Vec<usize>, num: usize, off: isize) {
    let old_pos = find::<usize>(vec, num);
    vec.remove(old_pos);
    let new_pos = (old_pos as isize + off).rem_euclid(vec.len() as isize) as usize;
    vec.insert(new_pos, num);
}

fn get<T: Copy>(numbers: &[T], mixed: &Vec<usize>, index: usize) -> T {
    numbers[mixed[index % mixed.len()]]
}

fn part1(input: Lines) -> String {
    let numbers = input
        .flat_map(|line| line.parse::<i16>().ok())
        .collect_vec();
    let mut mixed = (0..numbers.len()).collect_vec();
    for (id, &num) in numbers.iter().enumerate() {
        mix(&mut mixed, id, num as isize)
    }
    let zero = find(&numbers, 0);
    let zero = find(&mixed, zero);
    [1000, 2000, 3000]
        .map(|pos| get(&numbers, &mixed, zero + pos))
        .iter()
        .sum::<i16>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let decryption_key = 811589153;
    let numbers = input
        .flat_map(|line| line.parse::<isize>().ok())
        .map(|n| n * decryption_key)
        .collect_vec();
    let mut mixed = (0..numbers.len()).collect_vec();
    for _ in 0..10 {
        for (id, &num) in numbers.iter().enumerate() {
            mix(&mut mixed, id, num as isize)
        }
    }
    let zero = find(&numbers, 0);
    let zero = find(&mixed, zero);
    [1000, 2000, 3000]
        .map(|pos| get(&numbers, &mixed, zero + pos))
        .iter()
        .sum::<isize>()
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

    fn extract_all<T: Copy>(numbers: &[T], mixed: &[usize]) -> Vec<T> {
        mixed.iter().map(|&i| numbers[i]).collect()
    }

    #[test]
    fn test_mix() {
        let numbers: Vec<i16> = vec![1, 2, -3, 3, -2, 0, 4];
        let mut mixed = (0..numbers.len()).collect_vec();
        mix(&mut mixed, 0, 1);
        assert_eq!(extract_all(&numbers, &mixed), vec!(2, 1, -3, 3, -2, 0, 4));
        mix(&mut mixed, 1, 2);
        assert_eq!(extract_all(&numbers, &mixed), vec!(1, -3, 2, 3, -2, 0, 4));
        mix(&mut mixed, 2, -3);
        assert_eq!(extract_all(&numbers, &mixed), vec!(1, 2, 3, -2, -3, 0, 4));
        mix(&mut mixed, 3, 3);
        assert_eq!(extract_all(&numbers, &mixed), vec!(1, 2, -2, -3, 0, 3, 4));
        mix(&mut mixed, 4, -2);
        assert_eq!(extract_all(&numbers, &mixed), vec!(-2, 1, 2, -3, 0, 3, 4)); // -2 rotated from example
        mix(&mut mixed, 5, 0);
        assert_eq!(extract_all(&numbers, &mixed), vec!(-2, 1, 2, -3, 0, 3, 4)); // -2 rotated from example
        mix(&mut mixed, 6, 4);
        assert_eq!(extract_all(&numbers, &mixed), vec!(-2, 1, 2, -3, 4, 0, 3)); // -2 rotated from example
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "3");
        verify!(part2, input, "1623178306");
    }
}
