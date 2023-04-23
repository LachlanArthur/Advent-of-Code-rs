use crate::{bench::make_part_with_standard_tests, puzzles::Part};
use gcollections::ops::*;
use interval::{ops::*, Interval};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ConsoleLine {
    CdRoot,
    CdUp,
    CdTo(String),
    Ls,
    Dir(String),
    File { name: String, size: u64 },
}

#[derive(Debug, PartialEq, Eq)]
struct ConsoleLineParseError;

impl std::str::FromStr for ConsoleLine {
    type Err = ConsoleLineParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split_whitespace().collect_vec();

        match words.as_slice() {
            ["$", "cd", "/"] => Ok(ConsoleLine::CdRoot),
            ["$", "cd", ".."] => Ok(ConsoleLine::CdUp),
            ["$", "cd", dir] => Ok(ConsoleLine::CdTo(dir.to_string())),
            ["$", "ls"] => Ok(ConsoleLine::Ls),
            ["dir", dir] => Ok(ConsoleLine::Dir(dir.to_string())),
            [size, name] => {
                if let Ok(size) = size.parse::<u64>() {
                    Ok(ConsoleLine::File {
                        name: name.to_string(),
                        size,
                    })
                } else {
                    Err(ConsoleLineParseError)
                }
            }
            _ => Err(ConsoleLineParseError),
        }
    }
}

fn calculate_dir_sizes(input: &String) -> HashMap<Vec<std::string::String>, u64> {
    let mut dir_sizes: HashMap<Vec<String>, u64> = HashMap::new();
    let mut cwd: Vec<String> = vec!["/".to_string()];

    input
        .lines()
        .map(|s| {
            s.parse::<ConsoleLine>()
                .expect(format!("Failed to parse line {:?}", s).as_str())
        })
        .for_each(|line| match line {
            ConsoleLine::CdRoot => cwd = vec!["/".to_string()],
            ConsoleLine::CdUp => {
                cwd.pop().expect("Tried to cd up from root");
            }
            ConsoleLine::CdTo(dir) => cwd.push(dir),
            ConsoleLine::Ls => (),
            ConsoleLine::Dir(_) => (),
            ConsoleLine::File { name: _, size } => {
                // Update each ancestor path with the file size
                for i in 0..cwd.len() {
                    *dir_sizes.entry(cwd[0..=i].to_vec()).or_insert(0) += size;
                }
            }
        });

    dir_sizes
}

pub fn part1(input: &String) -> String {
    let dir_sizes = calculate_dir_sizes(input);

    let deletion_candidates = dir_sizes.values().cloned().filter(|size| size <= &100_000);

    deletion_candidates.sum::<u64>().to_string()
}

pub fn part2(input: &String) -> String {
    let mut dir_sizes = calculate_dir_sizes(input);

    let disk_size = 70_000_000;
    let need_size = 30_000_000;
    let dir_total = dir_sizes
        .remove(&vec!["/".to_string()])
        .expect("Root dir not found");
    let disk_free = disk_size - dir_total;
    let need_free = need_size - disk_free;

    let deletion_candidates = dir_sizes
        .values()
        .cloned()
        .filter(|size| size >= &need_free);

    deletion_candidates.min().expect("No deletion candidates found").to_string()
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("95437")),
        make_part_with_standard_tests(year, day, 2, part2, Some("24933642")),
    ]
}
