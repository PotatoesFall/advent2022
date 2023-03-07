use std::{collections::HashMap, fs};

use pathfinding::prelude::astar;

type ValvePos = u16;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    me: ValvePos,
    ele: ValvePos,
    min: i64,
    on: Vec<(ValvePos, i64)>,
}

#[derive(Debug)]
struct ValveInfo {
    flow_rate: i64,
    neighbors: Vec<(ValvePos, i64)>, // Valve and distance to valve
}

fn main() {
    let input = fs::read_to_string("input/day16-test.txt").unwrap();
    // let input = fs::read_to_string("input/day16.txt").unwrap();

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
                    .map(|v| (parse_valve(v.as_bytes(), 0)))
                    .collect(),
            },
        );
    });

    let total_flow_rate = valves.values().map(|v| v.flow_rate).sum();

    let start_pos = parse_valve(&['A' as u8, 'A' as u8]);

    let start = State {
        min: 4,
        on: Vec::new(),
        me: ActorState::Free(start_pos),
        ele: ActorState::Free(start_pos),
    };

    let mut highest_min = 0;
    let n_working_valves = valves.values().filter(|v| v.flow_rate > 0).count();
    let successors = |state: &State| {
        let mut neighbors = Vec::new();
        let cost = cost(&valves, &state.on, 1, total_flow_rate);

        if state.on.len() == n_working_valves {
            dbg!(1);
            neighbors.push((
                State {
                    ele_pos: state.ele_pos,
                    pos: state.pos,
                    min: 30,
                    on: state.on.clone(),
                },
                cost * (30 - state.min),
            ));
            return neighbors;
        }

        // me travel
        for neighbor in valves.get(&state.pos).unwrap().neighbors.iter() {
            neighbors.push((
                State {
                    min: state.min + 1,
                    pos: *neighbor,
                    on: state.on.clone(),
                    ele_pos: state.ele_pos,
                },
                cost,
            ));
        }

        // me turn on
        if !state.on.iter().any(|(v, _)| *v == state.pos)
            && valves.get(&state.pos).unwrap().flow_rate != 0
        {
            let mut on = state.on.clone();
            on.push((state.pos, state.min + 1));
            neighbors.push((
                State {
                    min: state.min + 1,
                    pos: state.pos,
                    ele_pos: state.ele_pos,
                    on,
                },
                cost,
            ));
        }

        let mut out = Vec::new();
        // elephant travel
        for neighbor in valves.get(&state.ele_pos).unwrap().neighbors.iter() {
            for (s, c) in neighbors.iter_mut() {
                out.push((
                    State {
                        min: s.min,
                        pos: s.pos,
                        ele_pos: *neighbor,
                        on: s.on.clone(),
                    },
                    *c,
                ))
            }
        }

        // elephant turn on
        if state.ele_pos != state.pos
            && !state.on.iter().any(|(v, _)| *v == state.ele_pos)
            && valves.get(&state.ele_pos).unwrap().flow_rate != 0
        {
            for (s, c) in neighbors.iter_mut() {
                let mut on = s.on.clone();
                on.push((s.ele_pos, s.min + 1));
                out.push((
                    State {
                        min: s.min,
                        pos: s.pos,
                        ele_pos: s.ele_pos,
                        on: on.clone(),
                    },
                    *c,
                ))
            }
        }

        if state.min > highest_min {
            dbg!(state.min);
            highest_min = state.min;
        }

        for s in out.iter_mut() {
            s.0.on.sort();
        }

        out
    };

    let heuristic = |state: &State| h(&valves, &state, total_flow_rate);

    let success = |state: &State| state.min == 30;

    let (path, cost) = astar(&start, successors, heuristic, success).unwrap();

    dbg!(path);

    dbg!(total_flow_rate * 26 - cost);
}

fn parse_valve(input: &[u8]) -> ValvePos {
    (input[0] as ValvePos) * 256 + input[1] as ValvePos
}

fn cost(
    valves: &HashMap<ValvePos, ValveInfo>,
    on: &Vec<(ValvePos, i64)>,
    minutes: i64,
    total_flow_rate: i64,
) -> i64 {
    let mut missed_flow = total_flow_rate;
    for (v, _) in on {
        missed_flow -= valves.get(v).unwrap().flow_rate;
    }

    missed_flow * minutes
}

fn h(valves: &HashMap<ValvePos, ValveInfo>, state: &State, total_flow_rate: i64) -> i64 {
    let mut closed_valves: Vec<ValvePos> = valves
        .keys()
        .filter(|k| valves.get(k).unwrap().flow_rate != 0 && !state.on.iter().any(|(v, _)| v == *k))
        .map(|k| k.clone())
        .collect();

    closed_valves.sort_by(|a, b| {
        let a_flow = valves.get(a).unwrap().flow_rate;
        let b_flow = valves.get(b).unwrap().flow_rate;

        b_flow.cmp(&a_flow) // DESC
    });

    let mut missed_flow = cost(valves, &state.on, 30 - state.min, total_flow_rate);

    let mut min = state.min + 1;
    for (i, v) in closed_valves.iter().enumerate() {
        missed_flow -= valves.get(&v).unwrap().flow_rate * (30 - min);

        // now we can turn off two per round!
        if i % 2 == 1 {
            min += 2;
        }
    }

    missed_flow
}
