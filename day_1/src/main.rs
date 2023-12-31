use itertools::Itertools;
use std::{fs, str::FromStr};

fn main() {
    let max_calories: usize = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|line| {
            line.split('\n')
                .map(|line| FromStr::from_str(line).unwrap_or(0))
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("{max_calories}");
}
