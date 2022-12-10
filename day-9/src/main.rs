extern crate core;

use std::{env, fs, mem};
use std::collections::BTreeSet;
use std::ops::Deref;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let instructions = file_content.lines();
    let mut rope = Rope::new();
    let mut visited = BTreeSet::new();

    const GRID_WIDTH: i32 = 26;
    const GRID_HEIGHT: i32 = 21;

    rope.print_grid(GRID_WIDTH, GRID_HEIGHT);

    for instruction in instructions {
        let mut parts = instruction.split(' ');
        let direction = parts.next().unwrap();
        let amount: i32 = parts.next().unwrap().parse().unwrap();

        println!();
        println!("== {} ==", instruction);

        for _ in 0..amount {
            rope.nudge_head(direction);
            visited.insert((rope.knots.last().unwrap().x, rope.knots.last().unwrap().y));


        }

        println!();
        rope.print_grid(GRID_WIDTH, GRID_HEIGHT);
    }

    println!("Rope: {:?}", rope);
    println!("Visited: {:?}", visited.len());
}

#[derive(Debug)]
struct Rope {
    knots: Vec<RopeKnot>,
    // head: RopeEnd,
    // tail: RopeEnd,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            knots: vec![
                RopeKnot::new('H'),
                RopeKnot::new('1'),
                RopeKnot::new('2'),
                RopeKnot::new('3'),
                RopeKnot::new('4'),
                RopeKnot::new('5'),
                RopeKnot::new('6'),
                RopeKnot::new('7'),
                RopeKnot::new('8'),
                RopeKnot::new('9'),
            ],
        }
    }

    pub fn nudge_head(&mut self, direction: &str) {
        self.knots.get_mut(0).unwrap().nudge_direction(direction);

        let indexes: Vec<usize> = self.knots.iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();

        let mut windows = indexes.windows(2);
        for window in windows {
            let mut window_iter = window.iter();
            let index_a = window_iter.next().unwrap();
            let index_b = window_iter.next().unwrap();

            // TODO this is just gross...
            let target_reference = self.knots.get_mut(*index_b).unwrap();
            let mut original_target = mem::take(target_reference);

            let a = self.knots.get(*index_a).unwrap().clone();
            Rope::update_following(a, &mut original_target);

            mem::swap(self.knots.get_mut(*index_b).unwrap(), &mut original_target);
        }


    }

    fn update_following(parent: &RopeKnot, following: &mut RopeKnot) {
        while Rope::get_distance(parent, following) > 1.5_f32 {
            let nudge_x = collapse_to_factor(parent.x - following.x);
            let nudge_y = collapse_to_factor(parent.y - following.y);

            following.x += nudge_x;
            following.y += nudge_y;
        }
    }

    fn get_distance(parent: &RopeKnot, following: &RopeKnot) -> f32 {
        let a = (parent.x - following.x).pow(2) as f32;
        let b = (parent.y - following.y).pow(2) as f32;

        (a + b).sqrt()
    }

    fn print_grid(&self, w: i32, h: i32) {
        for c in 0..h {
            let y = h - c - 1;

            for x in 0..w {
                let knot_option = self.knots
                    .iter()
                    .find(|knot| knot.x == x && knot.y == y);

                if let Some(knot) = knot_option {
                    print!("{}", knot.icon);
                } else {
                    print!(".");
                }

                // if self.head.x == x && self.head.y == y {
                //     print!("H");
                // } else if self.tail.x == x && self.tail.y == y {
                //     print!("T");
                // } else {
                //     print!(".")
                // }
            }

            println!();
        }
    }
}

#[derive(Debug, Default)]
struct RopeKnot {
    x: i32,
    y: i32,
    icon: char,
}

impl RopeKnot {
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
