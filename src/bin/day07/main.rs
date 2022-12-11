use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/day07.txt").expect("unable to read file");

    let root_dir = parse_input(input);

    println!("Part 1 - {}", part_1(&root_dir));
    println!("Part 2 - {}", part_2(&root_dir));
}

fn part_1(root_dir: &Dir) -> i64 {
    let mut sizes = HashMap::new();
    root_dir.get_sizes(&mut sizes, vec!["".to_string()]);
    sizes.values().filter(|s| **s <= 100000).sum()
}

fn part_2(root_dir: &Dir) -> i64 {
    let mut sizes = HashMap::new();
    root_dir.get_sizes(&mut sizes, vec!["".to_string()]);

    let space_to_free = sizes.get("").unwrap() - 40000000;

    *sizes
        .values()
        .filter(|s| **s >= space_to_free)
        .min()
        .unwrap()
}

fn parse_input(input: String) -> Dir {
    let lines: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();

    let mut cwd: Vec<String> = Vec::new();
    let mut root_dir = Dir::new();

    let mut i = 0;
    loop {
        let cmd = match lines.get(i) {
            Some(l) => l,
            None => break,
        };

        if cmd.starts_with("$ cd /") {
            cwd = Vec::new();
            i += 1;
            continue;
        }

        if cmd.starts_with("$ cd ..") {
            cwd.pop();
            i += 1;
            continue;
        }

        if cmd.starts_with("$ cd") {
            let dir_name = cmd.chars().skip(5).collect();
            cwd.push(dir_name);
            i += 1;
            continue;
        }

        if cmd.starts_with("$ ls") {
            while lines.get(i + 1).is_some() && !lines[i + 1].starts_with("$") {
                let line = &lines[i + 1];
                if line.starts_with("dir ") {
                    let dir_name = line.chars().skip(4).collect();
                    add_dir(&mut root_dir, &cwd, dir_name);
                } else {
                    let mut j = 0;
                    loop {
                        if !line.chars().nth(j).unwrap().is_numeric() {
                            break;
                        }

                        j += 1;
                    }

                    let file_size: i64 = line.chars().take(j).collect::<String>().parse().unwrap();
                    add_file(&mut root_dir, &cwd, file_size);
                }

                i += 1;
            }

            i += 1;
        }
    }

    root_dir
}

fn add_dir(tree: &mut Dir, cwd: &Vec<String>, new_dir_name: String) {
    let mut dir = tree;
    for dir_name in cwd {
        dir = dir
            .directories
            .get_mut(dir_name)
            .expect("directory does not exist");
    }
    dir.directories.insert(new_dir_name, Dir::new());
}

fn add_file(tree: &mut Dir, cwd: &Vec<String>, file_size: i64) {
    let mut dir = tree;
    for dir_name in cwd {
        dir = dir
            .directories
            .get_mut(dir_name)
            .expect("directory does not exist");
    }
    dir.files.push(file_size);
}

struct Dir {
    files: Vec<i64>,
    directories: HashMap<String, Dir>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            directories: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn get_sizes(&self, mut dst: &mut HashMap<String, i64>, path: Vec<String>) -> i64 {
        let mut total = 0;
        for (sub_name, sub_dir) in &self.directories {
            let mut sub_path = path.clone();
            sub_path.push(sub_name.clone());

            let sub_size = sub_dir.get_sizes(&mut dst, sub_path);
            total += sub_size;
        }

        let size = total + self.files.iter().sum::<i64>();
        dst.insert(path.join("/"), size);
        size
    }
}
