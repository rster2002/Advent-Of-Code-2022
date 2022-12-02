use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::thread::sleep;

fn main() {
    let file_path = env::args().nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut lines = file_content
        .lines();

    let mut total_score = 0;
    while let Some(line) = lines.next() {
        if line.len() < 3 {
            continue;
        }

        let mut chars = line.chars();

        let opponent = Shape::from_char(chars.next().unwrap());

        chars.next();

        let me = Shape::from_char(chars.next().unwrap());

        if me == opponent {
            total_score += me.get_score_for_shape() + 3;
            continue;
        }

        if me > opponent {
            total_score += me.get_score_for_shape() + 6;
        } else {
            total_score += me.get_score_for_shape();
        }
    }

    println!("Total scope: {}", total_score);
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_char(char: char) -> Self {
        return match char {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid char"),
        }
    }

    pub fn get_score_for_shape(&self) -> i32 {
        return match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.get_score_for_shape() == other.get_score_for_shape()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Equal);
        }

        let win = match other {
            Shape::Rock => self == &Shape::Paper,
            Shape::Paper => self == &Shape::Scissors,
            Shape::Scissors => self == &Shape::Rock,
        };

        return if win {
            Some(Greater)
        } else {
            Some(Less)
        }
    }
}
