use std::{cmp::Ordering};

use num_bigint::{BigUint, ToBigUint};

#[derive(Debug)]
struct Monkey {
    items: Vec<BigUint>,
    inspections: BigUint,
    operation: Operation,
    test: BigUint,
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
    Number(BigUint),
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
            inspections: 0.to_biguint().unwrap(),
            operation: op,
            test,
            target_true,
            target_false,
        });
    }

    let shared_mod: BigUint = monkeys.iter()
        .map(|m| m.test.clone())
        .product();

    let mut i = 0;
    while i < 10000 {
        play_round(&mut monkeys, shared_mod.clone());

        if i % 1000 == 0 || i == 20 {
            print_state(&monkeys);            
        }

        // print_state(&monkeys);

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

    let answer: BigUint = monkeys.iter()
        .take(2)
        .map(|m| m.inspections.clone())
        .product();

    dbg!(answer);
}

fn play_round(monkeys: &mut Vec<Monkey>, shared_mod: BigUint) {
    let mut i = 0_usize;

    while i < monkeys.len() {
        let mut targets = vec![];

        {
            let monkey = monkeys
                .get_mut(i)
                .unwrap();

            monkey.inspections += BigUint::from(monkey.items.len());

            for item in monkey.items.iter() {
                let new_worry = perform_op(item, &monkey.operation) % shared_mod.clone();
            
                let target = if new_worry.clone() % monkey.test.clone() == 0_i32.to_biguint().unwrap() {
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

fn perform_op(value: &BigUint, op: &Operation) -> BigUint {
    match op {
        Operation::Add(op_value) => value.clone() + get_op_value(value, op_value),
        Operation::Multiply(op_value) => value.clone() * get_op_value(value, op_value),
    }
}

fn get_op_value(value: &BigUint, op_value: &OperationValue) -> BigUint {
    match op_value {
        OperationValue::Old => value.clone(),
        OperationValue::Number(number) => number.clone()
    }
}