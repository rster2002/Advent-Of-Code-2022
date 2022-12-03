use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let lines = file_content
        .lines();

    // let mut sacks = vec!();
    let mut total: u32 = 0;
    for line in lines {
        let sack = Rucksack::from_line(line);
        let common_item = sack.common_item()
            .unwrap();

        total += item_to_priority(&common_item)
            .unwrap();
    }

    println!("Total priority: {}", total);
}

#[derive(Debug)]
struct Rucksack {
    pub compartments: [String; 2],
}

impl Rucksack {
    pub fn new() -> Rucksack {
        Rucksack {
            compartments: [String::new(), String::new()],
        }
    }

    pub fn from_line(line: &str) -> Rucksack {
        let mut sack = Rucksack::new();
        let compartment_size = line.len() / 2;

        for char in line.chars() {
            if sack.compartments[0].len() < compartment_size {
                sack.compartments[0].push(char);
            } else {
                sack.compartments[1].push(char);
            }
        }

        sack
    }

    pub fn common_item(&self) -> Option<char> {
        self.compartments[0]
            .chars()
            .find(|&char| self.compartments[1].contains(char))
    }
}

fn item_to_priority(char: &char) -> Option<u32> {
    let priority_str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`";
    let index_option = priority_str.chars().position(|c| &c == char);

    let Some(index) = index_option else {
        return None;
    };

    Some(index as u32 + 1)
}
