extern crate core;

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

        let win_state = WinState::from_char(chars.next().unwrap());
        let me = win_state.resolve_for_opponent(&opponent);

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

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_char(char: char) -> Self {
        return match char {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
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

    pub fn get_winning(&self) -> Self {
        return match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    pub fn get_losing(&self) -> Self {
        return match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
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

        return if &other.get_winning() == self {
            Some(Greater)
        } else {
            Some(Less)
        }
    }
}

#[derive(Debug)]
enum WinState {
    Win,
    Draw,
    Lose,
}

impl WinState {
    pub fn from_char(char: char) -> Self {
        match char {
            'X' => WinState::Lose,
            'Y' => WinState::Draw,
            'Z' => WinState::Win,
            _ => panic!("Invalid char"),
        }
    }

    pub fn resolve_for_opponent(&self, opponent: &Shape) -> Shape {
        match self {
            WinState::Draw => opponent.clone(),
            WinState::Win => opponent.get_winning(),
            WinState::Lose => opponent.get_losing(),
        }
    }
}
