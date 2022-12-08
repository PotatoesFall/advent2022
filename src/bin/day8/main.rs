use std::{collections::HashSet, fs};

fn main() {
    // let input = fs::read_to_string("input/day8-test.txt")
    let input = fs::read_to_string("input/day8.txt")
        .expect("unable to read file")
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("Part 1 - {}", part_1(&input));
    println!("Part 2 - {}", part_2(&input));
}

fn part_1(input: &Vec<Vec<char>>) -> i64 {
    let mut visible_tree_indices = HashSet::new();
    let mut previous;

    for row in 0..input.len() {
        // from left
        previous = '/';
        for col in 0..input[row].len() {
            if input[row][col] > previous {
                visible_tree_indices.insert((row, col));
                previous = input[row][col]
            }
            if input[row][col] == '9' {
                break;
            }
        }

        // from right
        previous = '/';
        for col in (0..input[row].len()).rev() {
            if input[row][col] > previous {
                visible_tree_indices.insert((row, col));
                previous = input[row][col]
            }
            if input[row][col] == '9' {
                break;
            }
        }
    }

    for col in 0..input[0].len() {
        // from top
        previous = '/';
        for row in 0..input.len() {
            if input[row][col] > previous {
                visible_tree_indices.insert((row, col));
                previous = input[row][col]
            }
            if input[row][col] == '9' {
                break;
            }
        }

        // from bottom
        previous = '/';
        for row in (0..input.len()).rev() {
            if input[row][col] > previous {
                visible_tree_indices.insert((row, col));
                previous = input[row][col]
            }
            if input[row][col] == '9' {
                break;
            }
        }
    }

    visible_tree_indices.len() as i64
}



fn part_2(input: &Vec<Vec<char>>) -> i64 {
    let mut scenic_scores = Vec::new();

    for row in 0..input.len() {
        for col in 0..input.len() {
            scenic_scores.push(scenic_score(&input, (row, col)));
        }
    }

    scenic_scores.into_iter().max().unwrap()
}

fn scenic_score(input: &Vec<Vec<char>>, location: (usize, usize)) -> i64 {
    // outside trees are zero
    if location.0 == 0
        || location.0 == input.len() - 1
        || location.1 == 0
        || location.1 == input[0].len() - 1
    {
        return 0;
    }

    let height = input[location.0][location.1];
    let mut vis_score = 1;

    // down
    for i in location.0 + 1..input.len() {
        if input[i][location.1] >= height || i == input.len() - 1 {
            vis_score *= i - location.0;
            break;
        }
    }

    // up
    for i in (0..location.0).rev() {
        if input[i][location.1] >= height || i == 0 {
            vis_score *= location.0 - i;
            break;
        }
    }

    // right
    for i in location.1 + 1..input[0].len() {
        if input[location.0][i] >= height || i == input[0].len() - 1 {
            vis_score *= i - location.1;
            break;
        }
    }

    // left
    for i in (0..location.1).rev() {
        if input[location.0][i] >= height || i == 0 {
            vis_score *= location.1 - i;
            break;
        }
    }

    vis_score as i64
}
