use std::{fs, str::FromStr};

struct Elf<'a> {
    lines: Vec<&'a str>,
}

impl Elf<'_> {
    pub fn total(&self) -> usize {
        self.lines
            .iter()
            .map(|line| FromStr::from_str(line).unwrap_or(0)) // Account for blank lines
            .reduce(|acc: usize, line| acc + line)
            .unwrap()
    }
}

fn main() {
    let max: usize = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|line| line.split('\n').collect())
        .map(|lines| Elf { lines })
        .map(|elf| elf.total())
        .max()
        .unwrap();

    println!("{max}");
}
