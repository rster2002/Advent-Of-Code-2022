use std::{env, fs};
use std::collections::VecDeque;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let lines = file_content.lines();

    let mut cpu_like = CPULike::new();
    cpu_like.push_instructions(lines);

    let mut sum: i32 = 0;
    while !cpu_like.is_done() {
        if (cpu_like.cycle + 20) % 40 == 0 {
            sum += cpu_like.cycle as i32 * cpu_like.register;
        }

        cpu_like.run_cycle();
    }

    println!("Done!");
    println!("Sum: {}", sum);
}

#[derive(Debug)]
struct CPULike {
    instructions: VecDeque<String>,
    cycle: u32,
    register: i32,
}

impl CPULike {
    pub fn new() -> Self {
        Self {
            instructions: Default::default(),
            cycle: 1,
            register: 1,
        }
    }

    pub fn is_done(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn push_instruction(&mut self, instruction: impl Into<String>) {
        self.instructions.push_back(instruction.into());
    }

    pub fn push_instructions(&mut self, instructions: impl Iterator<Item = impl Into<String>>) {
        for instruction in instructions {
            self.push_instruction(instruction);
        }
    }

    pub fn run_cycle(&mut self) {
        if let Some(instruction) = self.instructions.pop_front() {
            // println!("Running instruction: {}: {}", instruction, self.cycle);

            self.cycle += 1;
            self.run_instruction(instruction);
        }
    }

    fn run_instruction(&mut self, instruction: String) {
        let mut parts = instruction.split(' ');
        let name = parts.next().unwrap();

        match name {
            "addx" => {
                self.instructions.push_front(format!("xadd {}", parts.next().unwrap()));
            },

            "xadd" => {
                let value: i32 = parts.next().unwrap().parse().unwrap();
                self.register += value;
            },

            "noop" => {},

            _ => panic!("Invalid instruction name '{}'", name),
        }
    }
}
