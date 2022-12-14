use std::{collections::HashSet, fs, time::Instant};

fn main() {
    // let input = fs::read_to_string("input/day14-test.txt").unwrap();
    let input = fs::read_to_string("input/day14.txt").unwrap();

    let total_time = Instant::now();

    // parse into a hash set of rocks
    let (rocks, lowest) = map_rocks(input);

    let t = Instant::now();
    println!("Part 1 - {} ({:?})", part_1(&rocks, lowest), t.elapsed());

    let t = Instant::now();
    println!("Part 2 - {} ({:?})", part_2(&rocks, lowest), t.elapsed());

    println!("Total execution time: {:?}", total_time.elapsed());
}

fn map_rocks(input: String) -> (HashSet<(i64, i64)>, i64) {
    // first split into lines and pairs of i64
    let rock_lines = input.split("\n").map(|line| {
        line.split(" -> ").map(|p| {
            // split by comma and parse values
            let mut values = p.split(",").map(|v| v.parse().unwrap());
            (values.next().unwrap(), values.next().unwrap())
        })
    });

    // this will hold our rocks
    let mut rocks: HashSet<(i64, i64)> = HashSet::with_capacity(800); // seems to be enough preallocated space to prevent reallocating
    let mut lowest = 0;

    for mut rock_line in rock_lines {
        // starting point
        let mut cursor: (i64, i64) = rock_line.next().unwrap();

        for p in rock_line {
            // one step towards p
            let inc = ((p.0 - cursor.0).signum(), (p.1 - cursor.1).signum());

            // track the lowest rock
            if p.1 > lowest {
                lowest = p.1;
            }

            // map all points until and including p
            loop {
                rocks.insert(cursor);

                if cursor == p {
                    break;
                }
                cursor.0 += inc.0;
                cursor.1 += inc.1;
            }
        }
    }

    (rocks, lowest)
}

fn part_1(rocks: &HashSet<(i64, i64)>, lowest: i64) -> i64 {
    // will contain both rocks and sand
    let mut blocked = rocks.clone();

    'outer: loop {
        // new sand
        let mut p = (500, 0);

        loop {
            // when we fall past the lowest rock, we are done
            if p.1 == lowest {
                break 'outer;
            }

            // move down if possible
            if !blocked.contains(&(p.0, p.1 + 1)) {
                p.1 += 1;
                continue;
            }
            if !blocked.contains(&(p.0 - 1, p.1 + 1)) {
                p = (p.0 - 1, p.1 + 1);
                continue;
            }
            if !blocked.contains(&(p.0 + 1, p.1 + 1)) {
                p = (p.0 + 1, p.1 + 1);
                continue;
            }

            // can't move - add to set
            blocked.insert(p);
            break;
        }
    }

    (blocked.len() - rocks.len()) as i64
}

fn part_2(rocks: &HashSet<(i64, i64)>, lowest: i64) -> i64 {
    let mut sand = HashSet::with_capacity((lowest * lowest / 2) as usize); // preallocate excess space
    sand.insert((500, 0));

    // we go pyramid, top down
    for y in 1..lowest + 2 {
        // pyramid extends y from 500 in both directions
        for x in 500 - y..500 + y + 1 {
            // no sand where rocks are
            if rocks.contains(&(x, y)) {
                continue;
            }

            // only sand when sand is above
            if sand.contains(&(x + 1, y - 1))
                || sand.contains(&(x, y - 1))
                || sand.contains(&(x - 1, y - 1))
            {
                sand.insert((x, y));
            }
        }
    }

    sand.len() as i64
}
