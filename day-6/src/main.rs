use std::{env, fs};
use std::collections::BTreeMap;
use bounded_vec_deque::BoundedVecDeque;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");
    const BUFFER_SIZE: usize = 14;

    // Still use .lines to get rid of any \n
    let input = file_content.lines().next().unwrap();
    let mut buffer: BoundedVecDeque<char> = BoundedVecDeque::new(BUFFER_SIZE);

    for (i, char) in input.chars().enumerate() {
        buffer.push_back(char);

        if buffer.len() == BUFFER_SIZE && all_unique(&buffer) {
            println!("Char: {}", i + 1);
            return;
        }
    }

    println!("No marker found");
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
