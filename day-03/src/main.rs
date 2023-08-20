use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");

    let lines = file_content.lines();

    // let mut sacks = vec!();
    let mut total: u32 = 0;
    let mut current_group = Group::new();

    for line in lines {

        current_group.add_sack(line);

        if current_group.group_complete() {
            let common_item = current_group.common_item().unwrap();
            total += item_to_priority(&common_item).unwrap();

            current_group = Group::new();
        }
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

#[derive(Debug)]
struct Group<'a> {
    sack_contents: Vec<&'a str>,
}

impl<'a> Group<'a> {
    pub fn new() -> Self {
        Group {
            sack_contents: vec!(),
        }
    }

    pub fn add_sack(&mut self, sack: &'a str) {
        self.sack_contents.push(sack);
    }

    pub fn group_complete(&self) -> bool {
        self.sack_contents.len() == 3
    }

    pub fn common_item(&self) -> Option<char> {
        self.sack_contents[0].chars().find(|&char| {
            self.sack_contents[1].contains(char) && self.sack_contents[2].contains(char)
        })
    }
}
