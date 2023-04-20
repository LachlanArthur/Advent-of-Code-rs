use std::{collections::HashMap, path::PathBuf, time::Duration, vec};

use bench::{benchmark_puzzle_with_inputs, BenchResult, Part, TestFile};
use itertools::Itertools;
use structopt::StructOpt;

pub mod bench;
pub mod puzzles;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code in Rust")]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Day(DayCommand),
    All,
}

#[derive(Debug, StructOpt)]
struct DayCommand {
    year: u16,
    day: u8,
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Day(cmd) => {
            match cmd.input {
                Some(input) => {
                    let puzzles = register_puzzles();

                    let puzzle = puzzles
                        .get(&(cmd.year, cmd.day))
                        .expect("Puzzle not found")
                        .to_owned();

                    let tests = vec![TestFile {
                        input_name: "input".to_string(),
                        input_path: input.to_str().unwrap().to_string(),
                        expected: None,
                    }];

                    let mut parts: Vec<Part> = vec![];

                    for part in puzzle.parts.iter() {
                        parts.push(Part {
                            name: part.name.clone(),
                            func: part.func,
                            tests: tests.clone(),
                        });
                    }

                    // Create a new puzzle with the input file as the only test
                    let new_puzzle = Puzzle {
                        year: puzzle.year,
                        day: puzzle.day,
                        parts,
                    };

                    let results = run_puzzle(new_puzzle, true);

                    print_total_duration(results);
                }
                None => {
                    print_total_duration(run_puzzles(register_puzzles(), Some((cmd.year, cmd.day))));
                }
            };
        }

        Command::All => {
            print_total_duration(run_puzzles(register_puzzles(), None));
        }
    };
}

struct Puzzle {
    year: u16,
    day: u8,
    parts: Vec<Part>,
}

fn register_puzzles() -> HashMap<(u16, u8), Puzzle> {
    let mut puzzles: HashMap<(u16, u8), Puzzle> = HashMap::new();

    register_puzzle(&mut puzzles, 2022, 1, puzzles::year2022::day01::parts);
    register_puzzle(&mut puzzles, 2022, 2, puzzles::year2022::day02::parts);
    register_puzzle(&mut puzzles, 2022, 3, puzzles::year2022::day03::parts);

    return puzzles;
}

fn register_puzzle(
    list: &mut HashMap<(u16, u8), Puzzle>,
    year: u16,
    day: u8,
    parts: impl Fn(u16, u8) -> Vec<Part>,
) {
    list.insert(
        (year, day),
        Puzzle {
            year,
            day,
            parts: parts(year, day),
        },
    );
}

fn run_puzzle(puzzle: Puzzle, print: bool) -> Vec<BenchResult> {
    return benchmark_puzzle_with_inputs(puzzle.parts, print);
}

fn run_puzzles(
    puzzles: HashMap<(u16, u8), Puzzle>,
    matching: Option<(u16, u8)>,
) -> Vec<BenchResult> {
    return puzzles
        .into_iter()
        .filter(|((year, day), _)| matching.is_none() || Some((*year, *day)) == matching)
        .map(|(_, puzzle)| run_puzzle(puzzle, true))
        .flatten()
        .collect_vec();
}

fn print_total_duration(results: Vec<BenchResult>) {
    let total = results.iter().map(|r| r.elapsed).sum::<Duration>();

    println!("Finished in {:?}", total);
}
