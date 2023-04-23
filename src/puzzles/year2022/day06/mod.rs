use crate::{bench::make_part_with_standard_tests, puzzles::Part};
use gcollections::ops::*;
use interval::{ops::*, Interval};
use itertools::Itertools;
use std::collections::HashSet;
use std::slice::Windows;

fn find_unique_window(input: &String, window_size: usize) -> (usize, Vec<char>) {
    let chars = input.trim().chars().collect_vec();

    let (index, window) = chars
        .windows(window_size)
        .find_position(|chars| HashSet::<_>::from_iter(chars.iter()).len() == window_size)
        .expect("Unique window not found");

    (index + 0, window.to_vec())
}

pub fn part1(input: &String) -> String {
    let window_size = 4;
    let (index, _) = find_unique_window(input, window_size);

    (index + window_size).to_string()
}

pub fn part2(input: &String) -> String {
    let window_size = 14;
    let (index, _) = find_unique_window(input, window_size);

    (index + window_size).to_string()
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("7")),
        make_part_with_standard_tests(year, day, 2, part2, Some("19")),
    ]
}
