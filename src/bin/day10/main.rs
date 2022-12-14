use std::{fs, time::Instant};

fn main() {
    // let input = fs::read_to_string("input/day10-test.txt").unwrap();
    let input = fs::read_to_string("input/day10.txt").unwrap();
    let time = Instant::now();

    let mut register_values = Vec::new();
    register_values.push(0); // we start at 1 so write any value at index 0
    let mut register = 1;

    for line in input.split("\n") {
        let cmd = line.chars().take(4).collect::<String>();

        // one cycle
        register_values.push(register);

        match cmd.as_str() {
            "noop" => {}
            "addx" => {
                // an extra cycle
                register_values.push(register);

                // add value to register
                register += line
                    .chars()
                    .skip(5)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
            }
            _ => panic!("unexpected input"),
        }
    }

    let answer_1: i64 = [20, 60, 100, 140, 180, 220]
        .map(|cycle| register_values[cycle as usize] * cycle)
        .iter()
        .sum();

    println!("Part 1 - {}", answer_1);

    println!("Part 2:");

    for row in 0..6 {
        for col in 0..40 {
            // if distance from center is 1 or less, draw the #
            if (register_values[40 * row + col + 1] - col as i64).abs() <= 1 {
                print!("█");
            } else {
                print!("·");
            }
        }
        println!();
    }

    println!("Total execution time: {:?}", time.elapsed());
}
