use std::fs;
use std::collections::{HashMap, VecDeque};

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't read file");
    let contents: Vec<_> = binding.lines().collect();

    let max_size = 1000;

    let mut grid: Vec<Vec<bool>> = Vec::with_capacity(max_size);

    for i in 0..max_size {
        grid.push(Vec::new());
        for j in 0..max_size {
            grid[i].push(false);
        }
    }

    let mut cur_pos = (max_size as i32 / 2, max_size as i32 / 2);

    for line in contents {
        let parsed: Vec<&str> = line.split_whitespace().collect();

        let command = parsed[0].chars().nth(0).unwrap();
        let steps: i32 = parsed[1].parse().unwrap();
        let color = Color::from_string(parsed[2].to_string());

        let (dx, dy): (i32, i32) = match command {
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            _ => panic!("Incorrect symbol occured"),
        };

        for i in 0..steps {
            cur_pos = (cur_pos.0 + dx, cur_pos.1 + dy);

            grid[cur_pos.0 as usize][cur_pos.1 as usize] = true;
        }
    }

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    dfs(&mut visited, (0, 0), &grid);

    let mut exterior = 0;
    for (key, value) in visited {
        if value {
            exterior += 1;
        }
    }

    println!("{}", max_size * max_size - exterior);
}

fn dfs(visited: &mut HashMap<(i32, i32), bool>, pos: (i32, i32), grid: &Vec<Vec<bool>>) {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back(pos);

    visited.insert(pos, true);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx * dx + dy * dy != 1 {
                    continue;
                }

                let (new_x, new_y) = (pos.0 + dx, pos.1 + dy);

                if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
                    continue;
                }

                if let Some(tmp) = visited.get(&(new_x, new_y)) {
                    continue;
                }

                if grid[new_x as usize][new_y as usize] {
                    continue;
                }

                queue.push_back((new_x, new_y));
                visited.insert((new_x, new_y), true);
            }
        }
    }
}

struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

impl Color {
    fn from_string(line: String) -> Color {
        let red: u32 = line.chars().nth(2).unwrap().to_digit(16).unwrap() * 16 + line.chars().nth(3).unwrap().to_digit(16).unwrap();
        let green: u32 = line.chars().nth(4).unwrap().to_digit(16).unwrap() * 16 + line.chars().nth(5).unwrap().to_digit(16).unwrap();
        let blue: u32 = line.chars().nth(6).unwrap().to_digit(16).unwrap() * 16 + line.chars().nth(7).unwrap().to_digit(10).unwrap();

        Color{red, green, blue}
    }
}