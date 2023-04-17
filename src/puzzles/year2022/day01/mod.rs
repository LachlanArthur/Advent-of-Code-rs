use itertools::Itertools;
use std::fs;
use crate::bench::bench;

pub fn part1(input: &String) -> u64 {
    return input
        .trim()
        .split("\n\n")
        .map(|chunk| chunk.split("\n").map(|s| s.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap();
}

pub fn part2(input: &String) -> u64 {
    return input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|s| s.parse::<u64>().unwrap())
                .sum1::<u64>()
                .unwrap()
        })
        .sorted()
        .rev()
        .take(3)
        .sum1()
        .unwrap();
}

pub fn run() {
    let example = fs::read_to_string("src/puzzles/year2022/day01/example.txt")
        .expect("Should have been able to read the file");

    let input = fs::read_to_string("src/puzzles/year2022/day01/input.txt")
        .expect("Should have been able to read the file");

    bench("Part 1 Example", || part1(&example), Some(24000));
    bench("Part 1 Input", || part1(&input), None);

    bench("Part 2 Example", || part2(&example), Some(45000));
    bench("Part 2 Input", || part2(&input), None);
}
