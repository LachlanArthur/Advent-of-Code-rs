use itertools::Itertools;
use std::fs;
use crate::bench::bench;

pub fn part1(input: &String) -> u64 {
    todo!();
    // return input
    //     .trim()
    //     .split("\n");
}

pub fn part2(input: &String) -> u64 {
    todo!();
    // return input
    //     .trim()
    //     .split("\n");
}

pub fn run() {
    let example = fs::read_to_string("src/puzzles/year2022/dayX/example.txt")
        .expect("Should have been able to read the file");

    let input = fs::read_to_string("src/puzzles/year2022/dayX/input.txt")
        .expect("Should have been able to read the file");

    bench("Part 1 Example", || part1(&example), Some(1));
    bench("Part 1 Input", || part1(&input), None);

    bench("Part 2 Example", || part2(&example), Some(1));
    bench("Part 2 Input", || part2(&input), None);
}
