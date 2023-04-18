use crate::bench::bench;
use itertools::Itertools;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, ToPrimitive)]
enum RpsChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RpsChoice {
    fn value(&self) -> u64 {
        ToPrimitive::to_u64(self).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RpsChoiceParseError;

impl FromStr for RpsChoice {
    type Err = RpsChoiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RpsChoice::Rock),
            "B" => Ok(RpsChoice::Paper),
            "C" => Ok(RpsChoice::Scissors),
            "X" => Ok(RpsChoice::Rock),
            "Y" => Ok(RpsChoice::Paper),
            "Z" => Ok(RpsChoice::Scissors),
            _ => Err(RpsChoiceParseError),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, ToPrimitive)]
enum RpsOutcome {
    Lose = 0,
    Tie = 3,
    Win = 6,
}

impl RpsOutcome {
    fn value(&self) -> u64 {
        ToPrimitive::to_u64(self).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RpsOutcomeParseError;

impl FromStr for RpsOutcome {
    type Err = RpsOutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RpsOutcome::Lose),
            "Y" => Ok(RpsOutcome::Tie),
            "Z" => Ok(RpsOutcome::Win),
            _ => Err(RpsOutcomeParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RpsGame(RpsChoice, RpsChoice);

impl RpsGame {
    fn score(&self) -> u64 {
        &self.1.value() + &self.outcome().value()
    }

    fn outcome(&self) -> RpsOutcome {
        RpsOutcome::from(self)
    }
}

impl From<&RpsGame> for RpsOutcome {
    fn from(value: &RpsGame) -> Self {
        match value {
            RpsGame(RpsChoice::Rock, RpsChoice::Rock) => RpsOutcome::Tie,
            RpsGame(RpsChoice::Rock, RpsChoice::Paper) => RpsOutcome::Win,
            RpsGame(RpsChoice::Rock, RpsChoice::Scissors) => RpsOutcome::Lose,
            RpsGame(RpsChoice::Paper, RpsChoice::Rock) => RpsOutcome::Lose,
            RpsGame(RpsChoice::Paper, RpsChoice::Paper) => RpsOutcome::Tie,
            RpsGame(RpsChoice::Paper, RpsChoice::Scissors) => RpsOutcome::Win,
            RpsGame(RpsChoice::Scissors, RpsChoice::Rock) => RpsOutcome::Win,
            RpsGame(RpsChoice::Scissors, RpsChoice::Paper) => RpsOutcome::Lose,
            RpsGame(RpsChoice::Scissors, RpsChoice::Scissors) => RpsOutcome::Tie,
        }
    }
}

// Given their choice and a desired outcome, what should we choose?
impl From<(RpsChoice, RpsOutcome)> for RpsChoice {
    fn from(value: (RpsChoice, RpsOutcome)) -> Self {
        match value {
            (RpsChoice::Rock, RpsOutcome::Tie) => RpsChoice::Rock,
            (RpsChoice::Rock, RpsOutcome::Win) => RpsChoice::Paper,
            (RpsChoice::Rock, RpsOutcome::Lose) => RpsChoice::Scissors,
            (RpsChoice::Paper, RpsOutcome::Lose) => RpsChoice::Rock,
            (RpsChoice::Paper, RpsOutcome::Tie) => RpsChoice::Paper,
            (RpsChoice::Paper, RpsOutcome::Win) => RpsChoice::Scissors,
            (RpsChoice::Scissors, RpsOutcome::Win) => RpsChoice::Rock,
            (RpsChoice::Scissors, RpsOutcome::Lose) => RpsChoice::Paper,
            (RpsChoice::Scissors, RpsOutcome::Tie) => RpsChoice::Scissors,
        }
    }
}

pub fn part1(input: &String) -> u64 {
    let games = input.trim().split("\n").map(|s| {
        let game = s.split_whitespace().collect_vec();

        RpsGame(
            game[0].parse::<RpsChoice>().unwrap(),
            game[1].parse::<RpsChoice>().unwrap(),
        )
    });

    let scores = games.map(|game| game.score());

    return scores.sum();
}

pub fn part2(input: &String) -> u64 {
    let games = input.trim().split("\n").map(|s| {
        let game = s.split_whitespace().collect_vec();

        let their_choice = game[0].parse::<RpsChoice>().unwrap();
        let outcome = game[1].parse::<RpsOutcome>().unwrap();

        let our_choice = RpsChoice::from((their_choice, outcome));

        RpsGame(their_choice, our_choice)
    });

    let scores = games.map(|game| game.score());

    return scores.sum();
}

pub fn run() {
    let example = fs::read_to_string("src/puzzles/year2022/day02/example.txt")
        .expect("Should have been able to read the file");

    let input = fs::read_to_string("src/puzzles/year2022/day02/input.txt")
        .expect("Should have been able to read the file");

    bench("Part 1 Example", || part1(&example), Some(15));
    bench("Part 1 Input", || part1(&input), None);

    bench("Part 2 Example", || part2(&example), Some(12));
    bench("Part 2 Input", || part2(&input), None);
}
