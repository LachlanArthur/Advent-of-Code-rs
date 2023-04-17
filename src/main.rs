use structopt::StructOpt;

pub mod puzzles;
pub mod bench;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code in Rust")]
pub struct Opt {
    year: u64,
    day: u64,
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt { year: 2022, day: 1 } => puzzles::year2022::day01::run(),

        _ => panic!("Puzzle not found"),
    }
}
