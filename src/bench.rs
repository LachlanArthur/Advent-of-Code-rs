use std::time::Instant;

pub fn bench<T: std::fmt::Debug + std::cmp::PartialEq>(
    name: &str,
    func: impl Fn() -> T,
    expected: Option<T>,
) {
    let start = Instant::now();
    let result = func();
    let elapsed = Instant::elapsed(&start);

    println!("{}: {:?} [{:?}]", name, result, elapsed);

    if let Some(expected) = expected {
        assert!(result == expected);
    }
}
