extern crate core;

use std::{env, fs};
use std::collections::BTreeSet;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let instructions = file_content.lines();
    let mut rope = Rope::new();
    let mut visited = BTreeSet::new();

    const GRID_WIDTH: i32 = 6;
    const GRID_HEIGHT: i32 = 6;

    rope.print_grid(GRID_WIDTH, GRID_HEIGHT);

    for instruction in instructions {
        let mut parts = instruction.split(' ');
        let direction = parts.next().unwrap();
        let amount: i32 = parts.next().unwrap().parse().unwrap();

        println!();
        println!("== {} ==", instruction);

        for _ in 0..amount {
            rope.nudge_head(direction);

            println!();
            rope.print_grid(GRID_WIDTH, GRID_HEIGHT);
            visited.insert((rope.tail.x, rope.tail.y));
        }
    }

    println!("Rope: {:?}", rope);
    println!("Visited: {:?}", visited.len());
}

#[derive(Debug)]
struct Rope {
    head: RopeEnd,
    tail: RopeEnd,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: RopeEnd::new('H'),
            tail: RopeEnd::new('T'),
        }
    }

    pub fn nudge_head(&mut self, direction: &str) {
        self.head.nudge_direction(direction);
        self.update_tail();
    }

    fn update_tail(&mut self) {
        while self.get_distance() > 1.5_f32 {
            let difference_x = collapse_to_factor(self.head.x - self.tail.x);
            let difference_y = collapse_to_factor(self.head.y - self.tail.y);

            self.tail.x += difference_x;
            self.tail.y += difference_y;
        }
    }

    fn get_distance(&self) -> f32 {
        let a = (self.head.x - self.tail.x).pow(2) as f32;
        let b = (self.head.y - self.tail.y).pow(2) as f32;

        (a + b).sqrt()
    }

    fn print_grid(&self, w: i32, h: i32) {
        for c in 0..h {
            let y = h - c - 1;

            for x in 0..w {
                if self.head.x == x && self.head.y == y {
                    print!("H");
                } else if self.tail.x == x && self.tail.y == y {
                    print!("T");
                } else {
                    print!(".")
                }
            }

            println!();
        }
    }
}

#[derive(Debug)]
struct RopeEnd {
    x: i32,
    y: i32,
    icon: char,
}

impl RopeEnd {
    pub fn new(icon: char) -> Self {
        Self {
            x: 0,
            y: 0,
            icon,
        }
    }

    pub fn nudge_direction(&mut self, direction: &str) {
        match direction {
            "U" => self.y += 1,
            "D" => self.y -= 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => panic!("'{}' is not a valid direction", direction)
        }
    }
}

fn collapse_to_factor(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n < 0 {
        -1
    } else {
        1
    }
}
