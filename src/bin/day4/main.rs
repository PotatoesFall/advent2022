use std::{fs, str::Chars};

fn main() {
    let input = fs::read_to_string("input/day4.txt").expect("unable to read file");

    println!("Part 1 - {}", part_1(&input));
    println!("Part 2 - {}", part_2(&input));
}

fn part_1(input: &String) -> i64 {
    let pairs = parse_lines(input);

    pairs
        .iter()
        .filter(|ranges| full_overlap(ranges.0, ranges.1))
        .count() as i64
}

fn part_2(input: &String) -> i64 {
    let pairs = parse_lines(input);

    pairs
        .iter()
        .filter(|ranges| any_overlap(ranges.0, ranges.1))
        .count() as i64
}

fn full_overlap(range_1: (i64, i64), range_2: (i64, i64)) -> bool {
    (range_1.0 <= range_2.0 && range_1.1 >= range_2.1)
        || (range_1.0 >= range_2.0 && range_1.1 <= range_2.1)
}

fn any_overlap(range_1: (i64, i64), range_2: (i64, i64)) -> bool {
    (range_1.0 <= range_2.0 && range_1.1 >= range_2.0)
        || (range_1.0 > range_2.0 && range_1.0 <= range_2.1)
}

fn parse_lines(input: &String) -> Vec<((i64, i64), (i64, i64))> {
    input
        .split("\n")
        .map(|line| {
            let mut chars = line.chars();
            (
                (parse_number(&mut chars), parse_number(&mut chars)),
                (parse_number(&mut chars), parse_number(&mut chars)),
            )
        })
        .collect()
}

fn parse_number(stream: &mut Chars) -> i64 {
    let mut chars = Vec::new();
    loop {
        match stream.next() {
            Some(c) => {
                if c.is_numeric() {
                    chars.push(c)
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    chars
        .iter()
        .collect::<String>()
        .parse::<i64>()
        .expect("could not parse int")
}
