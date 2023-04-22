use crate::bench::make_part_with_standard_tests;
use crate::puzzles::Part;
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
            let left_range = left_min..=left_max;
            let right_range = right_min..=right_max;

            (left_range.contains(&right_min) && left_range.contains(&right_max))
                || (right_range.contains(&left_min) && right_range.contains(&left_max))
        })
        .count()
        .to_string()
}

pub fn part2(input: &String) -> String {
    parse(input)
        .filter(|(left_min, left_max, right_min, right_max)| {
            let left_range = left_min..=left_max;
            let right_range = right_min..=right_max;

            left_range.contains(&right_min)
                || left_range.contains(&right_max)
                || right_range.contains(&left_min)
                || right_range.contains(&left_max)
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
