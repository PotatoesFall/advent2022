/*
For Day 12 I *initially* used the A* search algorithm. Quick summary of how that works:
    Under the hood it uses a priority queue to keep track of the points
    which have the least theoretical cost to reach the goal (and the path to those points).
    The theoretical cost is the cost of the cheapest known path to that point,
    plus the value of the "heuristic" which is an input.
    It always explores the neighbors of the first item in the priority queue.
    If the heuristic is admissible, we are guaranteed to find the cheapest (=shortest) path.
I used an existing version of the algorithm from the pathfinding crate.

For part 2, we simply take the end as the starting point.
    The API of the pathfinding crate allows us to specify a function rather than a specific end point,
    so we can just check if we have reached a point with an elevation of a (= 0).
    I used a heuristic that looks for the closest 0.

    After some research online I realized that the expensive heuristic slowed down the algorithm.
    This unneccessary extra cost becomes apparent when visually inspecting the input.
    Therefore I used Dijkstra which is basically A* without a heuristic.
    Then after reading more I realized that's stupid as well, and BFS is much better suited since the cost is constant.
    Thanks internet!
*/

use std::{fs, time::Instant};

use pathfinding::prelude::bfs;

const START: i64 = 'S' as i64 - 'a' as i64;
const END: i64 = 'E' as i64 - 'a' as i64;

fn main() {
    // let input = fs::read_to_string("input/day12-test.txt").unwrap();
    let input = fs::read_to_string("input/day12.txt").unwrap();
    let mut height_map: Vec<Vec<i64>> = input
        .split("\n")
        .map(|line| line.chars().map(|ch| ch as i64 - 'a' as i64).collect())
        .collect();

    // get start and end
    let (start, end) = clean_map(&mut height_map);

    // Part 1
    let mut start_time = Instant::now();
    println!(
        "Part 1 - {} ({:?})",
        part_1(&height_map, &start, &end),
        start_time.elapsed()
    );

    // Part 2
    start_time = Instant::now();
    println!(
        "Part 2 - {} ({:?})",
        part_2(&height_map, &end),
        start_time.elapsed()
    );
}

// clean_map gets the start and end and sets them to 0 and 25, respectively
fn clean_map(height_map: &mut Vec<Vec<i64>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..height_map.len() {
        for j in 0..height_map[0].len() {
            if height_map[i][j] == START {
                start = (i, j);
                height_map[i][j] = 0;
            }

            if height_map[i][j] == END {
                end = (i, j);
                height_map[i][j] = 25;
            }
        }
    }

    (start, end)
}

fn part_1(height_map: &Vec<Vec<i64>>, start: &(usize, usize), end: &(usize, usize)) -> i64 {
    let success = |p: &(usize, usize)| *p == *end;

    let neighbors = neighbor_fn(height_map, false);

    bfs(start, neighbors, success).unwrap().len() as i64
}

// neighbor_fn makes a closure that will find all possible neighbors of a point
// it can optionally be reversed for part 2
// it is also a totally unnecessary exploration of lifetime parameters which I have now understood better, yay!
fn neighbor_fn<'a>(
    height_map: &'a Vec<Vec<i64>>,
    reverse: bool,
) -> impl Fn(&(usize, usize)) -> Vec<(usize, usize)> + 'a {
    return move |p: &(usize, usize)| {
        let neighbors = neighbors_of(height_map, p);
        let mut possible_neighbors = Vec::new();

        for neighbor in neighbors {
            let steepness = height_map[neighbor.0][neighbor.1] - height_map[p.0][p.1];
            if reverse {
                if steepness >= -1 {
                    possible_neighbors.push(neighbor);
                }
            } else {
                if steepness <= 1 {
                    possible_neighbors.push(neighbor);
                }
            }
        }

        possible_neighbors
    };
}

// neighbors_of gets all neighbors of p that aren't out of the bounds
// it doesn't check the height difference
fn neighbors_of(height_map: &Vec<Vec<i64>>, p: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if p.0 != 0 {
        neighbors.push((p.0 - 1, p.1));
    }
    if p.0 != height_map.len() - 1 {
        neighbors.push((p.0 + 1, p.1));
    }
    if p.1 != 0 {
        neighbors.push((p.0, p.1 - 1));
    }
    if p.1 != height_map[0].len() - 1 {
        neighbors.push((p.0, p.1 + 1));
    }

    neighbors
}

fn part_2(height_map: &Vec<Vec<i64>>, end: &(usize, usize)) -> i64 {
    let success = |p: &(usize, usize)| height_map[p.0][p.1] == 0;

    bfs(end, neighbor_fn(height_map, true), success)
        .unwrap()
        .len() as i64
}
