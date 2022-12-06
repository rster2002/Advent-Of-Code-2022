use std::{env, fs};
use std::collections::{VecDeque, BTreeMap};
use bounded_vec_deque::BoundedVecDeque;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");

    // Still use .lines to get rid of any \n
    let mut input = file_content.lines().next().unwrap();
    let mut buffer: BoundedVecDeque<char> = BoundedVecDeque::new(4);

    for (i, char) in input.chars().enumerate() {
        buffer.push_back(char);

        if buffer.len() == 4 {
            if all_unique(&buffer) {
                println!("Char: {}", i + 1);
                break;
            }
        }
    }
}

fn all_unique(buffer: &BoundedVecDeque<char>) -> bool {
    let mut map = BTreeMap::new();

    for (index, char) in buffer.iter().enumerate() {
        if map.contains_key(char) {
            return false;
        }

        map.insert(char, index);
    }

    true
}