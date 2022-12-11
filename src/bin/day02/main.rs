use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Hash)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(&self) -> i64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn parse(ch: char) -> Self {
        match ch {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            x => panic!("cannot parse char {}", x),
        }
    }

    fn winning_matchup(opponent: &Self) -> Self {
        match opponent {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn draw_matchup(opponent: &Self) -> Self {
        match opponent {
            Self::Rock => Self::Rock,
            Self::Paper => Self::Paper,
            Self::Scissors => Self::Scissors,
        }
    }

    fn losing_matchup(opponent: &Self) -> Self {
        match opponent {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

fn points(point_matrix: &HashMap<(Shape, Shape), i64>, matchup: &(Shape, Shape)) -> i64 {
    matchup.1.points() + point_matrix.get(matchup).expect("unknown matchup")
}

fn make_point_matrix() -> HashMap<(Shape, Shape), i64> {
    HashMap::from([
        ((Shape::Rock, Shape::Rock), 3),
        ((Shape::Rock, Shape::Paper), 6),
        ((Shape::Rock, Shape::Scissors), 0),
        ((Shape::Paper, Shape::Rock), 0),
        ((Shape::Paper, Shape::Paper), 3),
        ((Shape::Paper, Shape::Scissors), 6),
        ((Shape::Scissors, Shape::Rock), 6),
        ((Shape::Scissors, Shape::Paper), 0),
        ((Shape::Scissors, Shape::Scissors), 3),
    ])
}

fn main() {
    let input = fs::read_to_string("input/day02.txt").expect("unable to read file");

    let point_matrix = make_point_matrix();

    let mut matchups = parse_part_1(&input);
    let score1 = calc_all_points(&point_matrix, &matchups);

    println!("Part 1 - {}", score1);

    matchups = parse_part_2(&input);
    let score2 = calc_all_points(&point_matrix, &matchups);

    println!("Part 2 - {}", score2);

}

fn parse_part_1(input: &String) -> Vec<(Shape, Shape)> {
    input
        .split("\n")
        .map(|line| {
            let ch: Vec<char> = line.chars().collect();
            (Shape::parse(ch[0]), Shape::parse(ch[2]))
        })
        .collect()
}

fn parse_part_2(input: &String) -> Vec<(Shape, Shape)> {
    input
        .split("\n")
        .map(|line| {
            let ch: Vec<char> = line.chars().collect();
            let opponent = Shape::parse(ch[0]);
            let player = match ch[2] {
                'X' => Shape::losing_matchup(&opponent),
                'Y' => Shape::draw_matchup(&opponent),
                'Z' => Shape::winning_matchup(&opponent),
                x => panic!("cannot parse char {}", x),
            };
            (opponent, player)
        })
        .collect()
}

fn calc_all_points(
    point_matrix: &HashMap<(Shape, Shape), i64>,
    matchups: &Vec<(Shape, Shape)>,
) -> i64 {
    matchups
        .iter()
        .fold(0, |acc, matchup| acc + points(&point_matrix, matchup))
}
