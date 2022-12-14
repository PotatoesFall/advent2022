use std::{cmp::Ordering, fs, time::Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Int(i64),
    List(Vec<Value>),
}

impl Value {
    // What do you mean the input lines are already valid json????????? Homemade parser it is
    fn parse_str(input: &str) -> Value {
        let chars: Vec<char> = input.chars().collect();

        Self::parse_value(&chars).0
    }

    // this is not meant to be accessed directly, recursive parser that parse_str uses.
    // it returns the number of characters read from chars, so that recursing calls can skip them.
    // there is undubitably a better way but I don't know it!
    fn parse_value(chars: &[char]) -> (Value, usize) {
        match chars[0] {
            '[' => {
                // list - parse until closing `]`
                let mut list = Vec::new();
                let mut i = 1;
                loop {
                    if chars.len() == i {
                        panic!("missing `]` in input");
                    }

                    match chars[i] {
                        ']' => {
                            // end of list found
                            return (Self::List(list), i + 1);
                        }
                        ',' => {
                            // next value
                            i += 1;
                        }
                        _ => {
                            // a value is encountered, parse it and move the cursor
                            let (v, n) = Self::parse_value(&chars[i..chars.len()]);
                            list.push(v);
                            i += n;
                        }
                    }
                }
            }

            // number, use hacky ASCII for fast parsing
            n => {
                let mut v = n as i64 - '0' as i64;

                let mut j = 1;
                while chars[j].is_numeric() {
                    v *= 10;
                    v += chars[j] as i64 - '0' as i64;
                    j += 1;
                }

                (Self::Int(v), j)
            }
        }
    }

    // not meant to be used directly, used to implement Ord
    fn compare(left: &Value, right: &Value) -> Ordering {
        match left {
            Value::List(left) => match right {
                Value::List(right) => Self::compare_lists(left, right),
                Value::Int(_) => Self::compare_lists(left, &vec![right.clone()]),
            },
            Value::Int(left_int) => match right {
                Value::List(right) => Self::compare_lists(&vec![left.clone()], right),
                Value::Int(right) => {
                    let diff = left_int - right;
                    if diff < 0 {
                        return Ordering::Less;
                    }
                    if diff > 0 {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                }
            },
        }
    }

    // not meant to be used directly, used by compare
    fn compare_lists(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
        let mut i = 0;
        loop {
            if left.len() == i && right.len() == i {
                return Ordering::Equal;
            }
            if left.len() == i {
                return Ordering::Less;
            }
            if right.len() == i {
                return Ordering::Greater;
            }

            match Self::compare(&left[i], &right[i]) {
                Ordering::Equal => (),
                x => return x,
            }

            i += 1;
        }
    }
}

// implemented so that we can compare and sort without passing closures
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::compare(self, other)
    }
}

// required for Ord?
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // let input = fs::read_to_string("input/day13-test.txt").unwrap();
    let input = fs::read_to_string("input/day13.txt").unwrap();

    // time that stuff!
    let mut start_time = Instant::now();
    println!("Part 1 - {} ({:?})", part_1(&input), start_time.elapsed());
    start_time = Instant::now();
    println!("Part 2 - {} ({:?})", part_2(&input), start_time.elapsed());
}

fn part_1(input: &String) -> usize {
    // parse each pair of packets
    let pairs: Vec<(Value, Value)> = input
        .split("\n\n")
        .map(|pair_str| {
            let mut parsed = pair_str
                .split("\n")
                .map(|pair| (Value::parse_str(pair)))
                .into_iter();
            (parsed.next().unwrap(), parsed.next().unwrap())
        })
        .collect();

    // map to index + 1 ONLY if ordered correctly, otherwise 0, then sum
    // we implement Ord so we can just use <
    pairs
        .iter()
        .enumerate()
        .map(|(i, (left, right))| if left < right { i + 1 } else { 0 })
        .sum()
}

fn part_2(input: &String) -> usize {
    // reject empty lines during parsing
    let mut packets: Vec<Value> = input
        .split("\n")
        .filter(|line| line != &"")
        .map(|line| Value::parse_str(line))
        .collect();

    // add dividers
    let divider_1 = Value::parse_str("[[2]]");
    let divider_2 = Value::parse_str("[[6]]");
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    // We implement Ord so we can just sort :)
    packets.sort();

    // find indices of the dividers
    let mut i1 = 0;
    let mut i2 = 0;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == divider_1 {
            i1 = i + 1;
        }
        if *packet == divider_2 {
            i2 = i + 1;
        }
    }

    i1 * i2
}
