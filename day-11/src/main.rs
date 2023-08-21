use std::{cmp::Ordering};

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    inspections: u32,
    operation: Operation,
    test: u32,
    target_true: usize,
    target_false: usize,
}

#[derive(Debug)]
enum Operation {
    Add(OperationValue),
    Multiply(OperationValue),
}

#[derive(Debug)]
enum OperationValue {
    Old,
    Number(u32),
}

fn main() {
    let path = std::env::args().nth(1)
        .unwrap();

    let content = std::fs::read_to_string(path)
        .unwrap();

    let mut lines = content
        .lines();

    let mut monkeys = vec![];

    while let Some(_) = lines.next() {
        let starting_items = lines.next()
            .unwrap()
            .trim()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let op_line = lines.next()
            .unwrap()
            .trim()
            .replace("Operation: new = old ", "");

        let mut operation = op_line
            .split(' ');

        let op = operation.next().unwrap();

        let value = operation.next().unwrap();
        let value = match value {
            "old" => OperationValue::Old,
            _ => OperationValue::Number(value.parse().unwrap()),
        };

        let op = match op {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!("Invalid operation"),
        };

        let test = lines.next()
            .unwrap()
            .trim()
            .replace("Test: divisible by ", "")
            .parse()
            .unwrap();

        let target_true = lines.next()
            .unwrap()
            .trim()
            .replace("If true: throw to monkey ", "")
            .parse()
            .unwrap();

        let target_false = lines.next()
            .unwrap()
            .trim()
            .replace("If false: throw to monkey ", "")
            .parse()
            .unwrap();

        lines.next();

        monkeys.push(Monkey {
            items: starting_items,
            inspections: 0,
            operation: op,
            test,
            target_true,
            target_false,
        });
    }

    let mut i = 0;
    while i < 20 {
        play_round(&mut monkeys);
        print_state(&monkeys);

        i += 1;
    }

    monkeys
        .sort_by(|a, b| {
            if a.inspections < b.inspections {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

    let answer: u32 = monkeys.iter()
        .take(2)
        .map(|m| m.inspections)
        .product();

    dbg!(answer);
}

fn play_round(monkeys: &mut Vec<Monkey>) {
    let mut i = 0_usize;

    while i < monkeys.len() {
        let mut targets = vec![];

        {
            let monkey = monkeys
                .get_mut(i)
                .unwrap();

            monkey.inspections += monkey.items.len() as u32;

            for item in monkey.items.iter() {
                let new_worry = perform_op(*item, &monkey.operation) / 3;
            
                let target = if new_worry % monkey.test == 0 {
                    monkey.target_true
                } else {
                    monkey.target_false
                };

                targets.push((target, new_worry));
            }

            monkey.items.clear();
        }

        for (target, item) in targets {
            monkeys
                .get_mut(target)
                .unwrap()
                .items
                .push(item);
        }

        i += 1;
    }
}

fn print_state(monkeys: &[Monkey]) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", i, monkey.items);
    }
}

fn perform_op(value: u32, op: &Operation) -> u32 {
    match op {
        Operation::Add(op_value) => value + get_op_value(value, op_value),
        Operation::Multiply(op_value) => value * get_op_value(value, op_value),
    }
}

fn get_op_value(value: u32, op_value: &OperationValue) -> u32 {
    match op_value {
        OperationValue::Old => value,
        OperationValue::Number(number) => *number,
    }
}