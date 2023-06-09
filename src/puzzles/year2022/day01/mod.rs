use crate::bench::make_part_with_standard_tests;
use crate::puzzles::Part;
use itertools::Itertools;

pub fn part1(input: &String) -> String {
    input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|s| s.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(input: &String) -> String {
    input
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
        .sum1::<u64>()
        .unwrap()
        .to_string()
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("24000")),
        make_part_with_standard_tests(year, day, 2, part2, Some("45000")),
    ]
}
