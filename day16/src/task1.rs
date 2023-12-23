use std::collections::HashMap;
use std::{fs, collections::VecDeque};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();

    let mut rays: VecDeque<Ray> = VecDeque::new();

    let n = contents.len() as i32;
    let m = contents[0].len() as i32;

    rays.push_back(Ray{x: 0, y: -1, dir: Direction::Right});

    let mut visited: HashMap<(i32, i32), Energy> = HashMap::new();

    while rays.len() != 0 {
        let first = rays.pop_front().unwrap();

        let (new_x, new_y) = first.next_pos();

        if new_x < 0 || new_x >= n || new_y < 0 || new_y >= m {
            continue;
        }

        let cur_dir = &first.dir;

        let new_dir = change_direction(*cur_dir, contents[new_x as usize].chars().nth(new_y as usize).unwrap());

        if let Some(entry) = visited.get(&(new_x, new_y)) {
            if entry.index(first.dir) {
                continue;
            }

            for new_direction in new_dir {
                rays.push_back(Ray {x: new_x, y: new_y, dir: new_direction});
            }

            continue;
        }

        let mut cur_energy = Energy::new();
        cur_energy.set_energy(*cur_dir);

        visited.insert((new_x, new_y), cur_energy);

        for new_direction in new_dir {
            rays.push_back(Ray {x: new_x, y: new_y, dir: new_direction});
        }
    }

    //println!("{:#?}", visited);

    let mut ans = 0;
    for (_key, value) in visited {
        if value.index(Direction::Left) || value.index(Direction::Right)
            || value.index(Direction::Up) || value.index(Direction::Down) {
                ans += 1;
            }
    }

    println!("{ans}");
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

fn change_direction(current_dir: Direction, mirror: char) -> Vec<Direction> {
    match mirror {
        '.' => {
            return vec![current_dir];
        },
        '|' => {
            match current_dir {
                Direction::Left | Direction::Right => {
                    return vec![Direction::Up, Direction::Down];
                },
                Direction::Up | Direction::Down => {
                    return vec![current_dir];
                },
            };
        },
        '-' => {
            match current_dir {
                Direction::Left | Direction::Right => {
                    return vec![current_dir];
                },
                Direction::Up | Direction::Down => {
                    return vec![Direction::Left, Direction::Right];
                },
            };
        },
        '/' => {
            match current_dir {
                Direction::Left => {
                    return vec![Direction::Down];
                },
                Direction::Right => {
                    return vec![Direction::Up];
                },
                Direction::Up => {
                    return vec![Direction::Right];
                },
                Direction::Down => {
                    return vec![Direction::Left];
                }
            };
        },
        '\\' => {
            match current_dir {
                Direction::Left => {
                    return vec![Direction::Up];
                },
                Direction::Right => {
                    return vec![Direction::Down];
                },
                Direction::Up => {
                    return vec![Direction::Left];
                },
                Direction::Down => {
                    return vec![Direction::Right];
                }
            };
        },
        _ => {
            panic!("Unknown symbol");
        }
    };
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Ray {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Ray {
    fn next_pos(&self) -> (i32, i32) {
        match self.dir {
            Direction::Down => (self.x + 1, self.y),
            Direction::Left => (self.x, self.y - 1),
            Direction::Right => (self.x, self.y + 1),
            Direction::Up => (self.x - 1, self.y)
        }
    }
}

#[derive(Debug)]
struct Energy {
    is_ener: Vec<Vec<bool>>,
}

impl Energy {
    fn new() -> Energy {
        Energy {
            is_ener: vec![vec![false, false], vec![false, false]],
        }
    }

    fn index(&self, dir: Direction) -> bool {
        match dir {
            Direction::Down => self.is_ener[0][0],
            Direction::Left => self.is_ener[0][1],
            Direction::Right => self.is_ener[1][0],
            Direction::Up => self.is_ener[1][1],
        }
    }

    fn set_energy(&mut self, dir: Direction) {
        match dir {
            Direction::Down => self.is_ener[0][0] = true,
            Direction::Left => self.is_ener[0][1] = true,
            Direction::Right => self.is_ener[1][0] = true,
            Direction::Up => self.is_ener[1][1] = true,
        }
    }
}