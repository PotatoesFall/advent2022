use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01.txt").expect("unable to read file");

    let mut inventories = Vec::new();
    for block in input.split("\n\n") {
        let mut inventory = Vec::new();
        for line in block.split("\n") {
            if line == "" {
                continue;
            }

            inventory.push(line.parse::<i64>().unwrap());
        }

        inventories.push(inventory);
    }

    let mut sums: Vec<i64> = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();

    let max_sum = sums.iter().max().unwrap();

    println!("Part 1 - {}", max_sum);

    sums.sort();

    let sum_three: i64 = sums.iter().rev().take(3).sum();

    println!("Part 2 - {}", sum_three);
}
