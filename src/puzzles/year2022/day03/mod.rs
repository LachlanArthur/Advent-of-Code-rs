use crate::bench::make_part_with_standard_tests;
use crate::puzzles::Part;
use itertools::Itertools;

fn find_shared_letter_in_vector_of_strings(strings: &Vec<&str>) -> char {
    let mut shared_letters = strings[0].chars().collect::<std::collections::HashSet<_>>();

    for string in strings.iter().skip(1) {
        let string_set = string.chars().collect::<std::collections::HashSet<_>>();

        shared_letters = shared_letters
            .intersection(&string_set)
            .map(|x| *x)
            .collect::<std::collections::HashSet<_>>();
    }

    let shared_letter = shared_letters
        .iter()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_string()
        .chars()
        .next()
        .unwrap();

    shared_letter
}

fn char_value(c: char) -> u64 {
    let charcode = c as u8;

    let value = match c {
        'a'..='z' => charcode - ('a' as u8) + 1,
        'A'..='Z' => charcode - ('A' as u8) + 1 + 26,
        c => panic!("Invalid character: {:?}", c),
    };

    value as u64
}

pub fn part1(input: &String) -> String {
    input
        .trim()
        .split("\n")
        .map(|s| {
            let compartments = s.split_at(s.len() / 2);
            char_value(find_shared_letter_in_vector_of_strings(&vec![
                compartments.0,
                compartments.1,
            ]))
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &String) -> String {
    input
        .trim()
        .split("\n")
        .chunks(3)
        .into_iter()
        .map(|x| char_value(find_shared_letter_in_vector_of_strings(&x.collect_vec())))
        .sum::<u64>()
        .to_string()
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("157")),
        make_part_with_standard_tests(year, day, 2, part2, Some("70")),
    ]
}
