use crate::{bench::make_part_with_standard_tests, puzzles::Part};
use gcollections::ops::*;
use interval::{ops::*, Interval};
use itertools::Itertools;

fn parse(input: &String) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let (stacks_str, moves_str) = input
        .split_once("\n\n")
        .expect("Input must contain the stacks and moves, separated by a blank line");

    let crates_lines = stacks_str
        .lines()
        // Exclude the last line (just contains the index of each crate stack)
        .collect_vec()
        .split_last()
        .unwrap()
        .1
        .into_iter()
        // Extract the chars
        .map(|line| line.chars())
        // Extract the chars at indices 4n+1
        .map(|line_chars| {
            line_chars
                .enumerate()
                .filter_map(|(i, c)| match i % 4 {
                    1 => Some(c),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    // Build the crate stacks from bottom to top

    let stack_count = crates_lines[0].len();
    let stack_size = crates_lines.len();

    let mut crates: Vec<Vec<char>> = vec![];

    for col_index in 0..stack_count {
        let mut stack = vec![];

        for row_index in 0..stack_size {
            let value = crates_lines[row_index][col_index];

            if !value.is_whitespace() {
                stack.push(value);
            }
        }

        stack.reverse();
        crates.push(stack);
    }

    let moves = moves_str
        .lines()
        .map(|line| sscanf::scanf!(line, "move {u32} from {u32} to {u32}").unwrap())
        .map(|(count, from, to)| (count, from - 1, to - 1))
        .collect_vec();

    return (crates, moves);
}

pub fn part1(input: &String) -> String {
    let (mut crates, moves) = parse(input);

    for (count, from, to) in moves {
        let from_stack = crates[from as usize].clone();
        let mut to_stack = crates[to as usize].clone();

        let split_at = from_stack.len() - count as usize;
        let mut moving = from_stack[split_at..].to_vec();

        moving.reverse();
        to_stack.append(&mut moving);

        crates[from as usize] = from_stack[0..split_at].to_vec();
        crates[to as usize] = to_stack;
    }

    return crates.iter().map(|stack| stack.last().unwrap()).join("");
}

pub fn part2(input: &String) -> String {
    let (mut crates, moves) = parse(input);

    for (count, from, to) in moves {
        let from_stack = crates[from as usize].clone();
        let mut to_stack = crates[to as usize].clone();

        let split_at = from_stack.len() - count as usize;
        let mut moving = from_stack[split_at..].to_vec();

        to_stack.append(&mut moving);

        crates[from as usize] = from_stack[0..split_at].to_vec();
        crates[to as usize] = to_stack;
    }

    return crates.iter().map(|stack| stack.last().unwrap()).join("");
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("CMZ")),
        make_part_with_standard_tests(year, day, 2, part2, Some("MCD")),
    ]
}
