use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{bench::make_part_with_standard_tests, puzzles::Part};
use gcollections::ops::*;
use interval::{ops::*, Interval};
use itertools::Itertools;

#[derive(Debug)]
struct Forest {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

type Tree = (usize, usize, usize);

impl Forest {
    fn row(&self, y: usize) -> Vec<&Tree> {
        self.range(Interval::new(0, self.width), Interval::new(y, y))
    }

    fn col(&self, x: usize) -> Vec<&Tree> {
        self.range(Interval::new(x, x), Interval::new(0, self.height))
    }

    fn rows(&self) -> Vec<Vec<&Tree>> {
        (0..self.height).map(|y| self.row(y)).collect_vec()
    }

    fn cols(&self) -> Vec<Vec<&Tree>> {
        (0..self.width).map(|x| self.col(x)).collect_vec()
    }

    fn range(&self, x: Interval<usize>, y: Interval<usize>) -> Vec<&Tree> {
        self.trees
            .iter()
            .filter(|(tree_x, tree_y, _)| x.contains(tree_x) && y.contains(tree_y))
            .collect_vec()
    }
}

#[derive(Debug)]
struct ForestParseError;

fn ascending_trees(trees: Vec<&Tree>) -> Vec<&Tree> {
    let mut ascending = Vec::<&Tree>::new();

    for tree in trees {
        let (_, _, height) = tree;

        if let Some((_, _, prev_height)) = ascending.last() {
            if height > prev_height {
                ascending.push(tree);
            }
        } else {
            ascending.push(tree);
        }
    }

    ascending
}

impl FromStr for Forest {
    type Err = ForestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees: Vec<Tree> = vec![];
        let mut width: usize = 0;
        let mut height: usize = 0;

        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                trees.push((
                    x as usize,
                    y as usize,
                    c.to_string().parse::<usize>().expect("Non-numeric tree"),
                ));
                width = x + 1;
            }
            height = y + 1;
        }

        Ok(Forest {
            trees,
            width,
            height,
        })
    }
}

pub fn part1(input: &String) -> String {
    let forest = input.parse::<Forest>().expect("Failed to parse forest");

    let rows = forest.rows();
    let cols = forest.cols();

    let north_visible = cols
        .iter()
        .map(|col| ascending_trees(col.to_vec()))
        .collect_vec();
    let south_visible = cols
        .iter()
        .map(|col| ascending_trees(col.iter().rev().copied().collect_vec()))
        .collect_vec();
    let west_visible = rows
        .iter()
        .map(|row| ascending_trees(row.to_vec()))
        .collect_vec();
    let east_visible = rows
        .iter()
        .rev()
        .map(|row| ascending_trees(row.iter().rev().copied().collect_vec()))
        .collect_vec();

    let all_visible_trees = [north_visible, south_visible, west_visible, east_visible]
        .concat()
        .iter()
        .flatten()
        .unique()
        .copied()
        .collect_vec();

    return all_visible_trees.len().to_string();
}

pub fn part2(input: &String) -> String {
    let forest = input.parse::<Forest>().expect("Failed to parse forest");

    forest
        .trees
        .iter()
        .map(|(x, y, h)| {
            // Calculate the number of trees that are visible from this tree

            let col = forest.col(*x);
            let row = forest.row(*y);

            // Trees in each direction, appropriately ordered
            let north = &col[0..*y].iter().rev().copied().collect_vec();
            let south = &col[(y + 1)..].to_vec();
            let west = &row[0..*x].iter().rev().copied().collect_vec();
            let east = &row[(x + 1)..].to_vec();

            let directions = [north, south, west, east];

            let trees = directions.iter().copied().map(|direction| {
                let mut visible = 0;

                for (_, _, other_h) in direction {
                    visible += 1;

                    if other_h >= h {
                        return visible;
                    }
                }

                visible
            });

            let score = trees.product1::<usize>().unwrap();

            score
        })
        .max()
        .expect("No trees in forest")
        .to_string()
}

pub fn parts(year: u16, day: u8) -> Vec<Part> {
    vec![
        make_part_with_standard_tests(year, day, 1, part1, Some("21")),
        make_part_with_standard_tests(year, day, 2, part2, Some("8")),
    ]
}
