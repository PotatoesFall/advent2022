use std::{fs, time::Instant};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: [String; 3],
    test_divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
}

fn main() {
    println!("Day 11");

    let input = fs::read_to_string("input/day11.txt").unwrap();
    let t = Instant::now();

    let monkeys = parse_monkeys(input);

    println!("Part 1 - {}", part_1(monkeys.clone()));
    println!("Part 2 - {}", part_2(monkeys.clone()));

    println!("Total execution time: {:?}", t.elapsed());
}

fn parse_monkeys(input: String) -> [Monkey; 8] {
    input
        .split("\n\n")
        .map(|block| parse_monkey(block))
        .collect::<Vec<Monkey>>()
        .try_into()
        .unwrap()
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.split("\n").collect();
    Monkey {
        items: lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect(),
        operation: lines[2]
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap(),
        test_divisor: lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap(),
        true_monkey: lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap(),
        false_monkey: lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap(),
    }
}

fn part_1(mut monkeys: [Monkey; 8]) -> i64 {
    let mut monkey_business = [0; 8];

    for _ in 0..20 {
        for i in 0..8 {
            monkey_business[i] += monkeys[i].items.len() as i64;

            for mut worry in monkeys[i].items.clone() {
                // INSPECTION
                worry = inspect(&monkeys[i].operation, worry);

                // RELIEF
                worry = worry / 3;

                // TEST & THROW
                test_and_throw(&mut monkeys, i, worry);
            }

            monkeys[i].items.clear();
        }
    }

    monkey_business.sort_unstable();
    monkey_business[6] * monkey_business[7]
}

fn part_2(mut monkeys: [Monkey; 8]) -> i64 {
    let mut monkey_business = [0; 8];
    let divisor_product = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test_divisor);

    for _ in 0..10000 {
        for i in 0..8 {
            monkey_business[i] += monkeys[i].items.len() as i64;

            for mut worry in monkeys[i].items.clone() {
                // INSPECTION
                worry = inspect(&monkeys[i].operation, worry);

                // NO RELIEF, PREVENT OVERFLOW
                worry = worry % divisor_product;

                // TEST & THROW
                test_and_throw(&mut monkeys, i, worry);
            }

            monkeys[i].items.clear();
        }
    }

    monkey_business.sort_unstable();
    monkey_business[6] * monkey_business[7]
}

// inspect returns the new worry level after inspection,
// not accounting for any post-inspection relief
fn inspect(operation: &[String; 3], worry: i64) -> i64 {
    let v1 = parse_operand(operation[0].as_str(), worry);
    let v2 = parse_operand(operation[2].as_str(), worry);

    match operation[1].as_str() {
        "*" => v1 * v2,
        "+" => v1 + v2,
        x => panic!("unexpected operator: {}", x),
    }
}

fn parse_operand(operand: &str, worry: i64) -> i64 {
    match operand {
        "old" => worry,
        v => v.parse().unwrap(),
    }
}

// test_and_throw throws an item from monkey i to the correct monkey
// it does not remove the item from monkey i's item list
fn test_and_throw(monkeys: &mut [Monkey; 8], i: usize, worry: i64) {
    if worry % monkeys[i].test_divisor == 0 {
        monkeys[monkeys[i].true_monkey].items.push(worry);
    } else {
        monkeys[monkeys[i].false_monkey].items.push(worry);
    }
}
