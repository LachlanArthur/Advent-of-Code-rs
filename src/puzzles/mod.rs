pub mod year2022;

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

pub struct Puzzle {
    pub year: u16,
    pub day: u8,
    pub parts: Vec<Part>,
}

pub fn make_puzzle(year: u16, day: u8, parts: impl Fn(u16, u8) -> Vec<Part>) -> Puzzle {
    Puzzle {
        year,
        day,
        parts: parts(year, day),
    }
}
