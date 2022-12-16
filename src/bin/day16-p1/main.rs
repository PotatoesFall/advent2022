use std::{collections::HashMap, fs};

use pathfinding::prelude::astar;

type Valve = u16;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    pos: Valve,
    minute: i64,
    on: Vec<(Valve, i64)>,
}

#[derive(Debug)]
struct ValveInfo {
    flow_rate: i64,
    neighbors: Vec<Valve>,
}

fn main() {
    // let input = fs::read_to_string("input/day16-test.txt").unwrap();
    let input = fs::read_to_string("input/day16.txt").unwrap();

    let mut valves = HashMap::new();
    input.split("\n").for_each(|line| {
        let mut spl = line.split(" has flow rate=");
        let b = spl.next().unwrap().as_bytes();
        spl = spl.next().unwrap().split("; tunnel");
        let rate_str = spl.next().unwrap();

        let mut neighbor_str = spl.next().unwrap();
        neighbor_str = neighbor_str
            .strip_prefix("s lead to valves ")
            .unwrap_or(neighbor_str);
        neighbor_str = neighbor_str
            .strip_prefix(" leads to valve ")
            .unwrap_or(neighbor_str);

        valves.insert(
            parse_valve(&b[6..8]),
            ValveInfo {
                flow_rate: rate_str.parse().unwrap(),
                neighbors: neighbor_str
                    .split(", ")
                    .map(|v| parse_valve(v.as_bytes()))
                    .collect(),
            },
        );
    });

    let total_flow_rate = valves.values().map(|v| v.flow_rate).sum();

    let start = State {
        minute: 0,
        on: Vec::new(),
        pos: parse_valve(&['A' as u8, 'A' as u8]),
    };

    let successors = |state: &State| {
        let mut neighbors = Vec::new();
        let cost = cost(&valves, &state.on, 1, total_flow_rate);
        for neighbor in valves.get(&state.pos).unwrap().neighbors.iter() {
            neighbors.push((
                State {
                    minute: state.minute + 1,
                    pos: *neighbor,
                    on: state.on.clone(),
                },
                cost,
            ));
        }

        if !state.on.iter().any(|(v, _)| *v == state.pos) {
            let mut on = state.on.clone();
            on.push((state.pos, state.minute + 1));
            neighbors.push((
                State {
                    minute: state.minute + 1,
                    pos: state.pos,
                    on,
                },
                cost,
            ));
        }

        neighbors
    };

    let heuristic = |state: &State| h(&valves, &state, total_flow_rate);

    let success = |state: &State| state.minute == 30;

    let (_, cost) = astar(&start, successors, heuristic, success).unwrap();

    dbg!(total_flow_rate * 30 - cost);
}

fn parse_valve(input: &[u8]) -> Valve {
    (input[0] as Valve) * 256 + input[1] as Valve
}

fn cost(
    valves: &HashMap<Valve, ValveInfo>,
    on: &Vec<(Valve, i64)>,
    minutes: i64,
    total_flow_rate: i64,
) -> i64 {
    let mut missed_flow = total_flow_rate;
    for (v, _) in on {
        missed_flow -= valves.get(v).unwrap().flow_rate;
    }

    missed_flow * minutes
}

fn h(valves: &HashMap<Valve, ValveInfo>, state: &State, total_flow_rate: i64) -> i64 {
    let mut closed_valves: Vec<Valve> = valves
        .keys()
        .filter(|k| valves.get(k).unwrap().flow_rate != 0 && !state.on.iter().any(|(v, _)| v == *k))
        .map(|k| k.clone())
        .collect();

    closed_valves.sort_by(|a, b| {
        let a_flow = valves.get(a).unwrap().flow_rate;
        let b_flow = valves.get(b).unwrap().flow_rate;

        b_flow.cmp(&a_flow) // DESC
    });

    // let mut missed_flow = total_flow_rate * (30 - state.minute);
    let mut missed_flow = cost(valves, &state.on, 30 - state.minute, total_flow_rate);

    let mut min = state.minute + 1;
    for v in closed_valves {
        missed_flow -= valves.get(&v).unwrap().flow_rate * (30 - min);

        min += 2;
    }

    missed_flow
}
