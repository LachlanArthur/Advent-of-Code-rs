use itertools::Itertools;

use crate::{
    bench::{benchmark_puzzle_with_inputs, clone_puzzle_with_input, print_total_duration},
    puzzles::Puzzle,
    register_puzzles,
};

pub fn run_puzzle(year: u16, day: u8) {
    let puzzle = single_puzzle(year, day);
    print_total_duration(benchmark_puzzle_with_inputs(puzzle))
}

pub fn run_puzzle_with_custom_input(year: u16, day: u8, input: &str) {
    let puzzle = clone_puzzle_with_input(single_puzzle(year, day), input);
    print_total_duration(benchmark_puzzle_with_inputs(puzzle));
}

pub fn run_all_puzzles() {
    print_total_duration(
        register_puzzles()
            .flat_map(benchmark_puzzle_with_inputs)
            .collect_vec(),
    )
}

fn single_puzzle(year: u16, day: u8) -> Puzzle {
    register_puzzles()
        .find(|puzzle| puzzle.year == year && puzzle.day == day)
        .expect("Puzzle not found")
}
