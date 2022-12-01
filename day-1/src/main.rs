use std::{env, fs, mem};
use std::cmp::Ordering::{Greater, Less};
use std::collections::hash_map::Entry;
use std::fs::File;

fn main() {
    let file_path = env::args().nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut lines = file_content
        .lines();

    let mut elves: Vec<Elf> = vec!();
    let mut current_elf = Elf::default();

    while let Some(line) = lines.next() {
        if line == "" {
            let elf = mem::take(&mut current_elf);
            elves.push(elf);
        } else {
            let number_value = line.parse().expect("Could not parse line");
            current_elf.entries.push(number_value);
        }
    }

    elves.sort_by(|a, b| {
        if a.carried_calories() < b.carried_calories() {
            Greater
        } else {
            Less
        }
    });

    let sum = elves.iter()
        .take(3)
        .map(|v| v.carried_calories())
        .reduce(|a, v| a + v)
        .expect("Could not reduce input");

    println!("{:?}", sum);
}

#[derive(Debug)]
struct Elf {
    pub entries: Vec<i32>,
}

impl Default for Elf {
    fn default() -> Self {
        Elf {
            entries: vec!(),
        }
    }
}

impl Elf {
    pub fn carried_calories(&self) -> i32 {
        let mut sum = 0;

        for entry in &self.entries {
            sum += entry;
        }

        sum
    }
}
