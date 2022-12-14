use std::{env, fs};
use std::collections::{BTreeMap};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");

    let mut lines = file_content.lines();
    let mut stacks: BTreeMap<usize, Vec<char>> = BTreeMap::new();

    for line in lines.by_ref() {
        let mut chars = line.chars().enumerate();

        if line.contains('1') {
            break;
        }

        while let Some((i, char)) = chars.next() {
            if char == '[' {
                let (_, crate_label) = chars.next().unwrap();
                let stack_index = i / 4;

                stacks.entry(stack_index).or_default();

                let stack_vec = stacks.get_mut(&stack_index).unwrap();
                stack_vec.push(crate_label);
            }
        }
    }

    // Reverse the stacks as the lines are read top to bottom
    for (_, vec) in stacks.iter_mut() {
        vec.reverse();
    }

    lines.next();

    for line in lines {
        let mut parts = line.split(' ');

        // Calculate the indexes
        let nr_of_crates: usize = parts.nth(1).unwrap().parse().unwrap();
        let from_stack = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1_usize;
        let to_stack = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1_usize;

        // Mutable reference to the vec in the hashmap
        let source_stack = stacks.get_mut(&from_stack).unwrap();

        // .collect copies the elements of the iterator so a new mutable reference can be created
        // for the dest_stack.
        let target_source_length = source_stack.len() - nr_of_crates;
        let items: Vec<char> = source_stack.drain(target_source_length..).collect();

        // Items should be reversed as in the story they are moved one by one
        // items.reverse();

        // Mutable reference to the destination vec and add append the items
        let dest_stack = stacks.get_mut(&to_stack).unwrap();
        dest_stack.extend(items);
    }

    let mut top_crates = String::new();

    for (_, stack) in stacks.iter() {
        top_crates.push(*stack.last().unwrap());
    }

    println!("{}", top_crates);
}
