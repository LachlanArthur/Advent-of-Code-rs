# Advent of Code (in Rust)

No peeking! (unless you've already done it)

Trying to solve Advent of Code puzzles in Rust, while learning Rust.

## Usage

```
cargo run day 2022 1
cargo run day 2022 1 some/other/input.txt
cargo run all
```
or, once built:
```
aoc day 2022 1
aoc day 2022 1 some/other/input.txt
aoc all
```

## Build

```
cargo build --release
```

## Dev

Install [cargo-watch](https://crates.io/crates/cargo-watch):
```
cargo install cargo-watch
```
And then
```
cargo watch -x 'run day 2022 01'
```

## Todo

- Codegen command to create a new puzzle stub
