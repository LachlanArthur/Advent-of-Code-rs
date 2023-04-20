use crate::bench::{make_part_with_standard_tests, Part};
use itertools::Itertools;

pub fn part1(input: &String) -> String {
    todo!();
    // return input
    //     .trim()
    //     .split("\n");
}

pub fn part2(input: &String) -> String {
    todo!();
    // return input
    //     .trim()
    //     .split("\n");
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("157")),
        make_part_with_standard_tests(year, day, 2, part2, Some("70")),
    ]
}
