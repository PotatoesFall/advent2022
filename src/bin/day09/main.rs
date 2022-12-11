use std::{collections::HashSet, fs};

fn main() {
    let input_str = fs::read_to_string("input/day09.txt").expect("failed to read file");
    
    // parse into a direction and number of fields moved
    let input = parse_input(&input_str);

    let mut knots = vec![(0, 0); 10];

    // track all positions of the second and last knot
    let mut visited_1 = HashSet::new();
    let mut visited_2 = HashSet::new();

    // also record starting position
    visited_1.insert(knots[1]);
    visited_2.insert(knots[9]);

    for (direction, distance) in input {
        for _ in 0..distance {
            knots[0].0 += direction.0;
            knots[0].1 += direction.1;

            // apply operation for each following knot
            for i in 1..knots.len() {
                let h = knots[i - 1];
                let t = &mut knots[i];

                move_tail(&h, t)
            }

            // record new positions
            visited_1.insert(knots[1]);
            visited_2.insert(knots[9]);
        }
    }


    println!("Part 1 - {}", visited_1.len());
    println!("Part 2 - {}", visited_2.len());
}

fn parse_input(input: &String) -> Vec<((i64, i64), i64)> {
    input
        .split("\n")
        .map(|line| {
            let mut ch = line.chars();
            let direction = match ch.next().unwrap() {
                'U' => (0, 1),
                'D' => (0, -1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("unexpected direction character"),
            };
            let distance = ch.skip(1).collect::<String>().parse::<i64>().unwrap();

            (direction, distance)
        })
        .collect()
}

fn move_tail(h: &(i64, i64), t: &mut (i64, i64)) {
    let displacement = (h.0 - t.0, h.1 - t.1);

    // don't move if distance <= 1
    if displacement.0.abs() <= 1 && displacement.1.abs() <= 1 {
        return;
    }

    // otherwise move in one or both directions if applicable, but only by 1
    t.0 += displacement.0.signum();
    t.1 += displacement.1.signum();
}
