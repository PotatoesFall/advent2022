use std::fs;

fn main() {
    let input = fs::read_to_string("input/day6.txt").expect("unable to read file");

    println!("Part 1 - {}", solve(&input, 4));
    println!("Part 2 - {}", solve(&input, 14));
}

fn solve(input: &String, marker_len: usize) -> i64 {
    let mut history: Vec<char> = input.chars().take(marker_len).collect();

    for (i, ch) in input.chars().enumerate().skip(marker_len) {
        if unique(&history) {
            return i as i64;
        }

        for i in 0..marker_len - 1 {
            history[i] = history[i + 1];
        }
        history[marker_len - 1] = ch;
    }

    panic!("No unique set found");
}

fn unique(vals: &Vec<char>) -> bool {
    let l = vals.len();
    for i in 0..l - 1 {
        for j in i + 1..l {
            if vals[i] == vals[j] {
                return false;
            }
        }
    }

    true
}
