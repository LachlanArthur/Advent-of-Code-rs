use clap::StructOpt;
use puzzles::{make_puzzle, Puzzle};
use std::{path::PathBuf, vec};

pub mod bench;
pub mod command;
pub mod puzzles;
pub mod test;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code in Rust")]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Run a single puzzle
    Day(DayCommand),
    /// Run all puzzles
    All,
}

#[derive(Debug, StructOpt)]
pub struct DayCommand {
    year: u16,
    day: u8,
    /// Custom input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Day(cmd) => {
            match cmd.input {
                Some(input) => command::run_puzzle_with_custom_input(
                    cmd.year,
                    cmd.day,
                    input.to_str().unwrap(),
                ),
                None => command::run_puzzle(cmd.year, cmd.day),
            };
        }

        Command::All => command::run_all_puzzles(),
    };
}

pub fn register_puzzles() -> std::vec::IntoIter<Puzzle> {
    vec![
        make_puzzle(2022, 1, puzzles::year2022::day01::parts),
        make_puzzle(2022, 2, puzzles::year2022::day02::parts),
        make_puzzle(2022, 3, puzzles::year2022::day03::parts),
        make_puzzle(2022, 4, puzzles::year2022::day04::parts),
        make_puzzle(2022, 5, puzzles::year2022::day05::parts),
        make_puzzle(2022, 6, puzzles::year2022::day06::parts),
        make_puzzle(2022, 7, puzzles::year2022::day07::parts),
        make_puzzle(2022, 8, puzzles::year2022::day08::parts),
    ]
    .into_iter()
}
