use itertools::Itertools;
use std::{fs, str::FromStr};

pub fn total(lines: Vec<&str>) -> usize {
    lines
        .iter()
        .map(|line| FromStr::from_str(line).unwrap_or(0)) // Account for blank lines
        .sum()
}

fn main() {
    let max_calories: usize = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|line| line.split('\n').collect())
        .map(total)
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("{max_calories}");
}
