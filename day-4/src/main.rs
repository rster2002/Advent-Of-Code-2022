use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");

    let lines = file_content.lines();

    let mut number_of_duplicates = 0;
    for line in lines {
        let mut assignments = line.split(",");
        let first_assignment = SectionAssignment::from_str(assignments.next().unwrap());
        let second_assignment = SectionAssignment::from_str(assignments.next().unwrap());

        if first_assignment.check_colliding(&second_assignment) {
            number_of_duplicates += 1;
        }
    }

    println!("Number of duplicates: {}", number_of_duplicates);
}

#[derive(Debug)]
struct SectionAssignment(u32, u32);

impl SectionAssignment {
    pub fn from_str(string: &str) -> SectionAssignment {
        let mut parts = string.split("-");
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();

        SectionAssignment(start, end)
    }

    pub fn fully_contained(&self, other: &Self) -> bool {
        (self.0 <= other.0 && self.1 >= other.1) ||
            (other.0 <= self.0 && other.1 >= self.1)
    }

    pub fn check_colliding(&self, other: &Self) -> bool {
        other.0 <= self.1 && self.0 <= other.1
    }
}
