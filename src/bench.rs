use itertools::Itertools;
use std::fs;
use std::ops::Div;
use std::time::{Duration, Instant};

use crate::puzzles::{Part, Puzzle, TestFile};

pub struct BenchResult {
    pub name: String,
    pub elapsed: Duration,
}

pub fn bench(name: &str, func: impl Fn() -> String, expected: Option<String>) -> BenchResult {
    let result = func();

    if let Some(expected) = expected {
        assert!(result == expected, "Expected {:?} but got {:?}", expected, result);
    }

    // Warm the cache by pre-running a few more times
    func();
    func();
    func();
    func();
    func();

    // Benchmark
    let start = Instant::now();
    func();
    func();
    func();
    func();
    func();
    func();
    func();
    func();
    func();
    func();
    let elapsed = Instant::elapsed(&start).div(10);

    println!("{}: {} [{:?}]", name, result, elapsed);

    BenchResult {
        name: name.to_string(),
        elapsed,
    }
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("Failed to read file {}", path).as_str())
}

pub fn benchmark_puzzle_with_inputs(puzzle: Puzzle) -> Vec<BenchResult> {
    puzzle
        .parts
        .into_iter()
        .flat_map(|Part { name, func, tests }| {
            tests.into_iter().map(
                move |TestFile {
                          input_name,
                          input_path,
                          expected,
                      }| {
                    let input = read_file(input_path.as_str());

                    bench(
                        format!("{} {}", name, input_name).as_str(),
                        || func(&input),
                        expected,
                    )
                },
            )
        })
        .collect_vec()
}

pub fn make_part(
    year: u16,
    day: u8,
    part: u8,
    func: fn(&String) -> String,
    tests: Vec<TestFile>,
) -> Part {
    let name = format!("{} {:02} Part {}", year, day, part);

    Part {
        name: name.to_string(),
        func,
        tests,
    }
}

pub fn make_test_file(input_name: &str, input_path: &str, expected: Option<String>) -> TestFile {
    TestFile {
        input_name: input_name.to_string(),
        input_path: input_path.to_string(),
        expected,
    }
}

pub fn make_part_with_standard_tests(
    year: u16,
    day: u8,
    part: u8,
    func: fn(&String) -> String,
    expected: Option<&str>,
) -> Part {
    let path_base = format!("src/puzzles/year{}/day{:02}", year, day);

    let example = path_base.clone() + "/example.txt";
    let input = path_base.clone() + "/input.txt";

    let tests = vec![
        make_test_file("Example", example.as_str(), expected.map(|s| s.to_string())),
        make_test_file("Input", input.as_str(), None),
    ];

    make_part(year, day, part, func, tests)
}

pub fn clone_puzzle_with_input(puzzle: Puzzle, input: &str) -> Puzzle {
    Puzzle {
        year: puzzle.year,
        day: puzzle.day,
        parts: puzzle
            .parts
            .into_iter()
            .map(|part| Part {
                name: part.name,
                func: part.func,
                tests: vec![make_test_file("input", input, None)],
            })
            .collect_vec(),
    }
}

pub fn print_total_duration(results: Vec<BenchResult>) {
    let total = results.iter().map(|r| r.elapsed).sum::<Duration>();

    println!("Finished in {:?}", total);
}
