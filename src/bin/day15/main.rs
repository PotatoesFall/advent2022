use std::{collections::HashSet, fs, time::Instant};

struct Position {
    x: i64,
    y: i64,
}

struct Sensor {
    pos: Position,
    nearest_beacon: Position,
    distance: i64,
}

fn main() {
    // let input = fs::read_to_string("input/day15-test.txt").unwrap();
    // let y_test = 10;
    // let max_distress = 20;
    let input = fs::read_to_string("input/day15.txt").unwrap();
    let y_test = 2_000_000;
    let max_distress = 4_000_000;

    let t = Instant::now();

    let sensors = parse_input(input);
    println!("Parsing took {:?}", t.elapsed());

    let mut time = Instant::now();
    println!(
        "Part 1 - {} ({:?})",
        part1(&sensors, y_test),
        time.elapsed()
    );

    time = Instant::now();
    println!(
        "Part 2 - {} ({:?})",
        part2(&sensors, max_distress),
        time.elapsed()
    );

    println!("Completed in {:?}", t.elapsed());
}

fn parse_input(input: String) -> Vec<Sensor> {
    input
        .split("\n")
        .map(|line| {
            // do some splitting and indexing to extract numbers only
            let (left, right) = split_two(line, ":");
            let (s1, s2) = split_two(&left[12..left.len()], ", y=");
            let (b1, b2) = split_two(&right[24..right.len()], ", y=");

            // parse it all
            let mut s = Sensor {
                pos: Position {
                    x: s1.parse().unwrap(),
                    y: s2.parse().unwrap(),
                },
                nearest_beacon: Position {
                    x: b1.parse().unwrap(),
                    y: b2.parse().unwrap(),
                },
                distance: 0,
            };

            // precalculate distance because we need it a lot later
            s.distance = manhattan(&s.pos, &s.nearest_beacon);

            s
        })
        .collect()
}

// helper for splitting into two strings
fn split_two<'a>(input: &'a str, sep: &str) -> (&'a str, &'a str) {
    let mut split = input.split(sep);
    (split.next().unwrap(), split.next().unwrap())
}

// calculate manhattan distance
fn manhattan(a: &Position, b: &Position) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn part1(sensors: &Vec<Sensor>, y_test: i64) -> i64 {
    let mut range = (i64::MAX, i64::MIN);
    let mut beacons = HashSet::with_capacity(10); // should be enough

    // we assume a continuous range based on what we know about part 2
    for sensor in sensors {
        // calculate how wide the range is at y
        let spread = sensor.distance - (sensor.pos.y - y_test).abs();
        if spread < 0 {
            continue;
        }

        // expand range
        range.0 = range.0.min(sensor.pos.x - spread);
        range.1 = range.1.max(sensor.pos.x + spread);

        // don't count beacons
        if sensor.nearest_beacon.y == y_test {
            beacons.insert(sensor.nearest_beacon.x);
        }
    }

    range.1 - range.0 + 1 - beacons.len() as i64
}

