use std::fs;

fn main() {
    let input = fs::read_to_string("input/day03.txt").expect("unable to read file");

    println!("Part 1 - {}", part_1(&input));
    println!("Part 2 - {}", part_2(&input));
}

fn part_1(input: &String) -> i64 {
    input
        .split("\n")
        .map(|line| {
            let rucksack: Vec<char> = line.chars().collect();
            let mut compartments = rucksack.chunks(rucksack.len() / 2);
            let first_compartment = compartments.next().unwrap();
            let second_compartment = compartments.next().unwrap();

            for item_type_first in first_compartment {
                for item_type_right in second_compartment {
                    if item_type_first == item_type_right {
                        return priority(*item_type_first);
                    }
                }
            }

            panic!("no match found between the two compartments");
        })
        .sum()
}

fn part_2(input: &String) -> i64 {
    input
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| {
            for item_type_1 in lines[0].chars() {
                for item_type_2 in lines[1].chars() {
                    if item_type_1 == item_type_2 {
                        for item_type_3 in lines[2].chars() {
                            if item_type_3 == item_type_1 {
                                return priority(item_type_1);
                            }
                        }
                    }
                }
            }

            panic!("no badge found between the three rucksacks");
        })
        .sum()
}

fn priority(item_type: char) -> i64 {
    if item_type >= 'a' && item_type <= 'z' {
        item_type as i64 - 'a' as i64 + 1
    } else if item_type >= 'A' && item_type <= 'Z' {
        item_type as i64 - 'A' as i64 + 27
    } else {
        panic!("invalid item type");
    }
}
