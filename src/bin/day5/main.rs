use std::{fs, str::Chars, vec};

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let moves = parse_moves();

    println!("Part 1 - {}", part_1(&moves));
    println!("Part 2 - {}", part_2(&moves));
}

fn parse_moves() -> Vec<Move> {
    let input = fs::read_to_string("input/day5.txt").expect("unable to read file");

    input
        .split("\n")
        .skip(10)
        .map(|line| {
            let mut mv = Move {
                amount: 0,
                from: 0,
                to: 0,
            };
            let mut ch = line.chars();
            skip(&mut ch, 5);
            mv.amount = parse_number(&mut ch);
            skip(&mut ch, 5);
            mv.from = parse_number(&mut ch) - 1;
            skip(&mut ch, 3);
            mv.to = parse_number(&mut ch) - 1;
            mv
        })
        .collect()
}

fn skip(ch: &mut Chars, n: i64) {
    for _ in 0..n {
        ch.next().expect("expected more characters");
    }
}

fn input_hardcoded() -> Vec<Vec<char>> {
    vec![
        vec!['V', 'R', 'H', 'B', 'G', 'D', 'W'],
        vec!['F', 'R', 'C', 'G', 'N', 'J'],
        vec!['J', 'N', 'D', 'H', 'F', 'S', 'L'],
        vec!['V', 'S', 'D', 'J'],
        vec!['V', 'N', 'W', 'Q', 'R', 'D', 'H', 'S'],
        vec!['M', 'C', 'H', 'G', 'P'],
        vec!['C', 'H', 'Z', 'L', 'G', 'B', 'J', 'F'],
        vec!['R', 'J', 'S'],
        vec!['M', 'V', 'N', 'B', 'R', 'S', 'G', 'L'],
    ]
    .into_iter()
    .map(|mut v| {
        v.reverse();
        v
    })
    .collect()
}

fn part_1(moves: &Vec<Move>) -> String {
    let mut state = input_hardcoded();

    moves.iter().for_each(|mv| {
        for _ in 0..mv.amount {
            let ch = state[mv.from]
                .pop()
                .expect("attempt to take from empty pile");
            state[mv.to].push(ch);
        }
    });

    state.iter().map(|pile| pile.last().unwrap()).collect()
}

fn part_2(moves: &Vec<Move>) -> String {
    let mut state = input_hardcoded();

    moves.iter().for_each(|mv| {
        for i in 0..mv.amount {
            let ch = state[mv.from][state[mv.from].len() - mv.amount + i];
            state[mv.to].push(ch);
        }
        let i = state[mv.from].len() - mv.amount;
        state[mv.from].truncate(i);
    });

    state.iter().map(|pile| pile.last().unwrap()).collect()
}

fn parse_number(stream: &mut Chars) -> usize {
    let mut chars = Vec::new();
    loop {
        match stream.next() {
            Some(c) => {
                if c.is_numeric() {
                    chars.push(c)
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    chars
        .iter()
        .collect::<String>()
        .parse()
        .expect("could not parse int")
}