fn part2(sensors: &Vec<Sensor>, max_distress: i64) -> i64 {
    let mut p = Position { x: 0, y: 0 };

    // if there is only one spot, it is just outside the perimeter of another sensors range.
    // so we traverse the perimeters of all sensor ranges and check
    // there is probably a more elegant way but whatever, this works
    // reverse because we assume the creator put the correct ones at the end (works for my input, 10x faster)
    'outer: for i in (0..sensors.len()).rev() {
        for j in i + 1..sensors.len() {
            let s1 = &sensors[i];
            let s2 = &sensors[j];

            let d = manhattan(&s1.pos, &s2.pos);

            // not overlapping
            if d >= s1.distance + s2.distance {
                continue;
            }

            // no properly overlapping perimeter due to offset
            if (d - s1.distance - s2.distance) % 2 != 0 {
                continue;
            }

            // My initial approach was to follow the perimeters of all ranges,
            // and check every single point along it against all 28 ranges.
            // this took roughly 30ms.
            // the new approach is to check all combinations of two ranges,
            // find possible points where their perimeters touch,
            // and only check those points.

            // The below formulas were found through hours of puzzling.
            // Please do not ask me to explain them because I do not understand them.
            let x_term = s1.pos.x + s2.pos.x;
            let y_term = s1.pos.y + s2.pos.y;
            let x_diff = s1.pos.x - s2.pos.x;
            let y_diff = s1.pos.y - s2.pos.y;
            let d_term = s1.distance + s2.distance;
            let d_diff = s1.distance - s2.distance;

            let x_vals = [
                (x_term + y_diff + d_diff) / 2,
                (x_term + y_diff - d_diff) / 2,
                (x_term - y_diff + d_diff) / 2,
                (x_term - y_diff - d_diff) / 2,
                (x_term + y_diff + d_term) / 2,
                (x_term + y_diff - d_term) / 2,
                (x_term - y_diff + d_term) / 2,
                (x_term - y_diff - d_term) / 2,
            ];
            let y_vals = [
                (y_term + x_diff + d_diff) / 2,
                (y_term + x_diff - d_diff) / 2,
                (y_term - x_diff + d_diff) / 2,
                (y_term - x_diff - d_diff) / 2,
                (y_term + x_diff + d_term) / 2,
                (y_term + x_diff - d_term) / 2,
                (y_term - x_diff + d_term) / 2,
                (y_term - x_diff - d_term) / 2,
            ];

            for x in x_vals {
                for y in y_vals {
                    p = Position { x, y };
                    if p.x < s1.pos.x && p.x < s2.pos.x {
                        p.x -= 1;
                    } else if p.x < s1.pos.x && p.x < s2.pos.x {
                        p.x += 1;
                    } else if p.y < s1.pos.y && p.y < s2.pos.y {
                        p.y -= 1;
                    } else if p.y < s1.pos.y && p.y < s2.pos.y {
                        p.y += 1;
                    }

                    if manhattan(&p, &s1.pos) != s1.distance + 1
                        || manhattan(&p, &s2.pos) != s2.distance + 1
                    {
                        continue;
                    }

                    if check(sensors, &p, &max_distress) {
                        break 'outer;
                    }
                }
            }

            // The following comments are not useful, they are left for historical reasons, documenting my madness

            // APPRACHDFJSDLKFJDSKLFJ we do equations lol
            // p.x - s1.pos.x + p.y - s1.pos.y == s1.distance + 1 // perimeter 1
            // p.x - s1.pos.x - p.y + s1.pos.y == s1.distance + 1 // perimeter 1
            // -p.x + s1.pos.x + p.y - s1.pos.y == s1.distance + 1 // perimeter 1
            // -p.x + s1.pos.x - p.y + s1.pos.y == s1.distance + 1 // perimeter 1

            // p.x = s1.distance + 1 - p.y + s1.pos.x + s1.pos.y
            //   - 2 * p.y + s1.pos.y   + s1.pos.y ==  0
            // FUCK

            // manhattan(p,s2) == s2.distance + 1 // perimeter 2

            // THE over lap points will be:

            // now think about when to use which term. then u are good lol.
            // additional scenarios to consider are side overlaps instead of corner overlap

            // lets rephrase that. for corner overlap the formula changes per quadrant, but quadrants are divided diagonally
            // for side overlap, quadrants are divided laterally (fuk)

            // for x corner overlap where 1 is left of 2:
            // x = (x1 + x2)/2 +- (y1 - y2)/2 + (d1 - d2)/2
            // y = (y1 + y2)/2 +- ( (x1 - x2)/2 + (d1 + d2)/2 + 1)

            // for x corner overlap where 1 is right of 2:
            // x = (x1 + x2)/2 +- (y1 - y2)/2 - (d1 - d2)/2
            // y = (y1 + y2)/2 +- ( (x1 - x2)/2 - (d1 + d2)/2 + 1)

            // for y corner overlap where 1 is below 2:
            // x = (x1 + x2)/2 +- ( (y1 - y2)/2 + (d1 + d2)/2 + 1)
            // y = (y1 + y2)/2 +- (x1 - x2)/2 + (d1 - d2)/2

            // for y corner overlap where 1 is above 2:
            // x = (x1 + x2)/2 +- ( (y1 - y2)/2 - (d1 + d2)/2 + 1)
            // y = (y1 + y2)/2 +- (x1 - x2)/2 - (d1 - d2)/2

            // for  corner overlap
            // x = (x1 + x2)/2 +- ( (y1 - y2)/2 - (d1 + d2)/2 + 1)
            // y = (y1 + y2)/2 +- (x1 - x2)/2 - (d1 - d2)/2

            // make formulas for side overlap as well? I guess so lol

            // DELETE the stuff below here once we got that and only check the two points

            // // start from top corner
            // let mut corner = Position {
            //     x: s1.pos.x,
            //     y: s1.pos.y + s1.distance + 1,
            // };
            // let mut dist = s2.distance - manhattan(&corner, &s2.pos) + 1;
            // dbg!(dist);
            // if dist >= 0 {
            //     p = Position {
            //         x: corner.x - dist / 2,
            //         y: corner.y - dist / 2,
            //     };
            //     if check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }
            //     p.x = corner.x + dist / 2;
            //     if check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }
            // }

            // // start from bottom corner
            // corner = Position {
            //     x: s1.pos.x,
            //     y: s1.pos.y - s1.distance - 1,
            // };
            // dist = s2.distance - manhattan(&corner, &s2.pos) + 1;
            // if dist >= 0 {
            //     p = Position {
            //         x: corner.x - dist / 2,
            //         y: corner.y + dist / 2,
            //     };
            //     if check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }
            //     p.x = corner.x + dist / 2;
            //     if check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }
            // }

            // // from low x to middle, both high y and low y sides
            // for (dy, x) in (s1.pos.x - s1.distance - 1..s1.pos.x).enumerate() {
            //     // high y
            //     p = Position {
            //         x,
            //         y: s1.pos.y + dy as i64,
            //     };
            //     if manhattan(&p, &s2.pos) == s2.distance + 1 && check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }

            //     // low y
            //     p.y = s1.pos.y - dy as i64;
            //     if manhattan(&p, &s2.pos) == s2.distance + 1 && check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }
            // }

            // // from high x to middle, both high y and low y
            // for (dy, x) in (s1.pos.x..s1.pos.x + s1.distance + 2).rev().enumerate() {
            //     // high y
            //     p = Position {
            //         x,
            //         y: s1.pos.y + dy as i64,
            //     };
            //     if manhattan(&p, &s2.pos) == s2.distance + 1 && check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }

            //     // low y
            //     p.y = s1.pos.y - dy as i64;
            //     if manhattan(&p, &s2.pos) == s2.distance + 1 && check(sensors, &p, &max_distress) {
            //         break 'outer;
            //     }
            // }
        }
    }

    // calculate tuning frequency
    4_000_000 * p.x + p.y
}

// check if the beacon could exist at p
fn check(sensors: &Vec<Sensor>, p: &Position, max_distress: &i64) -> bool {
    if p.x < 0 || p.y < 0 || p.x > *max_distress || p.y > *max_distress {
        return false;
    }

    // check if any sensor range overlaps
    for s in sensors {
        if manhattan(&s.pos, &p) <= s.distance {
            return false;
        }
    }

    true
}
