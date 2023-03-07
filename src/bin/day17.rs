use std::{collections::HashMap, fmt, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Empty,
    Falling,
    Stopped,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Material::Empty => '.',
            Material::Falling => '@',
            Material::Stopped => '#',
        };

        write!(f, "{}", ch)
    }
}

fn main() {
    // let input = fs::read_to_string("input/day17-test.txt").unwrap();
    let input = fs::read_to_string("input/day17.txt").unwrap();

    let part1 = run_game(2022, input.clone());
    println!("Part 1 - {}", part1);

    let part2 = run_game(1000000000000, input.clone());
    println!("Part 2 - {}", part2);
}

fn run_game(rounds: i64, input: String) -> i64 {
    let jets: Vec<char> = input.chars().collect();
    let mut jet_i = 0;
    let mut game: [Vec<Material>; 7] = Default::default();

    let mut seen = HashMap::new();

    for i in 0..rounds {
        let dif = differential(&game);
        if let Some((old_round, old_h)) = seen.get(&(jet_i, i % 5, dif)) {
            let h_cur = highest(&game, Material::Stopped) + 1;
            let period = i - old_round;
            let per_period = h_cur - old_h;
            let remaining_rounds = rounds - i;
            let additional = remaining_rounds / period * per_period;
            let missing_rounds = remaining_rounds % period;

            for (seen_round, seen_h) in seen.values() {
                if *seen_round == old_round + missing_rounds {
                    return h_cur + additional + (seen_h - old_h);
                }
            }
            panic!("should be unreachable");
        }
        seen.insert(
            (jet_i, i % 5, dif),
            (i, highest(&game, Material::Stopped) + 1),
        );

        draw_shape(&mut game, i as i64);

        loop {
            apply_jet(&mut game, jets[jet_i]);
            jet_i += 1;
            if jet_i == jets.len() {
                jet_i = 0;
            }
            if move_down(&mut game) {
                freeze(&mut game);
                break;
            }
        }
    }

    highest(&game, Material::Stopped) + 1
}

fn differential(game: &[Vec<Material>; 7]) -> [i64; 5] {
    let mut h_vals = Vec::new();

    for col in game {
        let mut i = col.len();
        while i > 0 {
            i -= 1;
            if col[i] == Material::Stopped {
                break;
            }
        }
        h_vals.push(i)
    }

    let min_v = h_vals.iter().min().unwrap().clone();
    for v in h_vals.iter_mut() {
        *v -= min_v;
    }

    [
        h_vals[0] as i64,
        h_vals[1] as i64,
        h_vals[2] as i64,
        h_vals[3] as i64,
        h_vals[4] as i64,
    ]
}

fn freeze(game: &mut [Vec<Material>; 7]) {
    let y_start = highest(game, Material::Falling) as usize + 1;
    let mut y = y_start;
    while y > 0 && y_start - y <= 100 {
        y = y - 1;
        for x in 0..7 {
            if let Some(m) = game[x].get(y) {
                if *m == Material::Falling {
                    game[x][y] = Material::Stopped;
                }
            }
        }
    }
}

fn move_down(game: &mut [Vec<Material>; 7]) -> bool {
    let y_start = highest(game, Material::Falling);
    for y in (y_start - 3)..(y_start + 1) {
        for x in 0..7 {
            if let Some(v) = game[x].get(y as usize) {
                if *v != Material::Falling {
                    continue;
                }

                let y_dest = y - 1;
                if y_dest == -1 || game[x][y_dest as usize] == Material::Stopped {
                    return true;
                }
            }
        }
    }

    for y in (y_start - 3)..(y_start + 1) {
        for x in 0..7 {
            if let Some(v) = game[x].get(y as usize) {
                if *v != Material::Falling {
                    continue;
                }

                draw_point(game, x as i64, y - 1, Material::Falling);
                draw_point(game, x as i64, y, Material::Empty);
            }
        }
    }

    false
}

fn apply_jet(game: &mut [Vec<Material>; 7], jet: char) {
    let increment: i64 = match jet {
        '<' => -1,
        '>' => 1,
        x => panic!("{}", x),
    };

    let mut cols: Vec<i64> = (0..7).collect();
    if increment == 1 {
        cols.reverse();
    }

    let y_start = highest(game, Material::Falling);
    for y in (y_start - 3)..(y_start + 1) {
        for x in &cols {
            let col = &game[*x as usize];
            if col.len() <= y as usize || col[y as usize] != Material::Falling {
                continue;
            }
            let dest_x = x + increment;
            if dest_x > 6 || dest_x < 0 {
                return;
            }
            if let Some(m) = game[dest_x as usize].get(y as usize) {
                if *m == Material::Stopped {
                    return;
                }
            }
        }
    }

    // TODO this can be optimized by collecting the coordinates in the loop above
    for y in (y_start - 3)..(y_start + 1) {
        for x in &cols {
            let col = &game[*x as usize];
            if col.len() <= y as usize || col[y as usize] != Material::Falling {
                continue;
            }
            let dest_x = x + increment;
            draw_point(game, dest_x, y, Material::Falling);
            draw_point(game, *x, y, Material::Empty);
        }
    }
}

fn draw_shape(game: &mut [Vec<Material>; 7], round: i64) {
    let coordinates = match round % 5 {
        0 => vec![(2, 0), (3, 0), (4, 0), (5, 0)],
        1 => vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
        2 => vec![(4, 2), (4, 1), (2, 0), (3, 0), (4, 0)],
        3 => vec![(2, 0), (2, 1), (2, 2), (2, 3)],
        4 => vec![(2, 0), (3, 0), (2, 1), (3, 1)],
        x => panic!("impossible mod: {}", x),
    };

    for point in coordinates {
        let x = point.0;
        let y = highest(game, Material::Stopped) + 4 + point.1;
        draw_point(game, x, y, Material::Falling);
    }
}

fn highest(game: &[Vec<Material>; 7], mat: Material) -> i64 {
    let mut highest: i64 = -1;
    for col in game {
        let mut i = col.len();
        while i > 0 && col.len() - i <= 100 {
            i = i - 1;
            if col[i] == mat {
                if i as i64 > highest {
                    highest = i as i64;
                };
                break;
            }
        }
    }

    highest
}

fn draw_point(game: &mut [Vec<Material>; 7], x: i64, y: i64, mat: Material) {
    let col = &mut game[x as usize];

    // pad with empty fields
    for _ in col.len()..((y + 1) as usize) {
        col.push(Material::Empty);
    }

    col[y as usize] = mat;
}
