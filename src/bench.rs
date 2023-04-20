use itertools::Itertools;
use std::fs;
use std::time::{Duration, Instant};

pub struct Part {
    pub name: String,
    pub func: fn(&String) -> String,
    pub tests: Vec<TestFile>,
}

#[derive(Clone)]
pub struct TestFile {
    pub input_name: String,
    pub input_path: String,
    pub expected: Option<String>,
}

pub struct BenchResult {
    pub name: String,
    pub elapsed: Duration,
}

pub fn bench(
    name: &str,
    func: impl Fn() -> String,
    expected: Option<String>,
    print: bool,
) -> BenchResult {
    let start = Instant::now();
    let result = func();
    let elapsed = Instant::elapsed(&start);

    if print {
        println!("{}: {} [{:?}]", name, result, elapsed);
    }

    if let Some(expected) = expected {
        assert!(result == expected);
    }

    BenchResult {
        name: name.to_string(),
        elapsed,
    }
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("Failed to read file {}", path).as_str())
}

pub fn benchmark_puzzle_with_inputs(parts: Vec<Part>, print: bool) -> Vec<BenchResult> {
    parts
        .into_iter()
        .flat_map(|Part { name, func, tests }| {
            tests.into_iter().map(
                move |TestFile {
                          input_name,
                          input_path,
                          expected,
                      }| {
                    bench(
                        format!("{} {}", name, input_name).as_str(),
                        || func(&read_file(input_path.as_str())),
                        expected,
                        print,
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
