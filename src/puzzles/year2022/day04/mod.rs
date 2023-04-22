use crate::bench::make_part_with_standard_tests;
use crate::puzzles::Part;
use gcollections::ops::*;
use interval::ops::*;
use interval::Interval;
use itertools::Itertools;

fn parse(input: &String) -> impl Iterator<Item = (u32, u32, u32, u32)> + '_ {
    input
        .trim()
        .split("\n")
        .map(|line| sscanf::scanf!(line, "{u32}-{u32},{u32}-{u32}").unwrap())
}

pub fn part1(input: &String) -> String {
    parse(input)
        .filter(|(left_min, left_max, right_min, right_max)| {
            let left = Interval::new(*left_min, *left_max);
            let right = Interval::new(*right_min, *right_max);

            return left.is_proper_subset(&right) || right.is_proper_subset(&left);
        })
        .count()
        .to_string()
}

pub fn part2(input: &String) -> String {
    parse(input)
        .filter(|(left_min, left_max, right_min, right_max)| {
            let left = Interval::new(*left_min, *left_max);
            let right = Interval::new(*right_min, *right_max);

            !left.intersection(&right).is_empty()
        })
        .count()
        .to_string()
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("2")),
        make_part_with_standard_tests(year, day, 2, part2, Some("4")),
    ]
}
